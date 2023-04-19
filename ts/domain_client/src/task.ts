import memoizeOne from "memoize-one";
import { z } from "zod";
export const AudioGraphSpec = memoizeOne(() =>
  z.object({
    busses: z.record(z.lazy(BusSpec)),
    inserts: z.record(z.lazy(InsertSpec)),
    sources: z.record(z.lazy(SourceSpec)),
  })
);
export type AudioGraphSpec = z.infer<ReturnType<typeof AudioGraphSpec>>;

export const BusSpec = memoizeOne(() =>
  z.object({
    inputs: z.array(z.array(z.lazy(InputSpec))),
    midSideMode: z.union([z.lazy(MidSideMode), z.null()]),
  })
);
export type BusSpec = z.infer<ReturnType<typeof BusSpec>>;

export const DesiredTaskPlayState = memoizeOne(() =>
  z.discriminatedUnion("type", [
    z.object({ type: z.literal("idle") }),
    z.object({
      from: z.number(),
      play_id: z.number().int(),
      to: z.number(),
      type: z.literal("play"),
    }),
  ])
);
export type DesiredTaskPlayState = z.infer<
  ReturnType<typeof DesiredTaskPlayState>
>;

export const InputSpec = memoizeOne(() =>
  z.object({ gain: z.number(), source: z.lazy(OutputId) })
);
export type InputSpec = z.infer<ReturnType<typeof InputSpec>>;

export const InsertSpec = memoizeOne(() =>
  z.object({
    inputs: z.array(z.array(z.lazy(InputSpec))),
    instanceId: z.string(),
  })
);
export type InsertSpec = z.infer<ReturnType<typeof InsertSpec>>;

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

export const MidSideMode = memoizeOne(() =>
  z.enum(["encodeToMidSide", "decodeToLeftRight"])
);
export type MidSideMode = z.infer<ReturnType<typeof MidSideMode>>;

export const OutputId = memoizeOne(() =>
  z.discriminatedUnion("type", [
    z.object({
      id: z.tuple([z.number().int(), z.number().int()]),
      type: z.literal("source"),
    }),
    z.object({
      id: z.tuple([z.number().int(), z.number().int()]),
      type: z.literal("insert"),
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
    sourceUrl: z.string(),
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
