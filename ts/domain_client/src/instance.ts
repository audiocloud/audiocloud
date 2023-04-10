import memoizeOne from "memoize-one";
import { z } from "zod";
export const BinaryPosition = memoizeOne(() => z.union([z.object({byte: z.number().int(), }), z.object({bytes: z.tuple([z.number().int(), z.number().int(), ]), }), z.object({bit: z.tuple([z.number().int(), z.number().int(), ]), }), z.object({bitRange: z.array(z.tuple([z.number().int(), z.number().int(), ])), }), ]));
export type BinaryPosition = z.infer<ReturnType<typeof BinaryPosition>>;

export const Clamp = memoizeOne(() => z.object({max: z.number(), min: z.number(), }));
export type Clamp = z.infer<ReturnType<typeof Clamp>>;

export const DesiredInstancePlayState = memoizeOne(() => z.union([z.literal("stop"), z.object({play: z.object({duration: z.number(), play_id: z.number().int(), }), }), ]));
export type DesiredInstancePlayState = z.infer<ReturnType<typeof DesiredInstancePlayState>>;

export const DesiredInstancePowerState = memoizeOne(() => z.enum(["off", "on", ]));
export type DesiredInstancePowerState = z.infer<ReturnType<typeof DesiredInstancePowerState>>;

export const DriverServiceSpec = memoizeOne(() => z.object({driverId: z.string(), instanceIds: z.array(z.string()), }));
export type DriverServiceSpec = z.infer<ReturnType<typeof DriverServiceSpec>>;

export const HttpDriverParameter = memoizeOne(() => z.object({body: z.union([z.string(), z.null(), ]), headers: z.record(z.string()), method: z.lazy(HttpMethod), path: z.string(), }));
export type HttpDriverParameter = z.infer<ReturnType<typeof HttpDriverParameter>>;

export const HttpDriverReport = memoizeOne(() => z.object({body: z.union([z.string(), z.null(), ]), method: z.lazy(HttpMethod), path: z.string(), pollTimeMs: z.number().int(), response: z.string(), }));
export type HttpDriverReport = z.infer<ReturnType<typeof HttpDriverReport>>;

export const HttpMethod = memoizeOne(() => z.enum(["GET", "PUT", "POST", ]));
export type HttpMethod = z.infer<ReturnType<typeof HttpMethod>>;

export const InstanceDriverConfig = memoizeOne(() => z.discriminatedUnion('type', [z.object({frameMask: z.number().int(), parameterPages: z.array(z.lazy(UsbHidParameterPage)), parameters: z.record(z.array(z.lazy(UsbHidParameterConfig))), productId: z.union([z.number().int(), z.null(), ]), readIntervalMs: z.number().int(), readPageHandler: z.union([z.string(), z.null(), ]), reportPages: z.array(z.lazy(UsbHidReportPage)), reports: z.record(z.array(z.lazy(UsbHidReportConfig))), serialNumber: z.union([z.string(), z.null(), ]), type: z.literal("USBHID"), vendorId: z.union([z.number().int(), z.null(), ]), }), z.object({baudRate: z.number().int(), commentsStartWith: z.array(z.string()), errorsStartWith: z.array(z.string()), flowControl: z.union([z.lazy(SerialFlowControl), z.null(), ]), lineHandler: z.union([z.string(), z.null(), ]), parameters: z.record(z.array(z.lazy(SerialParameterConfig))), productId: z.union([z.number().int(), z.null(), ]), readResponseAfterEverySend: z.boolean(), receiveLineTerminator: z.string(), reports: z.record(z.array(z.lazy(SerialReportConfig))), sendLineTerminator: z.string(), serialNumber: z.union([z.string(), z.null(), ]), serialPort: z.union([z.string(), z.null(), ]), type: z.literal("serial"), vendorId: z.union([z.number().int(), z.null(), ]), }), z.object({host: z.string(), parameters: z.record(z.array(z.lazy(OscParameterConfig))), port: z.number().int(), type: z.literal("OSC"), useTcp: z.boolean(), }), z.object({baseUrl: z.string(), parameters: z.record(z.lazy(HttpDriverParameter)), reports: z.record(z.lazy(HttpDriverReport)), type: z.literal("HTTP"), }), z.object({type: z.literal("SPI"), }), ]));
export type InstanceDriverConfig = z.infer<ReturnType<typeof InstanceDriverConfig>>;

export const InstanceDriverEvent = memoizeOne(() => z.discriminatedUnion('type', [z.object({connected: z.boolean(), type: z.literal("connected"), }), z.object({state: z.lazy(InstancePowerState), type: z.literal("powerStateChanged"), }), z.object({state: z.lazy(InstancePlayState), type: z.literal("playStateChanged"), }), z.object({capturedAt: z.coerce.date(), channel: z.number().int(), instanceId: z.string(), reportId: z.string(), type: z.literal("report"), value: z.number(), }), ]));
export type InstanceDriverEvent = z.infer<ReturnType<typeof InstanceDriverEvent>>;

export const InstanceDriverReportEvent = memoizeOne(() => z.object({capturedAt: z.coerce.date(), channel: z.number().int(), instanceId: z.string(), reportId: z.string(), value: z.number(), }));
export type InstanceDriverReportEvent = z.infer<ReturnType<typeof InstanceDriverReportEvent>>;

export const InstanceFeature = memoizeOne(() => z.enum(["mediaTransport", "midiNoteOnOff", "digitalInputOutput", "routing", ]));
export type InstanceFeature = z.infer<ReturnType<typeof InstanceFeature>>;

export const InstanceMediaSpec = memoizeOne(() => z.object({durationMs: z.number().int(), play: z.lazy(ParameterCommand), positionReport: z.string(), reportTriggers: z.array(z.lazy(PlayStateReportTrigger)), rewind: z.lazy(ParameterCommand), stop: z.lazy(ParameterCommand), }));
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

export const InstancePowerSpec = memoizeOne(() => z.object({coolDownMs: z.number().int(), idleMs: z.number().int(), powerController: z.string(), powerOff: z.lazy(ParameterCommand), powerOn: z.lazy(ParameterCommand), warmUpMs: z.number().int(), }));
export type InstancePowerSpec = z.infer<ReturnType<typeof InstancePowerSpec>>;

export const InstancePowerState = memoizeOne(() => z.enum(["off", "coolingDown", "on", "warmingUp", ]));
export type InstancePowerState = z.infer<ReturnType<typeof InstancePowerState>>;

export const InstanceSpec = memoizeOne(() => z.object({driver: z.lazy(InstanceDriverConfig), host: z.string(), media: z.union([z.lazy(InstanceMediaSpec), z.null(), ]), model: z.lazy(InstanceModel), power: z.union([z.lazy(InstancePowerSpec), z.null(), ]), }));
export type InstanceSpec = z.infer<ReturnType<typeof InstanceSpec>>;

export const OscParameterConfig = memoizeOne(() => z.object({oscType: z.string(), pathTemplate: z.string(), transform: z.union([z.string(), z.null(), ]), }));
export type OscParameterConfig = z.infer<ReturnType<typeof OscParameterConfig>>;

export const ParameterCommand = memoizeOne(() => z.object({channel: z.number().int(), parameter: z.string(), value: z.number(), }));
export type ParameterCommand = z.infer<ReturnType<typeof ParameterCommand>>;

export const ParameterModel = memoizeOne(() => z.object({allowedValues: z.array(z.number()), channels: z.number().int(), max: z.number(), metadata: z.record(z.any()), min: z.number(), step: z.union([z.number(), z.null(), ]), unit: z.union([z.string(), z.null(), ]), }));
export type ParameterModel = z.infer<ReturnType<typeof ParameterModel>>;

export const PlayStateReportTrigger = memoizeOne(() => z.object({equals: z.union([z.number(), z.null(), ]), greaterThan: z.union([z.number(), z.null(), ]), lessThan: z.union([z.number(), z.null(), ]), report: z.string(), then: z.lazy(InstancePlayStateTransition), }));
export type PlayStateReportTrigger = z.infer<ReturnType<typeof PlayStateReportTrigger>>;

export const RegisterOrUpdateInstanceRequest = memoizeOne(() => z.object({driverConfig: z.lazy(InstanceDriverConfig), driverId: z.string(), id: z.string(), modelId: z.string(), playSpec: z.union([z.lazy(InstanceMediaSpec), z.null(), ]), powerSpec: z.union([z.lazy(InstancePowerSpec), z.null(), ]), }));
export type RegisterOrUpdateInstanceRequest = z.infer<ReturnType<typeof RegisterOrUpdateInstanceRequest>>;

export const RegisterOrUpdateInstanceResponse = memoizeOne(() => z.literal("success"));
export type RegisterOrUpdateInstanceResponse = z.infer<ReturnType<typeof RegisterOrUpdateInstanceResponse>>;

export const Remap = memoizeOne(() => z.discriminatedUnion('type', [z.object({type: z.literal("linear"), values: z.array(z.number()), }), z.object({pairs: z.array(z.tuple([z.number(), z.number(), ])), type: z.literal("pairs"), }), ]));
export type Remap = z.infer<ReturnType<typeof Remap>>;

export const ReportModel = memoizeOne(() => z.object({channels: z.number().int(), max: z.number(), metadata: z.record(z.any()), min: z.number(), step: z.union([z.number(), z.null(), ]), unit: z.union([z.string(), z.null(), ]), }));
export type ReportModel = z.infer<ReturnType<typeof ReportModel>>;

export const Rescale = memoizeOne(() => z.object({from: z.tuple([z.number(), z.number(), ]), to: z.tuple([z.number(), z.number(), ]), }));
export type Rescale = z.infer<ReturnType<typeof Rescale>>;

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

export const SetInstanceParameterResponse = memoizeOne(() => z.enum(["success", "parameterNotFound", "channelNotFound", "notConnected", "rpcFailure", ]));
export type SetInstanceParameterResponse = z.infer<ReturnType<typeof SetInstanceParameterResponse>>;

export const UsbHidParameterConfig = memoizeOne(() => z.object({clamp: z.union([z.lazy(Clamp), z.null(), ]), packing: z.lazy(ValuePacking), page: z.number().int(), position: z.lazy(BinaryPosition), remap: z.union([z.lazy(Remap), z.null(), ]), rescale: z.union([z.lazy(Rescale), z.null(), ]), transform: z.union([z.string(), z.null(), ]), }));
export type UsbHidParameterConfig = z.infer<ReturnType<typeof UsbHidParameterConfig>>;

export const UsbHidParameterPage = memoizeOne(() => z.object({copyFromReportPage: z.union([z.number().int(), z.null(), ]), header: z.array(z.number().int()), page: z.number().int(), size: z.number().int(), }));
export type UsbHidParameterPage = z.infer<ReturnType<typeof UsbHidParameterPage>>;

export const UsbHidReportConfig = memoizeOne(() => z.object({packing: z.lazy(ValuePacking), page: z.number().int(), position: z.lazy(BinaryPosition), remap: z.union([z.lazy(Remap), z.null(), ]), rescale: z.union([z.lazy(Rescale), z.null(), ]), transform: z.union([z.string(), z.null(), ]), }));
export type UsbHidReportConfig = z.infer<ReturnType<typeof UsbHidReportConfig>>;

export const UsbHidReportPage = memoizeOne(() => z.object({page: z.number().int(), size: z.number().int(), }));
export type UsbHidReportPage = z.infer<ReturnType<typeof UsbHidReportPage>>;

export const ValuePacking = memoizeOne(() => z.enum(["uint8", "uint16le", "uint16be", "uint32le", "uint32be", "int8", "int16le", "int16be", "int32le", "int32be", "float32le", "float32be", "float64le", "float64be", ]));
export type ValuePacking = z.infer<ReturnType<typeof ValuePacking>>;

export const WsDriverEvent = memoizeOne(() => z.discriminatedUnion('type', [z.object({capturedAt: z.coerce.date(), channel: z.number().int(), instanceId: z.string(), reportId: z.string(), type: z.literal("report"), value: z.number(), }), z.object({config: z.lazy(InstanceDriverConfig), type: z.literal("config"), }), z.object({type: z.literal("keepAlive"), }), ]));
export type WsDriverEvent = z.infer<ReturnType<typeof WsDriverEvent>>;

export const WsDriverRequest = memoizeOne(() => z.object({channel: z.number().int(), parameter: z.string(), type: z.literal("setParameter"), value: z.number(), }));
export type WsDriverRequest = z.infer<ReturnType<typeof WsDriverRequest>>;
