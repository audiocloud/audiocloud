// noinspection JSUnusedGlobalSymbols

import {z} from "zod"

export const SourceId = z.coerce.number().int();
export type SourceId = z.infer<typeof SourceId>;
export const InsertId = z.coerce.number().int();
export type InsertId = z.infer<typeof InsertId>;
export const BusId = z.coerce.number().int();
export type BusId = z.infer<typeof BusId>;
export const SinkId = z.coerce.number().int();
export type SinkId = z.infer<typeof SinkId>;

export const AudioGraphSpec = z.object({
    sources: z.record(SourceId, z.lazy(() => SourceSpec)),
    inserts: z.record(InsertId, z.lazy(() => InsertSpec)),
    busses: z.record(BusId, z.lazy(() => BusSpec)),
});
export type AudioGraphSpec = z.infer<typeof AudioGraphSpec>;

export const SourceSpec = z.object({
    startAt: z.number().int(),
    sourceUrl: z.string().url(),
    numChannels: z.number().int()
});
export type SourceSpec = z.infer<typeof SourceSpec>;

export const InputSpec = z.object({
    sourceId: z.lazy(() => OutputId),
    gain: z.number()
});
export type InputSpec = z.infer<typeof InputSpec>;

export const OutputId = z.discriminatedUnion('type', [
    z.object({type: z.literal('source'), id: z.tuple([SourceId, z.number().int()])}),
    z.object({type: z.literal('insert'), id: z.tuple([InsertId, z.number().int()])}),
    z.object({type: z.literal('bus'), id: z.tuple([BusId, z.number().int()])}),
]);
export type OutputId = z.infer<typeof OutputId>;

const inputs = () => z.array(z.array(z.lazy(() => InputSpec)));

export const InsertSpec = z.object({
    instanceId: z.string(),
    inputs: inputs(),
});
export type InsertSpec = z.infer<typeof InsertSpec>;

export const BusSpec = z.object({
    midSideMode: z.optional(z.lazy(() => MidSideMode)),
    inputs: inputs()
});
export type BusSpec = z.infer<typeof BusSpec>;

export const MidSideMode = z.enum(["encodeToMidSide", "decodeToLeftRight"]);
export type MidSideMode = z.infer<typeof MidSideMode>;

export const NodeId = z.discriminatedUnion('type', [
    z.object({type: z.literal('source'), id: SourceId}),
    z.object({type: z.literal('insert'), id: InsertId}),
    z.object({type: z.literal('bus'), id: BusId}),
    z.object({type: z.literal('sink'), id: SinkId})
]);
export type NodeId = z.infer<typeof NodeId>;

export const AudioGraphModification = z.discriminatedUnion('type', [
    z.object({type: z.literal('addSource'), sourceId: SourceId, sourceSpec: z.lazy(() => SourceSpec)}),
    z.object({type: z.literal('addInsert'), insertId: InsertId, insertSpec: z.lazy(() => InsertSpec)}),
    z.object({type: z.literal('addBus'), busId: BusId, busSpec: z.lazy(() => BusSpec)}),
    z.object({type: z.literal('removeSource'), sourceId: SourceId}),
    z.object({type: z.literal('removeInsert'), insertId: InsertId}),
    z.object({type: z.literal('removeBus'), busId: BusId}),
    z.object({type: z.literal('connect'), component: z.lazy(() => NodeId), inputChannel: z.number().int(), output: z.lazy(() => OutputId)}),
    z.object({type: z.literal('disconnect'), component: z.lazy(() => NodeId), inputChannel: z.number().int(), output: z.lazy(() => OutputId)}),
]);
export type AudioGraphModification = z.infer<typeof AudioGraphModification>;