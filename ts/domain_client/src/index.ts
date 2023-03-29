import Axios, {AxiosInstance} from "axios"
import {InstancesApi} from "./instances"
import {TasksApi} from "./tasks"

export class DomainApi {
    constructor(private readonly client: AxiosInstance) {
    }

    static create(url?: string, token?: string): DomainApi {
        const client = Axios.create({
            baseURL: url || "http://localhost:7202",
            timeout: 1000,
            headers: token ? {'authorization': `Bearer ${token}`} : {}
        })

        return new DomainApi(client)
    }

    setAuthorization(token: string) {
        this.client.defaults.headers.authorization = `Bearer ${token}`
    }

    instances(): InstancesApi {
        return new InstancesApi(this.client)
    }

    tasks(): TasksApi {
        return new TasksApi(this.client)
    }
}