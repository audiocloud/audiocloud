import { nanoid } from "nanoid";
import { pack, unpack } from "msgpackr";

import {
  createMessageHandler,
  ReceiveEvents,
  SendEvents,
  sendEventsHandler,
  SendRequest,
} from "./shared_socket";
import { createWebSocketClient } from "./ws_socket";
import { RtRequest } from "./rt";

export function createRtcSocket(
  baseUrl: string,
  handler: ReceiveEvents
): [SendEvents, SendRequest] {
  let pc: RTCPeerConnection = new RTCPeerConnection();
  let dc: RTCDataChannel;

  const [internal, upstreamSend] = createWebSocketClient(baseUrl, handler, {
    async offerPeerConnection(offer: string) {
      const remoteOffer: RTCSessionDescriptionInit = JSON.parse(offer);
      await pc.setRemoteDescription(remoteOffer);
      const answer = await pc.createAnswer();
      const answerJson = JSON.stringify(answer);
      upstreamSend({
        requestId: nanoid(),
        command: { type: "acceptPeerConnection", offer: answerJson },
      });
    },
    async offerIceCandidate(candidate: string) {
      await pc.addIceCandidate(JSON.parse(candidate));
    },
  });

  pc.onicecandidate = (event) => {
    let candidate = JSON.stringify(event.candidate);
    upstreamSend({
      requestId: nanoid(),
      command: {
        type: "offerPeerConnectionCandidate",
        candidate,
      },
    });
  };

  const messageHandler = createMessageHandler(handler);

  pc.ondatachannel = (event) => {
    dc = event.channel;
    internal.close();

    dc.onmessage = (event) => {
      messageHandler(unpack(event.data));
    };
  };

  pc.onconnectionstatechange = (event) => {
    handler.connectionChanged(pc.connectionState === "connected");
  };

  const send = (req: RtRequest) => dc.send(pack(req));

  return [sendEventsHandler(() => pc.close(), send), send];
}
