// noinspection JSUnusedGlobalSymbols

import {z} from "zod";
import {AxiosInstance} from "axios";

export const InstanceSummary = z.object({
    id: z.string(),
    modelId: z.string(),
    driverId: z.string(),
    powerState: z.lazy(() => InstancePowerStateSummary),
    playState: z.lazy(() => InstancePlayStateSummary),
});
export type InstanceSummary = z.infer<typeof InstanceSummary>;

export const InstancePlayStateSummary = z.object({
    changedAt: z.string().datetime(),
    state: z.lazy(() => InstancePlayState),
})
export type InstancePlayStateSummary = z.infer<typeof InstancePlayStateSummary>;

export const InstancePowerStateSummary = z.object({
    changedAt: z.string().datetime(),
    state: z.lazy(() => InstancePowerState),
})
export type InstancePowerStateSummary = z.infer<typeof InstancePowerStateSummary>;

export const InstancePlayState = z.enum(["playing", "rewinding", "stopped"]);
export type InstancePlayState = z.infer<typeof InstancePlayState>;

export const InstancePowerState = z.enum(["on", "off", "coolingDown", "warmingUp"]);
export type InstancePowerState = z.infer<typeof InstancePowerState>;

export const RegisterInstanceRequest = z.object({
    id: z.string(),
    modelId: z.string(),
    driverId: z.string(),
})
export type RegisterInstanceRequest = z.infer<typeof RegisterInstanceRequest>;

export const UpdateInstanceRequest = z.discriminatedUnion('type', [
    z.object({'type': z.literal('powerState'), 'update': z.lazy(() => InstancePowerState)}),
    z.object({'type': z.literal('playState'), 'update': z.lazy(() => InstancePlayState)}),
    z.object({'type': z.literal('pushReports'), 'update': z.lazy(() => PushInstanceReports)}),
]);
export type UpdateInstanceRequest = z.infer<typeof UpdateInstanceRequest>;

export const PushInstanceReports = z.object({
    reportId: z.string(),
    startAt: z.string().datetime(),
    values: z.array(z.tuple([z.number(), z.number()]))
})
export type PushInstanceReports = z.infer<typeof PushInstanceReports>;

export class InstancesApi {
    constructor(private client: AxiosInstance) {
    }

    async getInstances(): Promise<InstanceSummary[]> {
        return getInstances(this.client);
    }

    async getInstance(id: string): Promise<InstanceSummary> {
        return getInstance(this.client, id);
    }

    async registerInstance(instance: RegisterInstanceRequest): Promise<void> {
        return registerInstance(this.client, instance);
    }

    async updateInstance(id: string, update: Array<UpdateInstanceRequest>): Promise<void> {
        return updateInstance(this.client, id, update);
    }
}

export function getInstances(client: AxiosInstance): Promise<InstanceSummary[]> {
    return client.get('/instances').then(response => response.data);
}

export function getInstance(client: AxiosInstance, id: string): Promise<InstanceSummary> {
    return client.get(`/instances/${id}`).then(response => response.data);
}

export function registerInstance(client: AxiosInstance, instance: RegisterInstanceRequest): Promise<void> {
    return client.post('/instances', instance);
}

export function updateInstance(client: AxiosInstance, id: string, update: Array<UpdateInstanceRequest>): Promise<void> {
    return client.post(`/instances/${id}/update`, update);
}