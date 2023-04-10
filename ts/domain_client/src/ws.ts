import memoizeOne from "memoize-one";
import { z } from "zod";
export const DesiredInstancePlayState = memoizeOne(() =>
  z.union([
    z.literal("stop"),
    z.object({
      play: z.object({ duration: z.number(), play_id: z.number().int() }),
    }),
  ])
);
export type DesiredInstancePlayState = z.infer<
  ReturnType<typeof DesiredInstancePlayState>
>;

export const DesiredInstancePowerState = memoizeOne(() =>
  z.enum(["off", "on"])
);
export type DesiredInstancePowerState = z.infer<
  ReturnType<typeof DesiredInstancePowerState>
>;

export const InstanceDriverEvent = memoizeOne(() =>
  z.discriminatedUnion("type", [
    z.object({ connected: z.boolean(), type: z.literal("connected") }),
    z.object({
      state: z.lazy(InstancePowerState),
      type: z.literal("powerStateChanged"),
    }),
    z.object({
      state: z.lazy(InstancePlayState),
      type: z.literal("playStateChanged"),
    }),
    z.object({
      capturedAt: z.coerce.date(),
      channel: z.number().int(),
      instanceId: z.string(),
      reportId: z.string(),
      type: z.literal("report"),
      value: z.number(),
    }),
  ])
);
export type InstanceDriverEvent = z.infer<
  ReturnType<typeof InstanceDriverEvent>
>;

export const InstancePlayControl = memoizeOne(() =>
  z.object({
    desired: z.lazy(DesiredInstancePlayState),
    until: z.coerce.date(),
  })
);
export type InstancePlayControl = z.infer<
  ReturnType<typeof InstancePlayControl>
>;

export const InstancePlayState = memoizeOne(() =>
  z.union([
    z.enum(["rewinding", "idle", "busy"]),
    z.object({
      playing: z.object({ duration: z.number(), play_id: z.number().int() }),
    }),
  ])
);
export type InstancePlayState = z.infer<ReturnType<typeof InstancePlayState>>;

export const InstancePowerControl = memoizeOne(() =>
  z.object({
    desired: z.lazy(DesiredInstancePowerState),
    until: z.coerce.date(),
  })
);
export type InstancePowerControl = z.infer<
  ReturnType<typeof InstancePowerControl>
>;

export const InstancePowerState = memoizeOne(() =>
  z.enum(["off", "coolingDown", "on", "warmingUp"])
);
export type InstancePowerState = z.infer<ReturnType<typeof InstancePowerState>>;

export const SetInstanceParameter = memoizeOne(() =>
  z.object({
    channel: z.number().int(),
    parameter: z.string(),
    value: z.number(),
  })
);
export type SetInstanceParameter = z.infer<
  ReturnType<typeof SetInstanceParameter>
>;

export const SetInstanceParameterResponse = memoizeOne(() =>
  z.enum([
    "success",
    "parameterNotFound",
    "channelNotFound",
    "notConnected",
    "rpcFailure",
  ])
);
export type SetInstanceParameterResponse = z.infer<
  ReturnType<typeof SetInstanceParameterResponse>
>;

export const WsCommand = memoizeOne(() =>
  z.discriminatedUnion("type", [
    z.object({
      instance_id: z.string(),
      power: z.lazy(InstancePowerControl),
      type: z.literal("setInstancePowerControl"),
    }),
    z.object({
      instance_id: z.string(),
      play: z.lazy(InstancePlayControl),
      type: z.literal("setInstancePlayControl"),
    }),
    z.object({
      changes: z.array(z.lazy(SetInstanceParameter)),
      instanceId: z.string(),
      type: z.literal("setInstanceParameters"),
    }),
    z.object({
      instance_id: z.string(),
      type: z.literal("subscribeToInstanceEvents"),
    }),
    z.object({
      instance_id: z.string(),
      type: z.literal("unsubscribeFromInstanceEvents"),
    }),
  ])
);
export type WsCommand = z.infer<ReturnType<typeof WsCommand>>;

export const WsEvent = memoizeOne(() =>
  z.discriminatedUnion("type", [
    z.object({
      requestId: z.string(),
      success: z.boolean(),
      type: z.literal("setInstancePowerControl"),
    }),
    z.object({
      requestId: z.string(),
      success: z.boolean(),
      type: z.literal("setInstancePlayControl"),
    }),
    z.object({
      requestId: z.string(),
      response: z.lazy(SetInstanceParameterResponse),
      type: z.literal("setInstanceParameters"),
    }),
    z.object({
      event: z.lazy(InstanceDriverEvent),
      instanceId: z.string(),
      type: z.literal("instanceDriverEvent"),
    }),
    z.object({
      requestId: z.string(),
      success: z.boolean(),
      type: z.literal("subscribeToInstanceEvents"),
    }),
    z.object({
      requestId: z.string(),
      success: z.boolean(),
      type: z.literal("unsubscribeFromInstanceEvents"),
    }),
  ])
);
export type WsEvent = z.infer<ReturnType<typeof WsEvent>>;

export const WsRequest = memoizeOne(() =>
  z.object({ command: z.lazy(WsCommand), requestId: z.string() })
);
export type WsRequest = z.infer<ReturnType<typeof WsRequest>>;
