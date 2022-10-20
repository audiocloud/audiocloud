import { Client, Requester } from '@audiocloud/domain-client'
import { AxiosRequester } from '../axios'
import { addMinutes } from 'date-fns'
import expect from 'expect'

describe('domain create tasks', () => {
    const domain_service_url = process.env.AUDIOCLOUD_DOMAIN_SERVER_URL || 'http://127.0.0.1:7200'
    const domain_service_token = process.env.AUDIOCLOUD_DOMAIN_SERVER_TOKEN || 'super.secret.token'

    const app_id = 'test'
    const task_id = '261e0533-9e02-4d22-b0a5-ca89355c7e9d'
    const secure_key = 'c0a33653-3198-435d-a785-b59553a489db'

    const client = new Client(
        <Requester>new AxiosRequester({
            baseURL: domain_service_url,
            headers: { Authorization: `Bearer ${domain_service_token}` },
        })
    )

    const from = new Date()
    const to = addMinutes(from, 10)

    const full_task_id = `${app_id}:${task_id}`

    it('create a minimal legal task', async () => {
        const { ok } = await client.create_task({
            task_id: full_task_id,
            spec: {},
            security: {
                [secure_key]: {
                    audio: true,
                    media: true,
                    parameters: true,
                    structure: true,
                    transport: true,
                },
            },
            reservations: {
                from: from.toISOString(),
                to: to.toISOString(),
                fixed_instances: [],
            },
        })

        expect(ok?.created).toEqual({ task_id: full_task_id })
    }, 1000)

    afterAll(async () => {
        await client.delete_task(app_id, task_id)
    })
})
