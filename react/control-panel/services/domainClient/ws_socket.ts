import { parseURL, serializeURL } from "whatwg-url";
import WebSocket from "isomorphic-ws";
import { RtRequest } from "./types";
import {
  createMessageHandler,
  PeerConnectionHandler,
  ReceiveEvents,
  SendEvents,
  sendEventsHandler,
  SendRequest,
} from "./shared_socket";

// noinspection JSUnusedGlobalSymbols
export function createWebSocketClient(
  baseUrl: string,
  handler: ReceiveEvents,
  peerConnectionHandler?: PeerConnectionHandler
): [SendEvents, SendRequest] {
  const url = parseURL(baseUrl);
  if (!url) {
    throw new Error(`Could not parse base URL: "${url}"`);
  }

  url.scheme = url.scheme === "https" ? "wss" : "ws";
  url.path = ["ws"];

  let connected = false;

  const ws = new WebSocket(serializeURL(url, false));

  ws.onopen = () => {
    connected = true;
    handler.connectionChanged(connected);
  };
  ws.onclose = () => {
    connected = false;
    handler.connectionChanged(connected);
  };

  const send = (req: RtRequest) => {
    ws.send(JSON.stringify(req));
  };

  const messageHandler = createMessageHandler(handler, peerConnectionHandler);

  ws.onmessage = (message: unknown) => {
    messageHandler(JSON.parse((message as any).data)); // TO-DO: handle unknown
  };

  return [sendEventsHandler(() => ws.close(), send), send];
}
