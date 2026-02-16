import { Elysia } from 'elysia';

import {
  createRealtimeController,
  type RealtimeEventHub,
  type RealtimeProtocolErrorEnvelope,
  type RealtimeControllerDependencies,
  shouldBroadcastRouteMutation,
} from './realtime.controller';
import Log, { formatLogMetadata } from '../../utils/logging';

interface RealtimeSocketData {
  unsubscribe?: () => void;
}

export interface RealtimeRoutesDependencies extends RealtimeControllerDependencies {
  eventHub: RealtimeEventHub;
}

const sendProtocolError = (ws: { send: (payload: unknown) => unknown }, error: RealtimeProtocolErrorEnvelope) => {
  ws.send(error);
};

export const createRealtimeRoutes = (dependencies: RealtimeRoutesDependencies): Elysia => {
  const controller = createRealtimeController({
    dispatchHttpRequest: dependencies.dispatchHttpRequest,
  });

  return new Elysia().ws('/ws', {
    open(ws) {
      const wsData = ws.data as RealtimeSocketData;
      wsData.unsubscribe = dependencies.eventHub.subscribe((event) => {
        ws.send(event);
      });

      Log.info(
        `Core // Realtime Route // Client connected ${formatLogMetadata({
          subscribers: dependencies.eventHub.size(),
        })}`,
      );
    },
    close(ws) {
      const wsData = ws.data as RealtimeSocketData;
      wsData.unsubscribe?.();
      wsData.unsubscribe = undefined;

      Log.info(
        `Core // Realtime Route // Client disconnected ${formatLogMetadata({
          subscribers: dependencies.eventHub.size(),
        })}`,
      );
    },
    async message(ws, rawMessage) {
      const parsed = controller.parseRpcRequestEnvelope(rawMessage);

      if (!parsed.ok) {
        sendProtocolError(ws, parsed.error);
        Log.warn(
          `Core // Realtime Route // Protocol error ${formatLogMetadata({
            code: parsed.error.error.code,
            message: parsed.error.error.message,
          })}`,
        );
        return;
      }

      const request = parsed.value;
      const response = await controller.dispatchRpcRequest(request);
      ws.send(response);

      if (
        shouldBroadcastRouteMutation({
          method: request.method,
          path: request.path,
          status: response.status,
        })
      ) {
        dependencies.eventHub.publishRouteMutation({
          method: request.method,
          path: request.path,
          status: response.status,
          source: 'ws',
        });
      }
    },
  });
};
