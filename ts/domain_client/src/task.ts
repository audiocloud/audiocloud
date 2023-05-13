import memoizeOne from "memoize-one";
import { z } from "zod";
export const AudioGraphSpec = memoizeOne(() => z.object({busses: z.record(z.lazy(BusSpec)), deviceInserts: z.record(z.lazy(DeviceInsertSpec)), sources: z.record(z.lazy(SourceSpec)), virtualInserts: z.record(z.lazy(VirtualInsertSpec)), }));
export type AudioGraphSpec = z.infer<ReturnType<typeof AudioGraphSpec>>;

export const BusSpec = memoizeOne(() => z.object({inputs: z.array(z.array(z.lazy(OutputId))), numOutputs: z.number().int(), }));
export type BusSpec = z.infer<ReturnType<typeof BusSpec>>;

export const CreateTaskRequest = memoizeOne(() => z.object({appId: z.string(), from: z.coerce.date(), instances: z.record(z.lazy(InstanceAllocationRequest)), taskId: z.union([z.string(), z.null(), ]), to: z.coerce.date(), }));
export type CreateTaskRequest = z.infer<ReturnType<typeof CreateTaskRequest>>;

export const CreateTaskResponse = memoizeOne(() => z.discriminatedUnion('type', [z.object({app_id: z.string(), task_id: z.string(), type: z.literal("success"), }), z.object({type: z.literal("overlappingTask"), }), z.object({instance_id: z.string(), type: z.literal("noSuchInstance"), }), ]));
export type CreateTaskResponse = z.infer<ReturnType<typeof CreateTaskResponse>>;

export const DesiredTaskPlayState = memoizeOne(() => z.discriminatedUnion('type', [z.object({type: z.literal("idle"), }), z.object({end: z.number().int(), looping: z.boolean(), playId: z.number().int(), start: z.number().int(), startFrom: z.number().int(), type: z.literal("play"), }), ]));
export type DesiredTaskPlayState = z.infer<ReturnType<typeof DesiredTaskPlayState>>;

export const DeviceInsertSpec = memoizeOne(() => z.object({inputs: z.array(z.array(z.lazy(OutputId))), instanceId: z.string(), }));
export type DeviceInsertSpec = z.infer<ReturnType<typeof DeviceInsertSpec>>;

export const GraphPlaybackError = memoizeOne(() => z.discriminatedUnion('type', [z.object({error: z.string(), sink: z.number().int(), type: z.literal("incompatibleSinks"), }), z.object({play_id: z.number().int(), type: z.literal("graphAlreadyPlaying"), }), ]));
export type GraphPlaybackError = z.infer<ReturnType<typeof GraphPlaybackError>>;

export const GraphPlaybackState = memoizeOne(() => z.discriminatedUnion('type', [z.object({position: z.number().int(), type: z.literal("buffering"), }), z.object({position: z.number().int(), type: z.literal("playing"), }), z.object({type: z.literal("stopped"), }), ]));
export type GraphPlaybackState = z.infer<ReturnType<typeof GraphPlaybackState>>;

export const GraphPlayerEvent = memoizeOne(() => z.discriminatedUnion('type', [z.object({details: z.lazy(GraphPlaybackError), type: z.literal("error"), }), z.object({details: z.object({state: z.lazy(GraphPlaybackState), }), type: z.literal("graphStateChanged"), }), z.object({details: z.object({nodes: z.record(z.lazy(NodeInfo)), play_id: z.number().int(), }), type: z.literal("graphNodesPrepared"), }), z.object({details: z.object({data: z.array(z.number().int()), play_head: z.lazy(PlayHead), play_id: z.number().int(), sink_id: z.number().int(), }), type: z.literal("graphSinkCaptured"), }), z.object({details: z.object({events: z.array(z.lazy(NodeEvent)), node_id: z.lazy(NodeId), play_id: z.number().int(), }), type: z.literal("nodeEvents"), }), ]));
export type GraphPlayerEvent = z.infer<ReturnType<typeof GraphPlayerEvent>>;

export const InstanceAllocationRequest = memoizeOne(() => z.discriminatedUnion('type', [z.object({instance_id: z.string(), type: z.literal("fixed"), }), z.object({model_id: z.string(), type: z.literal("dynamic"), }), ]));
export type InstanceAllocationRequest = z.infer<ReturnType<typeof InstanceAllocationRequest>>;

export const InstanceDriverEvent = memoizeOne(() => z.discriminatedUnion('type', [z.object({connected: z.boolean(), type: z.literal("connected"), }), z.object({state: z.lazy(InstancePowerState), type: z.literal("powerStateChanged"), }), z.object({state: z.lazy(InstancePlayState), type: z.literal("playStateChanged"), }), z.object({capturedAt: z.coerce.date(), channel: z.number().int(), instanceId: z.string(), reportId: z.string(), type: z.literal("report"), value: z.number(), }), ]));
export type InstanceDriverEvent = z.infer<ReturnType<typeof InstanceDriverEvent>>;

export const InstancePlayState = memoizeOne(() => z.union([z.enum(["rewinding", "idle", "busy", ]), z.object({playing: z.object({duration: z.number(), play_id: z.number().int(), }), }), ]));
export type InstancePlayState = z.infer<ReturnType<typeof InstancePlayState>>;

export const InstancePowerState = memoizeOne(() => z.enum(["off", "coolingDown", "on", "warmingUp", ]));
export type InstancePowerState = z.infer<ReturnType<typeof InstancePowerState>>;

export const MediaId = memoizeOne(() => z.string());
export type MediaId = z.infer<ReturnType<typeof MediaId>>;

export const NodeEvent = memoizeOne(() => z.object({report: z.object({channel: z.number().int(), name: z.string(), value: z.number(), }), }));
export type NodeEvent = z.infer<ReturnType<typeof NodeEvent>>;

export const NodeId = memoizeOne(() => z.discriminatedUnion('type', [z.object({id: z.number().int(), type: z.literal("source"), }), z.object({id: z.number().int(), type: z.literal("deviceInsert"), }), z.object({id: z.number().int(), type: z.literal("virtualInsert"), }), z.object({id: z.number().int(), type: z.literal("bus"), }), z.object({id: z.number().int(), type: z.literal("deviceSink"), }), z.object({id: z.number().int(), type: z.literal("streamingSink"), }), ]));
export type NodeId = z.infer<ReturnType<typeof NodeId>>;

export const NodeInfo = memoizeOne(() => z.object({latency: z.number().int(), numInputs: z.number().int(), numOutputs: z.number().int(), parameters: z.record(z.lazy(ParameterModel)), reports: z.record(z.lazy(ReportModel)), }));
export type NodeInfo = z.infer<ReturnType<typeof NodeInfo>>;

export const OutputId = memoizeOne(() => z.discriminatedUnion('type', [z.object({id: z.tuple([z.number().int(), z.number().int(), ]), type: z.literal("source"), }), z.object({id: z.tuple([z.number().int(), z.number().int(), ]), type: z.literal("deviceInsert"), }), z.object({id: z.tuple([z.number().int(), z.number().int(), ]), type: z.literal("virtualInsert"), }), z.object({id: z.tuple([z.number().int(), z.number().int(), ]), type: z.literal("bus"), }), ]));
export type OutputId = z.infer<ReturnType<typeof OutputId>>;

export const ParameterModel = memoizeOne(() => z.object({channels: z.number().int(), metadata: z.record(z.any()), range: z.lazy(ValueRange), step: z.union([z.number(), z.null(), ]), unit: z.union([z.string(), z.null(), ]), }));
export type ParameterModel = z.infer<ReturnType<typeof ParameterModel>>;

export const PlayHead = memoizeOne(() => z.object({bufferSize: z.number().int(), generation: z.number().int(), playId: z.number().int(), playRegion: z.lazy(PlayRegion), position: z.number().int(), sampleRate: z.number().int(), }));
export type PlayHead = z.infer<ReturnType<typeof PlayHead>>;

export const PlayRegion = memoizeOne(() => z.object({end: z.number().int(), looping: z.boolean(), start: z.number().int(), }));
export type PlayRegion = z.infer<ReturnType<typeof PlayRegion>>;

export const ReportModel = memoizeOne(() => z.object({channels: z.number().int(), metadata: z.record(z.any()), range: z.lazy(ValueRange), unit: z.union([z.string(), z.null(), ]), }));
export type ReportModel = z.infer<ReturnType<typeof ReportModel>>;

export const SourceSpec = memoizeOne(() => z.object({mediaId: z.lazy(MediaId), numChannels: z.number().int(), startAt: z.number().int(), }));
export type SourceSpec = z.infer<ReturnType<typeof SourceSpec>>;

export const TaskEvent = memoizeOne(() => z.object({instanceEvents: z.array(z.lazy(InstanceDriverEvent)), playId: z.union([z.string(), z.null(), ]), playerEvents: z.array(z.lazy(GraphPlayerEvent)), }));
export type TaskEvent = z.infer<ReturnType<typeof TaskEvent>>;

export const TaskSpec = memoizeOne(() => z.object({app_id: z.string(), from: z.coerce.date(), graph_spec: z.lazy(AudioGraphSpec), host_id: z.string(), instances: z.record(z.string()), requests: z.record(z.lazy(InstanceAllocationRequest)), to: z.coerce.date(), }));
export type TaskSpec = z.infer<ReturnType<typeof TaskSpec>>;

export const ValueRange = memoizeOne(() => z.discriminatedUnion('type', [z.object({type: z.literal("toggle"), }), z.object({max: z.number(), min: z.number(), step: z.union([z.number(), z.null(), ]), type: z.literal("bounded"), }), z.object({type: z.literal("list"), values: z.array(z.number()), }), ]));
export type ValueRange = z.infer<ReturnType<typeof ValueRange>>;

export const VirtualInsertSpec = memoizeOne(() => z.object({inputs: z.array(z.array(z.lazy(OutputId))), modelId: z.string(), }));
export type VirtualInsertSpec = z.infer<ReturnType<typeof VirtualInsertSpec>>;
