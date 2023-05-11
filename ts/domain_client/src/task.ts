import memoizeOne from "memoize-one";
import { z } from "zod";
export const AudioGraphSpec = memoizeOne(() =>
  z.object({
    busses: z.record(z.lazy(BusSpec)),
    deviceInserts: z.record(z.lazy(DeviceInsertSpec)),
    sources: z.record(z.lazy(SourceSpec)),
    virtualInserts: z.record(z.lazy(VirtualInsertSpec)),
  })
);
export type AudioGraphSpec = z.infer<ReturnType<typeof AudioGraphSpec>>;

export const BusSpec = memoizeOne(() =>
  z.object({
    inputs: z.array(z.array(z.lazy(OutputId))),
    numOutputs: z.number().int(),
  })
);
export type BusSpec = z.infer<ReturnType<typeof BusSpec>>;

export const DesiredTaskPlayState = memoizeOne(() =>
  z.discriminatedUnion("type", [
    z.object({ type: z.literal("idle") }),
    z.object({
      end: z.number().int(),
      looping: z.boolean(),
      playId: z.number().int(),
      start: z.number().int(),
      startFrom: z.number().int(),
      type: z.literal("play"),
    }),
  ])
);
export type DesiredTaskPlayState = z.infer<
  ReturnType<typeof DesiredTaskPlayState>
>;

export const DeviceInsertSpec = memoizeOne(() =>
  z.object({
    inputs: z.array(z.array(z.lazy(OutputId))),
    instanceId: z.string(),
  })
);
export type DeviceInsertSpec = z.infer<ReturnType<typeof DeviceInsertSpec>>;

export const InstanceAllocationRequest = memoizeOne(() =>
  z.discriminatedUnion("type", [
    z.object({ instance_id: z.string(), type: z.literal("fixed") }),
    z.object({ model_id: z.string(), type: z.literal("dynamic") }),
  ])
);
export type InstanceAllocationRequest = z.infer<
  ReturnType<typeof InstanceAllocationRequest>
>;

export const MediaId = memoizeOne(() => z.string());
export type MediaId = z.infer<ReturnType<typeof MediaId>>;

export const OutputId = memoizeOne(() =>
  z.discriminatedUnion("type", [
    z.object({
      id: z.tuple([z.number().int(), z.number().int()]),
      type: z.literal("source"),
    }),
    z.object({
      id: z.tuple([z.number().int(), z.number().int()]),
      type: z.literal("deviceInsert"),
    }),
    z.object({
      id: z.tuple([z.number().int(), z.number().int()]),
      type: z.literal("virtualInsert"),
    }),
    z.object({
      id: z.tuple([z.number().int(), z.number().int()]),
      type: z.literal("bus"),
    }),
  ])
);
export type OutputId = z.infer<ReturnType<typeof OutputId>>;

export const SourceSpec = memoizeOne(() =>
  z.object({
    mediaId: z.lazy(MediaId),
    numChannels: z.number().int(),
    startAt: z.number().int(),
  })
);
export type SourceSpec = z.infer<ReturnType<typeof SourceSpec>>;

export const TaskSpec = memoizeOne(() =>
  z.object({
    app_id: z.string(),
    from: z.coerce.date(),
    graph_spec: z.lazy(AudioGraphSpec),
    host_id: z.string(),
    instances: z.record(z.string()),
    requests: z.record(z.lazy(InstanceAllocationRequest)),
    to: z.coerce.date(),
  })
);
export type TaskSpec = z.infer<ReturnType<typeof TaskSpec>>;

export const VirtualInsertSpec = memoizeOne(() =>
  z.object({ inputs: z.array(z.array(z.lazy(OutputId))), modelId: z.string() })
);
export type VirtualInsertSpec = z.infer<ReturnType<typeof VirtualInsertSpec>>;
