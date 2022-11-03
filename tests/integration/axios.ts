/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

import { Request, Requester, Result } from '@audiocloud/domain-client'
import Axios, { AxiosInstance, AxiosRequestConfig } from 'axios'

export class AxiosRequester implements Requester {
    axios: AxiosInstance

    constructor(defaults?: AxiosRequestConfig) {
        this.axios = Axios.create(defaults)
    }

    async request<B, T, E>(request: Request<B>): Promise<Result<T, E>> {
        const axios_request: AxiosRequestConfig = {
            url: request.path,
            headers: request.headers || {},
            method: request.method,
        }

        if ('body' in request) {
            axios_request.data = request.body
            axios_request.headers!['content-type'] = 'application/json'
        }

        const res = await this.axios.request(axios_request).catch(({ response }) => response)
        if (res.status >= 200 && res.status < 300) {
            return Promise.resolve({ ok: res.data, is_error: false, is_ok: true } as Result<T, E>)
        } else {
            return Promise.reject({ error: res.data, is_ok: false, is_error: true } as Result<T, E>)
        }
    }
}
