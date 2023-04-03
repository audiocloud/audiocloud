import { z } from "zod";

export const BinaryPosition = z.union([
  z.object({ byte: z.number().int() }),
  z.object({ bytes: z.tuple([z.number().int(), z.number().int()]) }),
  z.object({ bit: z.tuple([z.number().int(), z.number().int()]) }),
  z.object({
    bitRange: z.array(z.tuple([z.number().int(), z.number().int()])),
  }),
]);
export type BinaryPosition = z.infer<typeof BinaryPosition>;

export const Clamp = z.object({ max: z.number(), min: z.number() });
export type Clamp = z.infer<typeof Clamp>;

export const DesiredInstancePlayState = z.union([
  z.literal("stop"),
  z.object({
    play: z.object({ duration: z.number(), play_id: z.number().int() }),
  }),
]);
export type DesiredInstancePlayState = z.infer<typeof DesiredInstancePlayState>;

export const DesiredInstancePowerState = z.enum(["off", "on"]);
export type DesiredInstancePowerState = z.infer<
  typeof DesiredInstancePowerState
>;

export const InstanceDriverConfig = z.discriminatedUnion("type", [
  z.object({
    frameMask: z.number().int(),
    parameterPages: z.array(z.lazy(() => UsbHidParameterPage)),
    parameters: z.record(z.array(z.lazy(() => UsbHidParameterConfig))),
    productId: z.union([z.number().int(), z.null()]),
    readIntervalMs: z.number().int(),
    readPageHandler: z.union([z.string(), z.null()]),
    reportPages: z.array(z.lazy(() => UsbHidReportPage)),
    reports: z.record(z.array(z.lazy(() => UsbHidReportConfig))),
    serialNumber: z.union([z.string(), z.null()]),
    type: z.literal("USBHID"),
    vendorId: z.union([z.number().int(), z.null()]),
  }),
  z.object({
    baudRate: z.number().int(),
    commentsStartWith: z.array(z.string()),
    errorsStartWith: z.array(z.string()),
    flowControl: z.union([z.lazy(() => SerialFlowControl), z.null()]),
    lineHandler: z.union([z.string(), z.null()]),
    parameters: z.record(z.array(z.lazy(() => SerialParameterConfig))),
    productId: z.union([z.number().int(), z.null()]),
    readResponseAfterEverySend: z.boolean(),
    receiveLineTerminator: z.string(),
    reports: z.record(z.array(z.lazy(() => SerialReportConfig))),
    sendLineTerminator: z.string(),
    serialNumber: z.union([z.string(), z.null()]),
    serialPort: z.union([z.string(), z.null()]),
    type: z.literal("serial"),
    vendorId: z.union([z.number().int(), z.null()]),
  }),
  z.object({
    host: z.string(),
    parameters: z.record(z.array(z.lazy(() => OscParameterConfig))),
    port: z.number().int(),
    type: z.literal("OSC"),
    useTcp: z.boolean(),
  }),
]);
export type InstanceDriverConfig = z.infer<typeof InstanceDriverConfig>;

export const InstanceDriverEvent = z.object({
  capturedAt: z.coerce.date(),
  channel: z.number().int(),
  instanceId: z.string(),
  reportId: z.string(),
  type: z.literal("report"),
  value: z.number(),
});
export type InstanceDriverEvent = z.infer<typeof InstanceDriverEvent>;

export const OscParameterConfig = z.object({});
export type OscParameterConfig = z.infer<typeof OscParameterConfig>;

export const Remap = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("linear"),
    values: z.array(z.number()),
  }),
  z.object({
    pairs: z.array(z.tuple([z.number(), z.number()])),
    type: z.literal("pairs"),
  }),
]);
export type Remap = z.infer<typeof Remap>;

export const Rescale = z.object({
  from: z.tuple([z.number(), z.number()]),
  to: z.tuple([z.number(), z.number()]),
});
export type Rescale = z.infer<typeof Rescale>;

export const SerialDriverConfig = z.object({
  baudRate: z.number().int(),
  commentsStartWith: z.array(z.string()),
  errorsStartWith: z.array(z.string()),
  flowControl: z.union([z.lazy(() => SerialFlowControl), z.null()]),
  lineHandler: z.union([z.string(), z.null()]),
  parameters: z.record(z.array(z.lazy(() => SerialParameterConfig))),
  productId: z.union([z.number().int(), z.null()]),
  readResponseAfterEverySend: z.boolean(),
  receiveLineTerminator: z.string(),
  reports: z.record(z.array(z.lazy(() => SerialReportConfig))),
  sendLineTerminator: z.string(),
  serialNumber: z.union([z.string(), z.null()]),
  serialPort: z.union([z.string(), z.null()]),
  vendorId: z.union([z.number().int(), z.null()]),
});
export type SerialDriverConfig = z.infer<typeof SerialDriverConfig>;

export const SerialFlowControl = z.enum(["xonXoff", "rtsCts"]);
export type SerialFlowControl = z.infer<typeof SerialFlowControl>;

export const SerialParameterConfig = z.object({
  clamp: z.union([z.lazy(() => Clamp), z.null()]),
  formatString: z.union([z.string(), z.null()]),
  lineTerminator: z.union([z.string(), z.null()]),
  remap: z.union([z.lazy(() => Remap), z.null()]),
  rescale: z.union([z.lazy(() => Rescale), z.null()]),
  toString: z.union([z.string(), z.null()]),
  transform: z.union([z.string(), z.null()]),
});
export type SerialParameterConfig = z.infer<typeof SerialParameterConfig>;

export const SerialReportConfig = z.object({
  clamp: z.union([z.lazy(() => Clamp), z.null()]),
  matcher: z.lazy(() => SerialReportMatcher),
  remap: z.union([z.lazy(() => Remap), z.null()]),
  requestTimer: z.union([z.lazy(() => SerialRequestTimer), z.null()]),
  rescale: z.union([z.lazy(() => Rescale), z.null()]),
  value: z.lazy(() => SerialReportValueInterpretation),
});
export type SerialReportConfig = z.infer<typeof SerialReportConfig>;

export const SerialReportMatcher = z.discriminatedUnion("type", [
  z.object({ prefix: z.string(), type: z.literal("stringPrefix") }),
  z.object({
    regex: z.string(),
    type: z.literal("matches"),
  }),
]);
export type SerialReportMatcher = z.infer<typeof SerialReportMatcher>;

export const SerialReportValueInterpretation = z.discriminatedUnion("type", [
  z.object({ type: z.literal("parseFloat") }),
  z.object({
    format: z.string(),
    type: z.literal("parseDateTimeToSeconds"),
  }),
  z.object({ base: z.number().int(), type: z.literal("parseInteger") }),
  z.object({ function: z.string(), type: z.literal("custom") }),
]);
export type SerialReportValueInterpretation = z.infer<
  typeof SerialReportValueInterpretation
>;

export const SerialRequestTimer = z.object({
  intervalMs: z.number().int(),
  line: z.string(),
});
export type SerialRequestTimer = z.infer<typeof SerialRequestTimer>;

export const SetInstanceParameterRequest = z.object({
  channel: z.number().int(),
  parameter: z.string(),
  value: z.number(),
});
export type SetInstanceParameterRequest = z.infer<
  typeof SetInstanceParameterRequest
>;

export const SetInstancePlayRequest = z.object({
  play: z.lazy(() => DesiredInstancePlayState),
});
export type SetInstancePlayRequest = z.infer<typeof SetInstancePlayRequest>;

export const SetInstancePowerRequest = z.object({
  channel: z.number().int(),
  power: z.lazy(() => DesiredInstancePowerState),
});
export type SetInstancePowerRequest = z.infer<typeof SetInstancePowerRequest>;

export const UsbHidDriverConfig = z.object({
  frameMask: z.number().int(),
  parameterPages: z.array(z.lazy(() => UsbHidParameterPage)),
  parameters: z.record(z.array(z.lazy(() => UsbHidParameterConfig))),
  productId: z.union([z.number().int(), z.null()]),
  readIntervalMs: z.number().int(),
  readPageHandler: z.union([z.string(), z.null()]),
  reportPages: z.array(z.lazy(() => UsbHidReportPage)),
  reports: z.record(z.array(z.lazy(() => UsbHidReportConfig))),
  serialNumber: z.union([z.string(), z.null()]),
  vendorId: z.union([z.number().int(), z.null()]),
});
export type UsbHidDriverConfig = z.infer<typeof UsbHidDriverConfig>;

export const UsbHidParameterConfig = z.object({
  clamp: z.union([z.lazy(() => Clamp), z.null()]),
  packing: z.lazy(() => ValuePacking),
  page: z.number().int(),
  position: z.lazy(() => BinaryPosition),
  remap: z.union([z.lazy(() => Remap), z.null()]),
  rescale: z.union([z.lazy(() => Rescale), z.null()]),
  transform: z.union([z.string(), z.null()]),
});
export type UsbHidParameterConfig = z.infer<typeof UsbHidParameterConfig>;

export const UsbHidParameterPage = z.object({
  copyFromReportPage: z.union([z.number().int(), z.null()]),
  header: z.array(z.number().int()),
  page: z.number().int(),
  size: z.number().int(),
});
export type UsbHidParameterPage = z.infer<typeof UsbHidParameterPage>;

export const UsbHidReportConfig = z.object({
  packing: z.lazy(() => ValuePacking),
  page: z.number().int(),
  position: z.lazy(() => BinaryPosition),
  remap: z.union([z.lazy(() => Remap), z.null()]),
  requestTimers: z.array(z.lazy(() => SerialRequestTimer)),
  rescale: z.union([z.lazy(() => Rescale), z.null()]),
  transform: z.union([z.string(), z.null()]),
});
export type UsbHidReportConfig = z.infer<typeof UsbHidReportConfig>;

export const UsbHidReportPage = z.object({
  page: z.number().int(),
  size: z.number().int(),
});
export type UsbHidReportPage = z.infer<typeof UsbHidReportPage>;

export const ValuePacking = z.enum([
  "uint8",
  "uint16le",
  "uint16be",
  "uint32le",
  "uint32be",
  "int8",
  "int16le",
  "int16be",
  "int32le",
  "int32be",
  "float32le",
  "float32be",
  "float64le",
  "float64be",
]);
export type ValuePacking = z.infer<typeof ValuePacking>;

export const WsDriverEvent = z.discriminatedUnion("type", [
  z.object({
    capturedAt: z.coerce.date(),
    channel: z.number().int(),
    instanceId: z.string(),
    reportId: z.string(),
    type: z.literal("report"),
    value: z.number(),
  }),
  z.object({
    config: z.lazy(() => InstanceDriverConfig),
    type: z.literal("config"),
  }),
  z.object({ type: z.literal("keepAlive") }),
]);
export type WsDriverEvent = z.infer<typeof WsDriverEvent>;

export const WsDriverRequest = z.object({
  channel: z.number().int(),
  parameter: z.string(),
  type: z.literal("setParameter"),
  value: z.number(),
});
export type WsDriverRequest = z.infer<typeof WsDriverRequest>;
