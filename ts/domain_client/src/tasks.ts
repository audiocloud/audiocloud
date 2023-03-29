// noinspection JSUnusedGlobalSymbols

import {z} from 'zod';
import {AxiosInstance} from "axios"
import {AudioGraphModification, AudioGraphSpec} from "./graph"

export const TaskSummary = z.object({
    id: z.string(),
    appId: z.string(),
    from: z.coerce.date(),
    to: z.coerce.date(),
    instances: z.record(z.string(), z.lazy(() => TaskInstanceAllocation)),
    playState: z.lazy(() => TaskPlayStateSummary)
});
export type TaskSummary = z.infer<typeof TaskSummary>;

export const TaskPlayStateSummary = z.object({
    updatedAt: z.coerce.date(),
    status: z.lazy(() => TaskPlayState)
});
export type TaskPlayStateSummary = z.infer<typeof TaskPlayStateSummary>;

export const TaskPlayState = z.discriminatedUnion('type', [
    z.object({type: z.literal('stopped')}),
    z.object({type: z.literal('waitingForInstances'), missing: z.array(z.string()), waiting: z.array(z.string())}),
    z.object({type: z.literal('waitingForFiles'), missing: z.array(z.string()), downloading: z.array(z.string())}),
    z.object({type: z.literal('buffering')}),
    z.object({type: z.literal('playing')}),
]);
export type TaskPlayState = z.infer<typeof TaskPlayState>;

export const TaskInstanceAllocation = z.object({
    request: z.lazy(() => TaskInstanceAllocationRequest),
    instanceId: z.string(),
});
export type TaskInstanceAllocation = z.infer<typeof TaskInstanceAllocation>;

export const TaskInstanceAllocationRequest = z.discriminatedUnion('type', [
    z.object({type: z.literal('fixed'), instanceId: z.string()}),
    z.object({type: z.literal('dynamic'), modelId: z.string()}),
]);
export type TaskInstanceAllocationRequest = z.infer<typeof TaskInstanceAllocationRequest>;

export const CreateTaskRequest = z.object({
    from: z.coerce.date(),
    to: z.coerce.date(),
    instances: z.record(z.string(), z.lazy(() => TaskInstanceAllocationRequest)),
});
export type CreateTaskRequest = z.infer<typeof CreateTaskRequest>;

export const SetTaskGraphRequest = AudioGraphSpec;
export type SetTaskGraphRequest = z.infer<typeof SetTaskGraphRequest>;

export const ModifyTaskGraphRequest = z.array(z.lazy(() => AudioGraphModification));
export type ModifyTaskGraphRequest = z.infer<typeof ModifyTaskGraphRequest>;

export const ModifyTaskResourcesRequest = z.object({
    from: z.coerce.date().optional(),
    to: z.coerce.date().optional(),
    addInstances: z.record(z.string(), z.lazy(() => TaskInstanceAllocation)).optional(),
    removeInstances: z.array(z.string()).optional()
});
export type ModifyTaskResourcesRequest = z.infer<typeof ModifyTaskResourcesRequest>;

export class TasksApi {
    constructor(private instance: AxiosInstance) {
    }

    async getTask(id: string): Promise<TaskSummary> {
        return getTaskSummary(this.instance, id);
    }

    async setTaskGraph(id: string, req: SetTaskGraphRequest): Promise<void> {
        return setTaskGraph(this.instance, id, req);
    }

    async modifyTaskGraph(id: string, modifications: ModifyTaskGraphRequest): Promise<void> {
        return modifyTaskGraph(this.instance, id, modifications);
    }

    async modifyTaskResources(id: string, modify: ModifyTaskResourcesRequest): Promise<void> {
        return modifyTaskResources(this.instance, id, modify);
    }
}

export function getTaskSummary(instance: AxiosInstance, id: string): Promise<TaskSummary> {
    return instance.get(`/task/${id}`).then(r => TaskSummary.parse(r.data));
}

export function createTask(instance: AxiosInstance, create: CreateTaskRequest): Promise<TaskSummary> {
    return instance.post('/task', create).then(r => TaskSummary.parse(r.data));
}

export function setTaskGraph(instance: AxiosInstance, id: string, spec: SetTaskGraphRequest): Promise<void> {
    return instance.put(`/task/${id}/graph`, spec);
}

export function modifyTaskGraph(instance: AxiosInstance, id: string, modify: ModifyTaskGraphRequest): Promise<void> {
    return instance.patch(`/task/${id}/graph`, modify);
}

export function modifyTaskResources(instance: AxiosInstance, id: string, modify: ModifyTaskResourcesRequest): Promise<void> {
    return instance.post(`/task/${id}/resources`, modify);
}