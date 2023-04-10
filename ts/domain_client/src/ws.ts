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

export const InstancePlayControl = memoizeOne(() =>
  z.object({
    desired: z.lazy(DesiredInstancePlayState),
    until: z.coerce.date(),
  })
);
export type InstancePlayControl = z.infer<
  ReturnType<typeof InstancePlayControl>
>;

export const InstancePowerControl = memoizeOne(() =>
  z.object({
    desired: z.lazy(DesiredInstancePowerState),
    until: z.coerce.date(),
  })
);
export type InstancePowerControl = z.infer<
  ReturnType<typeof InstancePowerControl>
>;

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
      type: z.literal("setInstanceParameters"),
    }),
    z.object({
      instance_id: z.string(),
      type: z.literal("subscribeToInstanceReports"),
    }),
    z.object({
      instance_id: z.string(),
      type: z.literal("unsubscribeFromInstanceReports"),
    }),
  ])
);
export type WsCommand = z.infer<ReturnType<typeof WsCommand>>;

export const WsEvent = memoizeOne(() =>
  z.discriminatedUnion("type", [
    z.object({ type: z.literal("setInstancePowerControlResponse") }),
    z.object({
      instance_id: z.string(),
      report: z.string(),
      type: z.literal("instanceReport"),
    }),
  ])
);
export type WsEvent = z.infer<ReturnType<typeof WsEvent>>;

export const WsRequest = memoizeOne(() =>
  z.object({ command: z.lazy(WsCommand), requestId: z.string() })
);
export type WsRequest = z.infer<ReturnType<typeof WsRequest>>;
