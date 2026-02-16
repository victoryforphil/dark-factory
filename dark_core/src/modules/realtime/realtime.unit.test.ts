import { describe, expect, it } from 'bun:test';

import {
  createRealtimeController,
  createRealtimeEventHub,
  REALTIME_INTERNAL_ORIGIN_HEADER,
  REALTIME_INTERNAL_ORIGIN_WS_RPC,
  REALTIME_ROUTE_MUTATION_EVENT,
  shouldBroadcastRouteMutation,
} from './realtime.controller';

describe('realtime module unit', () => {
  it('parses valid rpc request payloads', () => {
    const controller = createRealtimeController({
      dispatchHttpRequest: async () => {
        throw new Error('not used in this test');
      },
    });

    const parsed = controller.parseRpcRequestEnvelope({
      type: 'rpc_request',
      id: 'req_1',
      method: 'get',
      path: 'products/',
      query: {
        limit: 25,
      },
    });

    expect(parsed).toEqual({
      ok: true,
      value: {
        type: 'rpc_request',
        id: 'req_1',
        method: 'GET',
        path: '/products/',
        query: {
          limit: 25,
        },
      },
    });
  });

  it('returns protocol errors for unsupported methods', () => {
    const controller = createRealtimeController({
      dispatchHttpRequest: async () => {
        throw new Error('not used in this test');
      },
    });

    const parsed = controller.parseRpcRequestEnvelope({
      type: 'rpc_request',
      method: 'TRACE',
      path: '/products/',
    });

    expect(parsed.ok).toBe(false);
    if (!parsed.ok) {
      expect(parsed.error.error.code).toBe('REALTIME_PROTOCOL_ERROR');
      expect(parsed.error.error.message).toContain('Unsupported RPC method');
    }
  });

  it('dispatches rpc requests through the app adapter', async () => {
    let capturedRequest: Request | undefined;

    const controller = createRealtimeController({
      dispatchHttpRequest: async (request) => {
        capturedRequest = request;

        return new Response(JSON.stringify({ ok: true, data: { id: 'prd_1' } }), {
          status: 201,
          headers: {
            'content-type': 'application/json',
          },
        });
      },
    });

    const response = await controller.dispatchRpcRequest({
      type: 'rpc_request',
      id: 'req_2',
      method: 'POST',
      path: '/products/',
      query: {
        include: 'full',
      },
      body: {
        locator: '@local:///tmp/demo',
      },
    });

    expect(response).toEqual({
      type: 'rpc_response',
      id: 'req_2',
      status: 201,
      path: '/products/',
      body: {
        ok: true,
        data: {
          id: 'prd_1',
        },
      },
    });

    expect(capturedRequest).toBeDefined();
    expect(capturedRequest?.method).toBe('POST');
    expect(capturedRequest?.headers.get(REALTIME_INTERNAL_ORIGIN_HEADER)).toBe(
      REALTIME_INTERNAL_ORIGIN_WS_RPC,
    );

    const url = new URL(capturedRequest?.url ?? 'http://localhost/');
    expect(url.pathname).toBe('/products/');
    expect(url.searchParams.get('include')).toBe('full');

    const requestBody = capturedRequest ? await capturedRequest.clone().json() : null;
    expect(requestBody).toEqual({ locator: '@local:///tmp/demo' });
  });

  it('rejects rpc dispatch attempts to websocket paths', async () => {
    const controller = createRealtimeController({
      dispatchHttpRequest: async () => {
        throw new Error('dispatch should not run for /ws path');
      },
    });

    const response = await controller.dispatchRpcRequest({
      type: 'rpc_request',
      id: 'req_3',
      method: 'GET',
      path: '/ws',
    });

    expect(response).toEqual({
      type: 'rpc_response',
      id: 'req_3',
      status: 400,
      path: '/ws',
      body: {
        ok: false,
        error: {
          code: 'REALTIME_RPC_UNSUPPORTED_PATH',
          message: 'Realtime RPC cannot target /ws routes.',
        },
      },
    });
  });

  it('creates mutation events and notifies subscribers', () => {
    const realtimeHub = createRealtimeEventHub();
    let receivedEventName = '';

    const unsubscribe = realtimeHub.subscribe((event) => {
      receivedEventName = event.event;
    });

    realtimeHub.publishRouteMutation({
      method: 'PATCH',
      path: '/variants/v_1',
      status: 200,
      source: 'http',
    });

    unsubscribe();

    expect(receivedEventName).toBe(REALTIME_ROUTE_MUTATION_EVENT);
  });

  it('ignores non-mutation and realtime paths for broadcast checks', () => {
    expect(
      shouldBroadcastRouteMutation({
        method: 'GET',
        path: '/products/',
        status: 200,
      }),
    ).toBe(false);

    expect(
      shouldBroadcastRouteMutation({
        method: 'POST',
        path: '/ws',
        status: 200,
      }),
    ).toBe(false);

    expect(
      shouldBroadcastRouteMutation({
        method: 'PATCH',
        path: '/actors/act_1',
        status: 200,
      }),
    ).toBe(true);
  });
});
