import {
  InstancePlayControl,
  InstancePlayState,
  InstancePowerControl,
  InstancePowerState,
  InstanceSpec,
  SetInstanceParameter,
} from "./instance";
import { RtEvent, RtRequest } from "./rt";
import { nanoid } from "nanoid";
import { match } from "ts-pattern";

export type SendRequest = (req: RtRequest) => void;

export interface ReceiveEvents {
  connectionChanged(connected: boolean): any;

  instanceReport(
    instance: string,
    name: string,
    channel: number,
    value: number
  ): void;

  instanceSpec(instanceId: string, spec: InstanceSpec | null): void;

  instanceConnectionChanged(instanceId: string, connected: boolean): void;

  instancePlayStateChanged(instanceId: string, state: InstancePlayState): void;

  instancePowerStateChanged(
    instanceId: string,
    state: InstancePowerState
  ): void;
}

export interface SendEvents {
  close(): void;

  setInstanceParameters(
    instanceId: string,
    changes: Array<SetInstanceParameter>
  ): void;

  setInstancePowerControl(
    instanceId: string,
    power: InstancePowerControl
  ): void;

  setInstancePlayControl(instanceId: string, play: InstancePlayControl): void;

  subscribeToInstanceEvents(instanceId: string): void;

  unsubscribeFromInstanceEvents(instanceId: string): void;
}

export interface PeerConnectionHandler {
  offerPeerConnection(offer: string): void;

  offerIceCandidate(candidate: string): void;
}

export function sendEventsHandler(
  close: () => void,
  send: (req: RtRequest) => void
): SendEvents {
  return {
    async close() {
      close();
    },
    setInstanceParameters(
      instance: string,
      changes: Array<SetInstanceParameter>
    ) {
      send({
        requestId: nanoid(),
        command: {
          type: "setInstanceParameters",
          instanceId: instance,
          changes,
        },
      });
    },
    setInstancePowerControl(instance: string, power: InstancePowerControl) {
      send({
        requestId: nanoid(),
        command: {
          type: "setInstancePowerControl",
          instanceId: instance,
          power,
        },
      });
    },
    setInstancePlayControl(instance: string, play: InstancePlayControl) {
      send({
        requestId: nanoid(),
        command: {
          type: "setInstancePlayControl",
          instanceId: instance,
          play: play,
        },
      });
    },
    subscribeToInstanceEvents(instanceId: string) {
      send({
        requestId: nanoid(),
        command: {
          type: "subscribeToInstanceEvents",
          instanceId,
        },
      });
    },
    unsubscribeFromInstanceEvents(instanceId: string) {
      send({
        requestId: nanoid(),
        command: {
          type: "unsubscribeFromInstanceEvents",
          instanceId,
        },
      });
    },
  };
}

export function createMessageHandler(
  handler: ReceiveEvents,
  peerConnectionHandler?: PeerConnectionHandler
) {
  return (data: any) => {
    const parsed = RtEvent().parse(data);
    match(parsed)
      .with({ type: "instanceDriverEvent" }, ({ instanceId, event }) => {
        match(event)
          .with({ type: "report" }, ({ reportId, channel, value }) => {
            handler.instanceReport(instanceId, reportId, channel, value);
          })
          .with({ type: "connected" }, ({ connected }) => {
            handler.instanceConnectionChanged(instanceId, connected);
          })
          .with({ type: "powerStateChanged" }, ({ state }) => {
            handler.instancePowerStateChanged(instanceId, state);
          })
          .with({ type: "playStateChanged" }, ({ state }) => {
            handler.instancePlayStateChanged(instanceId, state);
          })
          .exhaustive();
      })
      .with({ type: "setInstancePowerControl" }, ({ requestId, success }) => {
        console.log(
          "instance power request",
          requestId,
          success ? "success" : "failure"
        );
      })
      .with({ type: "setInstancePlayControl" }, ({ requestId, success }) => {
        console.log(
          "instance play request",
          requestId,
          success ? "success" : "failure"
        );
      })
      .with({ type: "setInstanceParameters" }, ({ requestId, response }) => {
        console.log("instance play request", requestId, response);
      })
      .with({ type: "subscribeToInstanceEvents" }, ({ requestId, success }) => {
        console.log(
          "instance subscribe to events request",
          requestId,
          success ? "success" : "failure"
        );
      })
      .with(
        { type: "unsubscribeFromInstanceEvents" },
        ({ requestId, success }) => {
          console.log(
            "instance unsubscribe to events request",
            requestId,
            success ? "success" : "failure"
          );
        }
      )
      .with({ type: "setInstanceSpec" }, ({ instanceId, spec }) => {
        handler.instanceSpec(instanceId, spec);
      })
      .with({ type: "offerPeerConnection" }, async ({ offer }) => {
        peerConnectionHandler?.offerPeerConnection(offer);
      })
      .with({ type: "offerPeerConnectionCandidate" }, ({ candidate }) => {
        peerConnectionHandler?.offerIceCandidate(candidate);
      })
      .exhaustive();
  };
}
