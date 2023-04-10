import WebSocket from "isomorphic-ws";
import {parseURL, serializeURL} from "whatwg-url";

import {SetInstanceParameter} from "./instance";
import {clearIntervalAsync, setIntervalAsync} from "set-interval-async";
import {WsEvent} from "./ws";
import {nanoid} from "nanoid";
import {match} from "ts-pattern";

// noinspection JSUnusedGlobalSymbols
export function createWebSocketClient(
  baseUrl: string,
  handler: ReceiveEvents,
  config: { refreshInterval: number } = {refreshInterval: 30}
): SendEvents {
  const url = parseURL(baseUrl);
  if (!url) {
    throw new Error(`Could not parse base URL: "${url}"`);
  }

  url.scheme = url.scheme === "https" ? "wss" : "ws";
  url.path = "ws";

  let connected = false;

  const ws = new WebSocket(serializeURL(url, false));
  const toSend = new Map<[string, number], number>();

  ws.onopen = () => {
    connected = true;
    handler.connectionChanged(connected);
  };
  ws.onclose = () => {
    connected = false;
    handler.connectionChanged(connected);
  };

  ws.onmessage = (message: any) => {
    const parsed = WsEvent().parse(JSON.parse(message.data));
    match(parsed)
      .with({type: "instanceDriverEvent"}, ({instanceId, event}) => {
        match(event)
          .with({type: "report"}, ({reportId, channel, value}) => {
            handler.instanceReport(instanceId, reportId, channel, value);
          })
          .with({type: "connected"}, ({connected}) => {
            handler.instanceConnectionChanged(instanceId, connected);
          })
          .exhaustive();
      })
      .with({type: "setInstancePowerControl"}, ({requestId, success}) => {
        console.log('instance power request', requestId, success ? 'success' : 'failure')
      })
      .with({type: "setInstancePlayControl"}, ({requestId, success}) => {
        console.log('instance play request', requestId, success ? 'success' : 'failure')
      })
      .with({type: "setInstanceParameters"}, ({requestId, response}) => {
        console.log('instance play request', requestId, response)
      })
      .with({type: "subscribeToInstanceEvents"}, ({requestId, success}) => {
        console.log('instance subscribe to events request', requestId, success ? 'success' : 'failure')
      })
      .with({type: "unsubscribeFromInstanceEvents"}, ({requestId, success}) => {
        console.log('instance unsubscribe to events request', requestId, success ? 'success' : 'failure')
      })
      .exhaustive();
  };

  const timer = setIntervalAsync(async () => {
    if (connected) {
      const request = {
        requestId: nanoid(),
        command: {
          type: 'setInstanceParameters',
          changes: [] as Array<SetInstanceParameter>
        },
      }

      for (const [[parameter, channel], value] of toSend.entries()) {
        request.command.changes.push({parameter, channel, value})
      }

      if (request.command.changes.length > 0) {
        ws.send(JSON.stringify(request));
      }
    }
  }, config.refreshInterval);

  return {
    async close() {
      await clearIntervalAsync(timer);
      ws.close();
    },
    setParameter(name: string, channel: number, value: number) {
      toSend.set([name, channel], value);
    },
    subscribeToInstanceEvents(instanceId: string) {
      if (connected) {
        ws.send(JSON.stringify({
          requestId: nanoid(),
          command: {
            type: 'subscribeToInstanceEvents',
            instanceId
          }
        }));
      } else {
        throw new Error("Not connected")
      }
    },
    unsubscribeFromInstanceEvents(instanceId: string) {
      if (connected) {
        ws.send(JSON.stringify({
          requestId: nanoid(),
          command: {
            type: 'unsubscribeFromInstanceEvents',
            instanceId
          }
        }));
      } else {
        throw new Error("Not connected")
      }
    }
  };
}

export interface ReceiveEvents {
  connectionChanged(connected: boolean): any;

  instanceReport(instance: String, name: string, channel: number, value: number): void;

  instanceConnectionChanged(instanceId: String, connected: boolean): void;
}

export interface SendEvents {
  close(): void;

  setParameter(name: string, channel: number, value: number): void;

  subscribeToInstanceEvents(instanceId: string): void;

  unsubscribeFromInstanceEvents(instanceId: string): void;
}
