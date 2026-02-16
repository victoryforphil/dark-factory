import { failure, toErrorMessage } from '../../utils/api-response';

export const REALTIME_WS_PATH = '/ws';
export const REALTIME_INTERNAL_ORIGIN_HEADER = 'x-dark-realtime-origin';
export const REALTIME_INTERNAL_ORIGIN_WS_RPC = 'ws-rpc';
export const REALTIME_ROUTE_MUTATION_EVENT = 'routes.mutated';

const RPC_REQUEST_BASE_URL = 'http://dark-core.internal';
const RPC_METHODS = new Set(['GET', 'POST', 'PATCH', 'DELETE']);
const RPC_MUTATION_METHODS = new Set(['POST', 'PATCH', 'DELETE']);

type RealtimeQueryValue = string | number | boolean | null;

type RealtimeEventSubscriber = (event: RealtimeEventEnvelope) => void;

export interface RealtimeRpcRequestEnvelope {
  type: 'rpc_request';
  id?: string;
  method: string;
  path: string;
  query?: Record<string, RealtimeQueryValue | undefined>;
  body?: unknown;
}

export interface RealtimeRpcResponseEnvelope {
  type: 'rpc_response';
  id: string;
  status: number;
  path: string;
  body: unknown;
}

export interface RealtimeProtocolErrorEnvelope {
  type: 'protocol_error';
  id?: string;
  error: {
    code: string;
    message: string;
  };
}

export interface RealtimeEventEnvelope<TPayload = Record<string, unknown>> {
  type: 'event';
  event: string;
  timestamp: string;
  payload: TPayload;
}

export interface RouteMutationPayload {
  method: string;
  path: string;
  status: number;
  source: 'http' | 'ws';
}

export interface RealtimeControllerDependencies {
  dispatchHttpRequest: (request: Request) => Promise<Response>;
}

export interface RealtimeController {
  parseRpcRequestEnvelope: (
    message: unknown,
  ) =>
    | {
        ok: true;
        value: RealtimeRpcRequestEnvelope;
      }
    | {
        ok: false;
        error: RealtimeProtocolErrorEnvelope;
      };
  dispatchRpcRequest: (message: RealtimeRpcRequestEnvelope) => Promise<RealtimeRpcResponseEnvelope>;
}

export interface RealtimeEventHub {
  subscribe: (subscriber: RealtimeEventSubscriber) => () => void;
  publish: (event: RealtimeEventEnvelope) => void;
  publishRouteMutation: (payload: RouteMutationPayload) => void;
  size: () => number;
}

const isRecord = (value: unknown): value is Record<string, unknown> => {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
};

const isQueryValue = (value: unknown): value is RealtimeQueryValue => {
  return (
    typeof value === 'string' ||
    typeof value === 'number' ||
    typeof value === 'boolean' ||
    value === null
  );
};

const parseMessagePayload = (value: unknown): unknown => {
  if (typeof value !== 'string') {
    return value;
  }

  const trimmed = value.trim();
  if (!trimmed) {
    return value;
  }

  try {
    return JSON.parse(trimmed) as unknown;
  } catch {
    return value;
  }
};

const protocolError = (message: string, id?: string): RealtimeProtocolErrorEnvelope => {
  return {
    type: 'protocol_error',
    ...(id ? { id } : {}),
    error: {
      code: 'REALTIME_PROTOCOL_ERROR',
      message,
    },
  };
};

const resolveRpcId = (id?: string): string => {
  const trimmed = id?.trim();

  if (trimmed) {
    return trimmed;
  }

  return crypto.randomUUID();
};

const normalizeRpcMethod = (method: string): string | null => {
  const normalized = method.trim().toUpperCase();

  if (!RPC_METHODS.has(normalized)) {
    return null;
  }

  return normalized;
};

export const normalizeRpcPath = (path: string): string => {
  const trimmed = path.trim();

  if (!trimmed) {
    return '/';
  }

  return trimmed.startsWith('/') ? trimmed : `/${trimmed}`;
};

const parseQueryRecord = (
  value: unknown,
): Record<string, RealtimeQueryValue | undefined> | null | undefined => {
  if (value === undefined) {
    return undefined;
  }

  if (value === null) {
    return null;
  }

  if (!isRecord(value)) {
    return null;
  }

  const parsed: Record<string, RealtimeQueryValue | undefined> = {};

  for (const [key, entryValue] of Object.entries(value)) {
    if (entryValue === undefined) {
      parsed[key] = undefined;
      continue;
    }

    if (!isQueryValue(entryValue)) {
      return null;
    }

    parsed[key] = entryValue;
  }

  return parsed;
};

export const parseRealtimeRpcRequestEnvelope = (
  message: unknown,
):
  | {
      ok: true;
      value: RealtimeRpcRequestEnvelope;
    }
  | {
      ok: false;
      error: RealtimeProtocolErrorEnvelope;
    } => {
  const payload = parseMessagePayload(message);

  if (!isRecord(payload)) {
    return {
      ok: false,
      error: protocolError('Realtime RPC payload must be an object.'),
    };
  }

  const id = typeof payload.id === 'string' ? payload.id : undefined;

  if (payload.type !== 'rpc_request') {
    return {
      ok: false,
      error: protocolError('Unsupported websocket message type.', id),
    };
  }

  if (typeof payload.method !== 'string') {
    return {
      ok: false,
      error: protocolError('RPC method must be a string.', id),
    };
  }

  const method = normalizeRpcMethod(payload.method);
  if (!method) {
    return {
      ok: false,
      error: protocolError(`Unsupported RPC method: ${payload.method}`, id),
    };
  }

  if (typeof payload.path !== 'string') {
    return {
      ok: false,
      error: protocolError('RPC path must be a string.', id),
    };
  }

  const query = parseQueryRecord(payload.query);
  if (query === null) {
    return {
      ok: false,
      error: protocolError('RPC query must be an object of primitive values.', id),
    };
  }

  return {
    ok: true,
    value: {
      type: 'rpc_request',
      ...(id ? { id } : {}),
      method,
      path: normalizeRpcPath(payload.path),
      ...(query ? { query } : {}),
      ...('body' in payload ? { body: payload.body } : {}),
    },
  };
};

const buildRpcUrl = (
  path: string,
  query?: Record<string, RealtimeQueryValue | undefined>,
): string => {
  const url = new URL(path, RPC_REQUEST_BASE_URL);

  if (query) {
    for (const [key, value] of Object.entries(query)) {
      if (value === undefined) {
        continue;
      }

      url.searchParams.set(key, String(value));
    }
  }

  return url.toString();
};

const shouldSendBody = (method: string, body: unknown): boolean => {
  if (method === 'GET') {
    return false;
  }

  return body !== undefined && body !== null;
};

const parseHttpResponseBody = async (response: Response): Promise<unknown> => {
  const responseText = await response.text();

  if (!responseText.trim()) {
    return null;
  }

  try {
    return JSON.parse(responseText) as unknown;
  } catch {
    return responseText;
  }
};

export const dispatchRealtimeRpcRequest = async (
  dependencies: RealtimeControllerDependencies,
  envelope: RealtimeRpcRequestEnvelope,
): Promise<RealtimeRpcResponseEnvelope> => {
  const requestId = resolveRpcId(envelope.id);
  const path = normalizeRpcPath(envelope.path);

  if (isRealtimePath(path)) {
    return {
      type: 'rpc_response',
      id: requestId,
      status: 400,
      path,
      body: failure('REALTIME_RPC_UNSUPPORTED_PATH', 'Realtime RPC cannot target /ws routes.'),
    };
  }

  try {
    const requestUrl = buildRpcUrl(path, envelope.query);
    const headers = new Headers();
    headers.set(REALTIME_INTERNAL_ORIGIN_HEADER, REALTIME_INTERNAL_ORIGIN_WS_RPC);

    let body: string | undefined;
    if (shouldSendBody(envelope.method, envelope.body)) {
      headers.set('content-type', 'application/json');
      body = JSON.stringify(envelope.body);
    }

    const response = await dependencies.dispatchHttpRequest(
      new Request(requestUrl, {
        method: envelope.method,
        headers,
        ...(body ? { body } : {}),
      }),
    );

    return {
      type: 'rpc_response',
      id: requestId,
      status: response.status,
      path,
      body: await parseHttpResponseBody(response),
    };
  } catch (error) {
    return {
      type: 'rpc_response',
      id: requestId,
      status: 500,
      path,
      body: failure('REALTIME_DISPATCH_FAILED', toErrorMessage(error)),
    };
  }
};

const isRealtimePath = (path: string): boolean => {
  return path === REALTIME_WS_PATH || path.startsWith(`${REALTIME_WS_PATH}/`);
};

export const shouldBroadcastRouteMutation = (input: {
  method: string;
  path: string;
  status: number;
}): boolean => {
  const method = normalizeRpcMethod(input.method);
  if (!method || !RPC_MUTATION_METHODS.has(method)) {
    return false;
  }

  if (!Number.isFinite(input.status) || input.status < 200 || input.status >= 300) {
    return false;
  }

  const path = normalizeRpcPath(input.path);
  if (isRealtimePath(path)) {
    return false;
  }

  return true;
};

export const createRouteMutationEvent = (
  payload: RouteMutationPayload,
): RealtimeEventEnvelope<RouteMutationPayload> => {
  return {
    type: 'event',
    event: REALTIME_ROUTE_MUTATION_EVENT,
    timestamp: new Date().toISOString(),
    payload,
  };
};

export const createRealtimeController = (
  dependencies: RealtimeControllerDependencies,
): RealtimeController => {
  return {
    parseRpcRequestEnvelope: parseRealtimeRpcRequestEnvelope,
    dispatchRpcRequest: (message) => dispatchRealtimeRpcRequest(dependencies, message),
  };
};

export const createRealtimeEventHub = (): RealtimeEventHub => {
  const subscribers = new Set<RealtimeEventSubscriber>();

  const publish = (event: RealtimeEventEnvelope): void => {
    for (const subscriber of subscribers) {
      try {
        subscriber(event);
      } catch {
        // Ignore subscriber-level failures and keep broadcasting.
      }
    }
  };

  return {
    subscribe(subscriber) {
      subscribers.add(subscriber);

      return () => {
        subscribers.delete(subscriber);
      };
    },
    publish,
    publishRouteMutation(payload) {
      publish(createRouteMutationEvent(payload));
    },
    size() {
      return subscribers.size;
    },
  };
};
