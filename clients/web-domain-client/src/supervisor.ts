/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

import { DomainClientMessage, DomainServerMessage } from '@audiocloud/domain-client'
import { nanoid } from 'nanoid'

import { WebSocket } from './web_socket'
import { WebRtcSocket } from './web_rtc'

type ResponseHandlerRegistration = {
    at: Date
    resolve: (msg: DomainServerMessage) => boolean
    reject: (err: Error) => void
}
type QueuedMessage = { at: Date; message: DomainClientMessage; socket_id: string | null }

export class SocketsSupervisor {
    client_id = nanoid()
    private sockets = new Map<string, Socket>()
    private any_open = false
    private response_handlers: Array<ResponseHandlerRegistration> = []
    private message_queue: Array<QueuedMessage> = []

    constructor(private readonly base_url: string, private readonly events: SocketSupervisorEvents) {
        this.add_web_socket()
        setInterval(this.on_update.bind(this), 250)
    }

    on_update() {
        for (const [key, value] of this.sockets.entries()) {
            if (value.get_state() == 'dead') {
                console.log('dropping dead socket', key)
                this.sockets.delete(key)
            }
        }
    }

    get num_web_rtc_sockets() {
        let count = 0
        for (const socket of this.sockets.values()) {
            if (socket instanceof WebRtcSocket) {
                count += 1
            }
        }

        return count
    }

    emit(message: DomainClientMessage, socket_id?: string) {
        this.message_queue.push({ at: new Date(), message, socket_id: socket_id || null })
        this.flush_messages()
    }

    on_open(socket_id: string) {
        console.log('on_open', socket_id)
        this.refresh_overall_state()
    }

    on_close(socket_id: string) {
        console.log('on_close', socket_id)
        this.refresh_overall_state()
    }

    on_message(socket_id: string, message: DomainServerMessage) {
        console.log('on_message', socket_id, message)
        if ('ping' in message) {
            this.emit({ pong: { challenge: message.ping.challenge, response: '' } }, socket_id)
        } else if ('submit_peer_connection_candidate' in message) {
            const socket = this.sockets.get(message.submit_peer_connection_candidate.socket_id)
            if (socket && socket instanceof WebRtcSocket) {
                socket.sumbit_remote_candidate(message.submit_peer_connection_candidate.candidate || null)
            }
        } else {
            this.response_handlers = this.response_handlers.filter(({ at: registered_at, resolve, reject }) => {
                if (new Date(registered_at.valueOf() + 15000) <= new Date()) {
                    reject(new Error('timeout'))
                    return false
                } else {
                    return !resolve(message)
                }
            })
        }
    }

    add_web_socket() {
        const socket = new WebSocket(this.base_url, this)
        this.sockets.set(socket.id, socket)
    }

    add_web_rtc_socket() {
        WebRtcSocket.new(this)
    }

    request(
        message: DomainClientMessage,
        resolve: (msg: DomainServerMessage) => boolean,
        reject: (err: Error) => void
    ) {
        this.response_handlers.push({ at: new Date(), resolve, reject })
        this.emit(message)
    }

    std_request<R>(message: DomainClientMessage, flat_map: (msg: DomainServerMessage) => R | null) {
        return new Promise<R>((resolve, reject) => {
            this.request(
                message,
                (msg) => {
                    const mapped = flat_map(msg)
                    if (mapped === null) {
                        return false
                    } else {
                        resolve(mapped)
                        return true
                    }
                },
                reject
            )
        })
    }

    request_peer_connection() {
        const request_id = nanoid()
        return this.std_request({ request_peer_connection: { request_id } }, (msg) => {
            if ('peer_connection_response' in msg && msg.peer_connection_response.request_id == request_id) {
                return msg.peer_connection_response.result
            } else {
                return null
            }
        })
    }

    request_answer_peer_connection(socket_id: string, answer: string) {
        const request_id = nanoid()
        return this.std_request({ answer_peer_connection: { request_id, socket_id, answer } }, (msg) => {
            if (
                'answer_peer_connection_response' in msg &&
                msg.answer_peer_connection_response.request_id == request_id
            ) {
                return msg.answer_peer_connection_response.result
            } else {
                return null
            }
        })
    }

    request_submit_peer_candidate(socket_id: string, candidate: string | null) {
        const request_id = nanoid()
        return this.std_request({ submit_peer_connection_candidate: { request_id, socket_id, candidate } }, (msg) => {
            if (
                'peer_connection_candidate_response' in msg &&
                msg.peer_connection_candidate_response.request_id == request_id
            ) {
                return msg.peer_connection_candidate_response.result
            } else {
                return null
            }
        })
    }

    register_web_rtc_socket(socket: WebRtcSocket) {
        this.sockets.set(socket.id, socket)
    }

    private refresh_overall_state() {
        let any_open = false
        for (const socket of this.sockets.values()) {
            any_open = any_open || socket.get_state() == 'open'
        }

        const is_change = this.any_open != any_open
        this.any_open = any_open

        if (is_change) {
            if (any_open) {
                this.events.on_open()
            } else {
                this.events.on_close()
            }
        }
    }

    private get_best_open_socket(): Socket | null {
        const open_sockets = []
        for (const socket of this.sockets.values()) {
            if (socket.get_state() == 'open') {
                open_sockets.push(socket)
            }
        }

        if (!open_sockets.length) {
            return null
        }

        open_sockets.sort((a, b) => b.score - a.score)

        return open_sockets[0]
    }

    private flush_messages() {
        if (this.any_open) {
            this.message_queue = this.message_queue.filter(({ at: queued_at, message, socket_id }) => {
                if (new Date(queued_at.valueOf() + 15000) <= new Date()) {
                    return false
                } else {
                    const socket = socket_id ? this.sockets.get(socket_id) : this.get_best_open_socket()
                    if (socket) {
                        socket.send(message)
                        return false
                    }
                    return true
                }
            })
        }
    }
}

export interface SocketSupervisorEvents {
    on_open(): void

    on_close(): void
}

export interface Socket {
    id: string

    score: number

    get_state(): SocketState

    send(event: DomainClientMessage): void
}

export type SocketState = 'open' | 'closed' | 'dead'
