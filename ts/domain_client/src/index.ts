import { AxiosInstance } from "axios";
import WebSocket from "isomorphic-ws";
import { parseURL, serializeURL } from "whatwg-url";
import mitt, { Emitter } from "mitt";

import {
  InstanceDriverConfig,
  SetInstanceParameterRequest,
  WsDriverEvent,
  WsDriverRequest,
} from "./types";
import { clearIntervalAsync, setIntervalAsync } from "set-interval-async";

export * from "./types";

export namespace Driver {
  export async function getConfig(
    client: AxiosInstance
  ): Promise<InstanceDriverConfig> {
    const { data } = await client.get("/config");
    return InstanceDriverConfig.parse(data);
  }

  export async function setParameter(
    client: AxiosInstance,
    parameter: string,
    channel: number,
    value: number
  ): Promise<null> {
    const { data } = await client.put(
      `/parameter/${parameter}/${channel}`,
      value.toString(),
      { headers: { "content-type": "text/plain" } }
    );

    return data;
  }

  export function createWebSocketClient(
    baseUrl: string,
    handler: ReceiveEvents,
    config: { refreshInterval: number } = { refreshInterval: 30 }
  ): SendEvents {
    const url = parseURL(baseUrl);
    if (!url) {
      throw new Error(`Could not parse base URL: "${url}"`);
    }

    url.scheme = url.scheme === "https" ? "wss" : "ws";
    url.path = "/ws";

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
      const parsed = WsDriverEvent.parse(JSON.parse(message.data));
      switch (parsed.type) {
        case "report":
          handler.report(parsed.reportId, parsed.channel, parsed.value);
          break;
      }
    };

    const timer = setIntervalAsync(async () => {
      if (connected) {
        for (const [[parameter, channel], value] of toSend.entries()) {
          let request: WsDriverRequest = {
            type: "setParameter",
            parameter,
            channel,
            value,
          };

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
    };
  }

  export interface ReceiveEvents {
    connectionChanged(connected: boolean): any;

    report(name: string, channel: number, value: number): void;
  }

  export interface SendEvents {
    close(): void;

    setParameter(name: string, channel: number, value: number): void;
  }
}
