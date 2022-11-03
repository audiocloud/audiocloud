/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

import ReconnectingWebSocket from 'reconnecting-websocket'
import { nanoid } from 'nanoid'

import { Socket, SocketsSupervisor, SocketState } from './supervisor'
import { DomainClientMessage } from '@audiocloud/domain-client'
import { pack, unpack } from 'msgpackr'

export class WebSocket implements Socket {
    private ws: ReconnectingWebSocket
    private url: string
    id = nanoid()
    score = 1

    constructor(base_url: string, private readonly supervisor: SocketsSupervisor) {
        const ws_url = base_url.replace(/http/, 'ws')

        this.url = `${ws_url}/ws/${supervisor.client_id}/${this.id}`
        this.ws = new ReconnectingWebSocket(this.url)
        this.ws.binaryType = 'arraybuffer'
        this.ws.onmessage = this.on_message.bind(this)
        this.ws.onopen = this.on_open.bind(this)
        this.ws.onclose = this.on_close.bind(this)
    }

    private on_message(event: MessageEvent) {
        this.supervisor.on_message(this.id, unpack(new Uint8Array(event.data)))
    }

    private on_open() {
        this.supervisor.on_open(this.id)
    }

    private on_close() {
        this.supervisor.on_close(this.id)
    }

    get_state(): SocketState {
        if (this.ws.readyState == ReconnectingWebSocket.OPEN) {
            return 'open'
        } else {
            return 'closed'
        }
    }

    send(event: DomainClientMessage): void {
        this.ws.send(pack(event))
    }
}