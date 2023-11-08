import {
  InstancePlayControl,
  InstancePlayState,
  InstancePowerControl,
  InstancePowerState,
  InstanceSpec,
  RtEvent,
  RtRequest,
  SetInstanceParameter,
} from "./types";
import { nanoid } from "nanoid";
import { match } from "ts-pattern";

export type SendRequest = (req: RtRequest) => void;

export interface ReceiveEvents {
  connectionChanged(connected: boolean): any;

  instanceReport(
    device: string,
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
      device: string,
      changes: Array<SetInstanceParameter>
    ) {
      send({
        requestId: nanoid(),
        command: {
          type: "setInstanceParameters",
          instanceId: device,
          changes,
        },
      });
    },
    setInstancePowerControl(device: string, power: InstancePowerControl) {
      send({
        requestId: nanoid(),
        command: {
          type: "setInstancePowerControl",
          instanceId: device,
          power,
        },
      });
    },
    setInstancePlayControl(device: string, play: InstancePlayControl) {
      send({
        requestId: nanoid(),
        command: {
          type: "setInstancePlayControl",
          instanceId: device,
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
          "device power request",
          requestId,
          success ? "success" : "failure"
        );
      })
      .with({ type: "setInstancePlayControl" }, ({ requestId, success }) => {
        console.log(
          "device play request",
          requestId,
          success ? "success" : "failure"
        );
      })
      .with({ type: "setInstanceParameters" }, ({ requestId, response }) => {
        console.log("device play request", requestId, response);
      })
      .with({ type: "subscribeToInstanceEvents" }, ({ requestId, success }) => {
        console.log(
          "device subscribe to events request",
          requestId,
          success ? "success" : "failure"
        );
      })
      .with(
        { type: "unsubscribeFromInstanceEvents" },
        ({ requestId, success }) => {
          console.log(
            "device unsubscribe to events request",
            requestId,
            success ? "success" : "failure"
          );
        }
      )
      .with({ type: "setInstanceSpec" }, ({ instanceId, spec }) => {
        handler.instanceSpec(instanceId, spec);
      })
      .exhaustive();
  };
}
