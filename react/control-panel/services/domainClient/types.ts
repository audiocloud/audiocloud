import memoizeOne from "memoize-one";
import { z } from "zod";
export const AudioGraphSpec = memoizeOne(() => z.object({busses: z.record(z.lazy(BusSpec)), deviceInserts: z.record(z.lazy(DeviceInsertSpec)), sources: z.record(z.lazy(SourceSpec)), virtualInserts: z.record(z.lazy(VirtualInsertSpec)), }));
export type AudioGraphSpec = z.infer<ReturnType<typeof AudioGraphSpec>>;

export const BinaryPosition = memoizeOne(() => z.union([z.object({byte: z.number().int(), }), z.object({bytes: z.tuple([z.number().int(), z.number().int(), ]), }), z.object({bit: z.tuple([z.number().int(), z.number().int(), ]), }), z.object({bitRange: z.array(z.tuple([z.number().int(), z.number().int(), ])), }), ]));
export type BinaryPosition = z.infer<ReturnType<typeof BinaryPosition>>;

export const BusSpec = memoizeOne(() => z.object({inputs: z.array(z.array(z.lazy(OutputId))), numOutputs: z.number().int(), }));
export type BusSpec = z.infer<ReturnType<typeof BusSpec>>;

export const Clamp = memoizeOne(() => z.object({max: z.number(), min: z.number(), }));
export type Clamp = z.infer<ReturnType<typeof Clamp>>;

export const CreateTaskRequest = memoizeOne(() => z.object({appId: z.string(), from: z.coerce.date(), devices: z.record(z.lazy(InstanceAllocationRequest)), taskId: z.union([z.string(), z.null(), ]), to: z.coerce.date(), }));
export type CreateTaskRequest = z.infer<ReturnType<typeof CreateTaskRequest>>;

export const CreateTaskResponse = memoizeOne(() => z.discriminatedUnion('type', [z.object({app_id: z.string(), task_id: z.string(), type: z.literal("success"), }), z.object({type: z.literal("overlappingTask"), }), z.object({device_id: z.string(), type: z.literal("noSuchInstance"), }), ]));
export type CreateTaskResponse = z.infer<ReturnType<typeof CreateTaskResponse>>;

export const CreateUserRequest = memoizeOne(() => z.object({password: z.string(), }));
export type CreateUserRequest = z.infer<ReturnType<typeof CreateUserRequest>>;

export const CreateUserResponse = memoizeOne(() => z.object({id: z.string(), }));
export type CreateUserResponse = z.infer<ReturnType<typeof CreateUserResponse>>;

export const DeleteUserResponse = memoizeOne(() => z.object({deleted: z.boolean(), id: z.string(), }));
export type DeleteUserResponse = z.infer<ReturnType<typeof DeleteUserResponse>>;

export const DesiredInstancePlayState = memoizeOne(() => z.union([z.literal("stop"), z.object({play: z.object({duration: z.number(), play_id: z.number().int(), }), }), ]));
export type DesiredInstancePlayState = z.infer<ReturnType<typeof DesiredInstancePlayState>>;

export const DesiredInstancePowerState = memoizeOne(() => z.enum(["off", "on", ]));
export type DesiredInstancePowerState = z.infer<ReturnType<typeof DesiredInstancePowerState>>;

export const DesiredTaskPlayState = memoizeOne(() => z.discriminatedUnion('type', [z.object({type: z.literal("idle"), }), z.object({end: z.number().int(), looping: z.boolean(), playId: z.number().int(), sinks: z.record(z.lazy(SinkSpec)), start: z.number().int(), startFrom: z.number().int(), type: z.literal("play"), }), ]));
export type DesiredTaskPlayState = z.infer<ReturnType<typeof DesiredTaskPlayState>>;

export const DeviceInsertSpec = memoizeOne(() => z.object({inputs: z.array(z.array(z.lazy(OutputId))), instanceId: z.string(), }));
export type DeviceInsertSpec = z.infer<ReturnType<typeof DeviceInsertSpec>>;

export const DriverServiceSpec = memoizeOne(() => z.object({driverId: z.string(), instanceIds: z.array(z.string()), }));
export type DriverServiceSpec = z.infer<ReturnType<typeof DriverServiceSpec>>;

export const GraphPlaybackError = memoizeOne(() => z.discriminatedUnion('type', [z.object({error: z.string(), sink: z.number().int(), type: z.literal("incompatibleSinks"), }), z.object({play_id: z.number().int(), type: z.literal("graphAlreadyPlaying"), }), ]));
export type GraphPlaybackError = z.infer<ReturnType<typeof GraphPlaybackError>>;

export const GraphPlaybackState = memoizeOne(() => z.discriminatedUnion('type', [z.object({position: z.number().int(), type: z.literal("buffering"), }), z.object({position: z.number().int(), type: z.literal("playing"), }), z.object({type: z.literal("stopped"), }), ]));
export type GraphPlaybackState = z.infer<ReturnType<typeof GraphPlaybackState>>;

export const GraphPlayerEvent = memoizeOne(() => z.discriminatedUnion('type', [z.object({details: z.lazy(GraphPlaybackError), type: z.literal("error"), }), z.object({details: z.object({state: z.lazy(GraphPlaybackState), }), type: z.literal("graphStateChanged"), }), z.object({details: z.object({nodes: z.record(z.lazy(NodeInfo)), play_id: z.number().int(), }), type: z.literal("graphNodesPrepared"), }), z.object({details: z.object({data: z.array(z.number().int()), play_head: z.lazy(PlayHead), play_id: z.number().int(), sink_id: z.number().int(), }), type: z.literal("graphSinkCaptured"), }), z.object({details: z.object({events: z.array(z.lazy(NodeEvent)), node_id: z.lazy(NodeId), play_id: z.number().int(), }), type: z.literal("nodeEvents"), }), ]));
export type GraphPlayerEvent = z.infer<ReturnType<typeof GraphPlayerEvent>>;

export const HttpDriverParameter = memoizeOne(() => z.object({body: z.union([z.string(), z.null(), ]), headers: z.record(z.string()), method: z.lazy(HttpMethod), url: z.string(), }));
export type HttpDriverParameter = z.infer<ReturnType<typeof HttpDriverParameter>>;

export const HttpDriverReport = memoizeOne(() => z.object({body: z.union([z.string(), z.null(), ]), method: z.lazy(HttpMethod), path: z.string(), pollTimeMs: z.number().int(), response: z.string(), }));
export type HttpDriverReport = z.infer<ReturnType<typeof HttpDriverReport>>;

export const HttpMethod = memoizeOne(() => z.enum(["GET", "PUT", "POST", ]));
export type HttpMethod = z.infer<ReturnType<typeof HttpMethod>>;

export const InstanceAllocationRequest = memoizeOne(() => z.discriminatedUnion('type', [z.object({device_id: z.string(), type: z.literal("fixed"), }), z.object({model_id: z.string(), type: z.literal("dynamic"), }), ]));
export type InstanceAllocationRequest = z.infer<ReturnType<typeof InstanceAllocationRequest>>;

export const InstanceAttachment = memoizeOne(() => z.object({device: z.string(), inputs: z.array(z.number().int()), outputs: z.array(z.number().int()), }));
export type InstanceAttachment = z.infer<ReturnType<typeof InstanceAttachment>>;

export const InstanceDriverConfig = memoizeOne(() => z.discriminatedUnion('type', [z.object({frameMask: z.number().int(), parameterPages: z.array(z.lazy(UsbHidParameterPage)), parameters: z.record(z.array(z.lazy(UsbHidParameterConfig))), productId: z.union([z.number().int(), z.null(), ]), readDurationMs: z.number().int(), readPageHandler: z.union([z.string(), z.null(), ]), reportPages: z.array(z.lazy(UsbHidReportPage)), reports: z.record(z.array(z.lazy(UsbHidReportConfig))), serialNumber: z.union([z.string(), z.null(), ]), type: z.literal("USBHID"), vendorId: z.union([z.number().int(), z.null(), ]), }), z.object({baudRate: z.number().int(), commentsStartWith: z.array(z.string()), errorsStartWith: z.array(z.string()), flowControl: z.union([z.lazy(SerialFlowControl), z.null(), ]), lineHandler: z.union([z.string(), z.null(), ]), parameters: z.record(z.array(z.lazy(SerialParameterConfig))), productId: z.union([z.number().int(), z.null(), ]), readResponseAfterEverySend: z.boolean(), receiveLineTerminator: z.string(), reports: z.record(z.array(z.lazy(SerialReportConfig))), sendLineTerminator: z.string(), serialNumber: z.union([z.string(), z.null(), ]), serialPort: z.union([z.string(), z.null(), ]), type: z.literal("serial"), vendorId: z.union([z.number().int(), z.null(), ]), }), z.object({host: z.string(), parameters: z.record(z.array(z.lazy(OscParameterConfig))), port: z.number().int(), type: z.literal("OSC"), useTcp: z.boolean(), }), z.object({baseUrl: z.string(), parameters: z.record(z.lazy(HttpDriverParameter)), reports: z.record(z.lazy(HttpDriverReport)), type: z.literal("HTTP"), }), z.object({type: z.literal("SPI"), }), z.object({type: z.literal("mock"), }), ]));
export type InstanceDriverConfig = z.infer<ReturnType<typeof InstanceDriverConfig>>;

export const InstanceDriverEvent = memoizeOne(() => z.discriminatedUnion('type', [z.object({connected: z.boolean(), type: z.literal("connected"), }), z.object({state: z.lazy(InstancePowerState), type: z.literal("powerStateChanged"), }), z.object({state: z.lazy(InstancePlayState), type: z.literal("playStateChanged"), }), z.object({capturedAt: z.coerce.date(), channel: z.number().int(), instanceId: z.string(), reportId: z.string(), type: z.literal("report"), value: z.number(), }), ]));
export type InstanceDriverEvent = z.infer<ReturnType<typeof InstanceDriverEvent>>;

export const InstanceDriverReportEvent = memoizeOne(() => z.object({capturedAt: z.coerce.date(), channel: z.number().int(), instanceId: z.string(), reportId: z.string(), value: z.number(), }));
export type InstanceDriverReportEvent = z.infer<ReturnType<typeof InstanceDriverReportEvent>>;

export const InstanceFeature = memoizeOne(() => z.enum(["mediaTransport", "midiNoteOnOff", "digitalInputOutput", "routing", ]));
export type InstanceFeature = z.infer<ReturnType<typeof InstanceFeature>>;

export const InstanceMediaSpec = memoizeOne(() => z.object({durationMs: z.number().int(), play: z.lazy(SetParameterCommand), positionReport: z.string(), reportTriggers: z.array(z.lazy(PlayStateReportTrigger)), rewind: z.lazy(SetParameterCommand), stop: z.lazy(SetParameterCommand), }));
export type InstanceMediaSpec = z.infer<ReturnType<typeof InstanceMediaSpec>>;

export const InstanceModel = memoizeOne(() => z.object({audioInputs: z.number().int(), audioOutputs: z.number().int(), parameters: z.record(z.lazy(ParameterModel)), reports: z.record(z.lazy(ReportModel)), supports: z.array(z.lazy(InstanceFeature)), }));
export type InstanceModel = z.infer<ReturnType<typeof InstanceModel>>;

export const InstancePlayControl = memoizeOne(() => z.object({desired: z.lazy(DesiredInstancePlayState), until: z.coerce.date(), }));
export type InstancePlayControl = z.infer<ReturnType<typeof InstancePlayControl>>;

export const InstancePlayState = memoizeOne(() => z.union([z.enum(["rewinding", "idle", "busy", ]), z.object({playing: z.object({duration: z.number(), play_id: z.number().int(), }), }), ]));
export type InstancePlayState = z.infer<ReturnType<typeof InstancePlayState>>;

export const InstancePlayStateTransition = memoizeOne(() => z.enum(["setRewinding", "setIdle", "setBusy", "setPlaying", ]));
export type InstancePlayStateTransition = z.infer<ReturnType<typeof InstancePlayStateTransition>>;

export const InstancePowerControl = memoizeOne(() => z.object({desired: z.lazy(DesiredInstancePowerState), until: z.coerce.date(), }));
export type InstancePowerControl = z.infer<ReturnType<typeof InstancePowerControl>>;

export const InstancePowerSpec = memoizeOne(() => z.object({coolDownMs: z.number().int(), driverNeedsPower: z.boolean(), idleMs: z.number().int(), powerController: z.string(), powerOff: z.lazy(SetParameterCommand), powerOn: z.lazy(SetParameterCommand), warmUpMs: z.number().int(), }));
export type InstancePowerSpec = z.infer<ReturnType<typeof InstancePowerSpec>>;

export const InstancePowerState = memoizeOne(() => z.enum(["off", "coolingDown", "on", "warmingUp", ]));
export type InstancePowerState = z.infer<ReturnType<typeof InstancePowerState>>;

export const InstanceSpec = memoizeOne(() => z.object({attachment: z.union([z.lazy(InstanceAttachment), z.null(), ]), driver: z.lazy(InstanceDriverConfig), host: z.string(), media: z.union([z.lazy(InstanceMediaSpec), z.null(), ]), model: z.lazy(InstanceModel), power: z.union([z.lazy(InstancePowerSpec), z.null(), ]), }));
export type InstanceSpec = z.infer<ReturnType<typeof InstanceSpec>>;

export const LoginUserResponse = memoizeOne(() => z.object({token: z.string(), }));
export type LoginUserResponse = z.infer<ReturnType<typeof LoginUserResponse>>;

export const MediaDownloadSpec = memoizeOne(() => z.object({fromUrl: z.string(), sha256: z.string(), size: z.number().int(), }));
export type MediaDownloadSpec = z.infer<ReturnType<typeof MediaDownloadSpec>>;

export const MediaDownloadState = memoizeOne(() => z.object({done: z.union([z.lazy(MediaSpec), z.null(), ]), error: z.union([z.string(), z.null(), ]), progress: z.number(), updatedAt: z.coerce.date(), }));
export type MediaDownloadState = z.infer<ReturnType<typeof MediaDownloadState>>;

export const MediaId = memoizeOne(() => z.string());
export type MediaId = z.infer<ReturnType<typeof MediaId>>;

export const MediaSpec = memoizeOne(() => z.object({id: z.lazy(MediaId), sha256: z.string(), }));
export type MediaSpec = z.infer<ReturnType<typeof MediaSpec>>;

export const MediaUploadSpec = memoizeOne(() => z.object({toUrl: z.string(), }));
export type MediaUploadSpec = z.infer<ReturnType<typeof MediaUploadSpec>>;

export const MediaUploadState = memoizeOne(() => z.object({error: z.union([z.string(), z.null(), ]), progress: z.number(), updatedAt: z.coerce.date(), uploaded: z.boolean(), }));
export type MediaUploadState = z.infer<ReturnType<typeof MediaUploadState>>;

export const NodeEvent = memoizeOne(() => z.object({report: z.object({channel: z.number().int(), name: z.string(), value: z.number(), }), }));
export type NodeEvent = z.infer<ReturnType<typeof NodeEvent>>;

export const NodeId = memoizeOne(() => z.discriminatedUnion('type', [z.object({id: z.number().int(), type: z.literal("source"), }), z.object({id: z.number().int(), type: z.literal("deviceInsert"), }), z.object({id: z.number().int(), type: z.literal("virtualInsert"), }), z.object({id: z.number().int(), type: z.literal("bus"), }), z.object({id: z.number().int(), type: z.literal("deviceSink"), }), z.object({id: z.number().int(), type: z.literal("streamingSink"), }), ]));
export type NodeId = z.infer<ReturnType<typeof NodeId>>;

export const NodeInfo = memoizeOne(() => z.object({latency: z.number().int(), numInputs: z.number().int(), numOutputs: z.number().int(), parameters: z.record(z.lazy(ParameterModel)), reports: z.record(z.lazy(ReportModel)), }));
export type NodeInfo = z.infer<ReturnType<typeof NodeInfo>>;

export const OscParameterConfig = memoizeOne(() => z.object({address: z.string(), clamp: z.union([z.lazy(Clamp), z.null(), ]), oscType: z.string(), remap: z.union([z.lazy(Remap), z.null(), ]), rescale: z.union([z.lazy(Rescale), z.null(), ]), transform: z.union([z.string(), z.null(), ]), }));
export type OscParameterConfig = z.infer<ReturnType<typeof OscParameterConfig>>;

export const OutputId = memoizeOne(() => z.discriminatedUnion('type', [z.object({id: z.tuple([z.number().int(), z.number().int(), ]), type: z.literal("source"), }), z.object({id: z.tuple([z.number().int(), z.number().int(), ]), type: z.literal("deviceInsert"), }), z.object({id: z.tuple([z.number().int(), z.number().int(), ]), type: z.literal("virtualInsert"), }), z.object({id: z.tuple([z.number().int(), z.number().int(), ]), type: z.literal("bus"), }), ]));
export type OutputId = z.infer<ReturnType<typeof OutputId>>;

export const ParameterModel = memoizeOne(() => z.object({channels: z.number().int(), metadata: z.record(z.any()), range: z.lazy(ValueRange), step: z.union([z.number(), z.null(), ]), unit: z.union([z.string(), z.null(), ]), }));
export type ParameterModel = z.infer<ReturnType<typeof ParameterModel>>;

export const PlayHead = memoizeOne(() => z.object({bufferSize: z.number().int(), generation: z.number().int(), playId: z.number().int(), playRegion: z.lazy(PlayRegion), position: z.number().int(), sampleRate: z.number().int(), }));
export type PlayHead = z.infer<ReturnType<typeof PlayHead>>;

export const PlayRegion = memoizeOne(() => z.object({end: z.number().int(), looping: z.boolean(), start: z.number().int(), }));
export type PlayRegion = z.infer<ReturnType<typeof PlayRegion>>;

export const PlayStateReportTrigger = memoizeOne(() => z.object({equals: z.union([z.number(), z.null(), ]), greaterThan: z.union([z.number(), z.null(), ]), lessThan: z.union([z.number(), z.null(), ]), report: z.string(), then: z.lazy(InstancePlayStateTransition), }));
export type PlayStateReportTrigger = z.infer<ReturnType<typeof PlayStateReportTrigger>>;

export const RegisterOrUpdateInstanceRequest = memoizeOne(() => z.object({driverConfig: z.lazy(InstanceDriverConfig), driverId: z.string(), id: z.string(), modelId: z.string(), playSpec: z.union([z.lazy(InstanceMediaSpec), z.null(), ]), powerSpec: z.union([z.lazy(InstancePowerSpec), z.null(), ]), }));
export type RegisterOrUpdateInstanceRequest = z.infer<ReturnType<typeof RegisterOrUpdateInstanceRequest>>;

export const RegisterOrUpdateInstanceResponse = memoizeOne(() => z.literal("success"));
export type RegisterOrUpdateInstanceResponse = z.infer<ReturnType<typeof RegisterOrUpdateInstanceResponse>>;

export const Remap = memoizeOne(() => z.discriminatedUnion('type', [z.object({type: z.literal("linear"), values: z.array(z.number()), }), z.object({pairs: z.array(z.tuple([z.number(), z.number(), ])), type: z.literal("pairs"), }), ]));
export type Remap = z.infer<ReturnType<typeof Remap>>;

export const ReportModel = memoizeOne(() => z.object({channels: z.number().int(), metadata: z.record(z.any()), range: z.lazy(ValueRange), unit: z.union([z.string(), z.null(), ]), }));
export type ReportModel = z.infer<ReturnType<typeof ReportModel>>;

export const Rescale = memoizeOne(() => z.object({from: z.tuple([z.number(), z.number(), ]), to: z.tuple([z.number(), z.number(), ]), }));
export type Rescale = z.infer<ReturnType<typeof Rescale>>;

export const RtCommand = memoizeOne(() => z.discriminatedUnion('type', [z.object({instanceId: z.string(), power: z.lazy(InstancePowerControl), type: z.literal("setInstancePowerControl"), }), z.object({instanceId: z.string(), play: z.lazy(InstancePlayControl), type: z.literal("setInstancePlayControl"), }), z.object({changes: z.array(z.lazy(SetInstanceParameter)), instanceId: z.string(), type: z.literal("setInstanceParameters"), }), z.object({instanceId: z.string(), type: z.literal("subscribeToInstanceEvents"), }), z.object({instanceId: z.string(), type: z.literal("unsubscribeFromInstanceEvents"), }), ]));
export type RtCommand = z.infer<ReturnType<typeof RtCommand>>;

export const RtEvent = memoizeOne(() => z.discriminatedUnion('type', [z.object({requestId: z.string(), success: z.boolean(), type: z.literal("setInstancePowerControl"), }), z.object({requestId: z.string(), success: z.boolean(), type: z.literal("setInstancePlayControl"), }), z.object({instanceId: z.string(), spec: z.union([z.lazy(InstanceSpec), z.null(), ]), type: z.literal("setInstanceSpec"), }), z.object({requestId: z.string(), response: z.lazy(SetInstanceParameterResponse), type: z.literal("setInstanceParameters"), }), z.object({event: z.lazy(InstanceDriverEvent), instanceId: z.string(), type: z.literal("instanceDriverEvent"), }), z.object({requestId: z.string(), success: z.boolean(), type: z.literal("subscribeToInstanceEvents"), }), z.object({requestId: z.string(), success: z.boolean(), type: z.literal("unsubscribeFromInstanceEvents"), }), ]));
export type RtEvent = z.infer<ReturnType<typeof RtEvent>>;

export const RtRequest = memoizeOne(() => z.object({command: z.lazy(RtCommand), requestId: z.string(), }));
export type RtRequest = z.infer<ReturnType<typeof RtRequest>>;

export const SerialFlowControl = memoizeOne(() => z.enum(["xonXoff", "rtsCts", ]));
export type SerialFlowControl = z.infer<ReturnType<typeof SerialFlowControl>>;

export const SerialParameterConfig = memoizeOne(() => z.object({clamp: z.union([z.lazy(Clamp), z.null(), ]), formatString: z.union([z.string(), z.null(), ]), lineTerminator: z.union([z.string(), z.null(), ]), remap: z.union([z.lazy(Remap), z.null(), ]), rescale: z.union([z.lazy(Rescale), z.null(), ]), toString: z.union([z.string(), z.null(), ]), transform: z.union([z.string(), z.null(), ]), }));
export type SerialParameterConfig = z.infer<ReturnType<typeof SerialParameterConfig>>;

export const SerialReportConfig = memoizeOne(() => z.object({clamp: z.union([z.lazy(Clamp), z.null(), ]), matcher: z.lazy(SerialReportMatcher), remap: z.union([z.lazy(Remap), z.null(), ]), requestTimer: z.union([z.lazy(SerialRequestTimer), z.null(), ]), rescale: z.union([z.lazy(Rescale), z.null(), ]), value: z.lazy(SerialReportValueInterpretation), }));
export type SerialReportConfig = z.infer<ReturnType<typeof SerialReportConfig>>;

export const SerialReportMatcher = memoizeOne(() => z.discriminatedUnion('type', [z.object({prefix: z.string(), skip: z.union([z.number().int(), z.null(), ]), take: z.union([z.number().int(), z.null(), ]), type: z.literal("stringPrefix"), }), z.object({regex: z.string(), type: z.literal("matches"), }), ]));
export type SerialReportMatcher = z.infer<ReturnType<typeof SerialReportMatcher>>;

export const SerialReportValueInterpretation = memoizeOne(() => z.discriminatedUnion('type', [z.object({type: z.literal("parseFloat"), }), z.object({format: z.string(), type: z.literal("parseDateTimeToSeconds"), }), z.object({base: z.number().int(), type: z.literal("parseInteger"), }), z.object({function: z.string(), type: z.literal("custom"), }), ]));
export type SerialReportValueInterpretation = z.infer<ReturnType<typeof SerialReportValueInterpretation>>;

export const SerialRequestTimer = memoizeOne(() => z.object({intervalMs: z.number().int(), line: z.string(), }));
export type SerialRequestTimer = z.infer<ReturnType<typeof SerialRequestTimer>>;

export const SetInstanceParameter = memoizeOne(() => z.object({channel: z.number().int(), parameter: z.string(), value: z.number(), }));
export type SetInstanceParameter = z.infer<ReturnType<typeof SetInstanceParameter>>;

export const SetInstanceParameterResponse = memoizeOne(() => z.enum(["success", "parameterNotFound", "channelNotFound", "notConnected", "encodingError", "connectionError", "rpcFailure", ]));
export type SetInstanceParameterResponse = z.infer<ReturnType<typeof SetInstanceParameterResponse>>;

export const SetParameterCommand = memoizeOne(() => z.object({channel: z.number().int(), parameter: z.string(), value: z.number(), }));
export type SetParameterCommand = z.infer<ReturnType<typeof SetParameterCommand>>;

export const SinkSpec = memoizeOne(() => z.object({inputs: z.array(z.array(z.lazy(OutputId))), sampleRate: z.number().int(), }));
export type SinkSpec = z.infer<ReturnType<typeof SinkSpec>>;

export const SourceSpec = memoizeOne(() => z.object({mediaId: z.lazy(MediaId), numChannels: z.number().int(), startAt: z.number().int(), }));
export type SourceSpec = z.infer<ReturnType<typeof SourceSpec>>;

export const TaskEvent = memoizeOne(() => z.object({instanceEvents: z.array(z.lazy(InstanceDriverEvent)), playId: z.union([z.string(), z.null(), ]), playerEvents: z.array(z.lazy(GraphPlayerEvent)), }));
export type TaskEvent = z.infer<ReturnType<typeof TaskEvent>>;

export const TaskSpec = memoizeOne(() => z.object({app_id: z.string(), from: z.coerce.date(), graph_spec: z.lazy(AudioGraphSpec), host_id: z.string(), devices: z.record(z.string()), requests: z.record(z.lazy(InstanceAllocationRequest)), to: z.coerce.date(), }));
export type TaskSpec = z.infer<ReturnType<typeof TaskSpec>>;

export const UsbHidParameterConfig = memoizeOne(() => z.object({clamp: z.union([z.lazy(Clamp), z.null(), ]), packing: z.lazy(ValuePacking), page: z.number().int(), position: z.lazy(BinaryPosition), remap: z.union([z.lazy(Remap), z.null(), ]), rescale: z.union([z.lazy(Rescale), z.null(), ]), transform: z.union([z.string(), z.null(), ]), }));
export type UsbHidParameterConfig = z.infer<ReturnType<typeof UsbHidParameterConfig>>;

export const UsbHidParameterPage = memoizeOne(() => z.object({copyFromReportPage: z.union([z.number().int(), z.null(), ]), header: z.array(z.number().int()), page: z.number().int(), size: z.number().int(), }));
export type UsbHidParameterPage = z.infer<ReturnType<typeof UsbHidParameterPage>>;

export const UsbHidReportConfig = memoizeOne(() => z.object({packing: z.lazy(ValuePacking), page: z.number().int(), position: z.lazy(BinaryPosition), remap: z.union([z.lazy(Remap), z.null(), ]), rescale: z.union([z.lazy(Rescale), z.null(), ]), transform: z.union([z.string(), z.null(), ]), }));
export type UsbHidReportConfig = z.infer<ReturnType<typeof UsbHidReportConfig>>;

export const UsbHidReportPage = memoizeOne(() => z.object({page: z.number().int(), size: z.number().int(), }));
export type UsbHidReportPage = z.infer<ReturnType<typeof UsbHidReportPage>>;

export const UserSpec = memoizeOne(() => z.object({id: z.string(), password: z.string(), }));
export type UserSpec = z.infer<ReturnType<typeof UserSpec>>;

export const UserSummary = memoizeOne(() => z.object({id: z.string(), }));
export type UserSummary = z.infer<ReturnType<typeof UserSummary>>;

export const ValuePacking = memoizeOne(() => z.enum(["uint8", "uint16le", "uint16be", "uint32le", "uint32be", "int8", "int16le", "int16be", "int32le", "int32be", "float32le", "float32be", "float64le", "float64be", ]));
export type ValuePacking = z.infer<ReturnType<typeof ValuePacking>>;

export const ValueRange = memoizeOne(() => z.discriminatedUnion('type', [z.object({type: z.literal("toggle"), }), z.object({max: z.number(), min: z.number(), step: z.union([z.number(), z.null(), ]), type: z.literal("bounded"), }), z.object({type: z.literal("list"), values: z.array(z.number()), }), ]));
export type ValueRange = z.infer<ReturnType<typeof ValueRange>>;

export const VirtualInsertSpec = memoizeOne(() => z.object({inputs: z.array(z.array(z.lazy(OutputId))), modelId: z.string(), }));
export type VirtualInsertSpec = z.infer<ReturnType<typeof VirtualInsertSpec>>;
