/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

import ReconnectingWebSocket from 'reconnecting-websocket';
import {pack, unpack} from 'msgpackr';
import {DomainClientMessage, DomainError, DomainServerMessage} from "@audiocloud/js-client/src/domain_api";
import {nanoid} from "nanoid";

const iceServers = [{urls: 'stun:stun.l.google.com:19302'}];

interface Request {
    reject(err: Error): void

    resolve(value: any): void
}

export class WebDomainClient {
    private readonly web_socket_url: string;
    private readonly message_queue: Array<DomainClientMessage> = []
    private web_socket: ReconnectingWebSocket
    private peer_connection: RTCPeerConnection | null = null
    private data_channel: RTCDataChannel | null = null
    private rtc_socket_id: string | null = null
    private rtc_ice_candidates: Array<string> = []
    private requests: Record<string, Request> = {}

    constructor(private readonly api_url: string, private web_rtc_enabled = false) {
        this.web_socket_url = api_url.replace(/http/, 'ws') + '/ws'
        console.log(this.web_socket_url)
        this.web_socket = new ReconnectingWebSocket(this.web_socket_url);
        this.web_socket.binaryType = 'arraybuffer'
        this.web_socket.onopen = this.on_open.bind(this)
        this.web_socket.onclose = this.on_close.bind(this)
        this.web_socket.onmessage = this.on_raw_message.bind(this)
        if (web_rtc_enabled) {
            setTimeout(this.update_web_rtc.bind(this), 1000)
        }
    }

    get connected() {
        const web_sockeet_connected = this.web_socket.readyState == ReconnectingWebSocket.OPEN
        let data_channel_connected = false
        if (this.data_channel) {
            data_channel_connected = this.data_channel.readyState == 'open'
        }

        return web_sockeet_connected || data_channel_connected
    }

    private on_open() {
        console.log('[web_socket]: open')
        this.flush_pending_messages()
    }

    private flush_pending_messages() {
        while (this.message_queue.length > 0 && this.connected) {
            const packed = pack(this.message_queue.shift())
            if (this.data_channel?.readyState == 'open') {
                this.data_channel.send(packed)
            } else {
                this.web_socket.send(packed)
            }
        }
    }

    private on_close() {
        console.log('[web_socket]: close')
    }

    private on_raw_message(data: MessageEvent) {
        let message: DomainServerMessage
        if (data.data instanceof ArrayBuffer) {
            message = unpack(new Uint8Array(data.data))
        } else {
            message = JSON.parse(data.data)
        }

        this.on_message(message)
    }

    private on_message(message: DomainServerMessage) {
        console.log('on_message', message)
        if ("ping" in message) {
            this.on_ping(message.ping.challenge)
        } else if ("peer_connection_response" in message) {
            const {peer_connection_response} = message
            if ("ok" in peer_connection_response.result) {
                const {socket_id, remote_description} = peer_connection_response.result.ok.created
                this.on_peer_connection_ok(remote_description, socket_id)
            } else {
                this.on_peer_connection_error(peer_connection_response.result.error)
            }
        } else if ("submit_peer_connection_candidate" in message) {
            this.on_peer_connection_candidate(message.submit_peer_connection_candidate.candidate)
        } else if ("answer_peer_connection_response" in message) {
            const {result} = message.answer_peer_connection_response
            if ("ok" in result) {
                this.on_peer_connection_answer_ok()
            } else {
                this.on_peer_connection_answer_error(result.error)
            }
        } else if ("submit_peer_connection_candidate" in message) {
            const {candidate} = message.submit_peer_connection_candidate
            this.on_peer_connection_candidate(candidate)
        }
    }

    private on_ping(challenge: string) {
        this.send_event({pong: {challenge, response: 'pingety ping pong'}})
    }

    private on_peer_connection_ok(remote_description: string, socket_id: string) {
        console.log('on_peer_connection_ok', remote_description, socket_id)
        this.rtc_socket_id = socket_id
        if (this.peer_connection) {
            this.peer_connection.setRemoteDescription(JSON.parse(remote_description))
            this.peer_connection.createAnswer().then((answer) => {
                if (this.peer_connection) {
                    this.peer_connection.setLocalDescription(answer)

                    console.log('answer', answer)

                    this.send_event({answer_peer_connection: {request_id: nanoid(), answer: JSON.stringify(answer), socket_id}})
                    this.flush_rtc_ice_candidates()
                }
            }).catch(err => {
                console.error('createAnswer', err)
            })
        }
    }

    private on_peer_connection_error(error: DomainError) {
        console.warn('on_peer_connection_error', error)
    }

    private on_peer_connection_candidate(candidate?: string | null) {
        console.log('on_peer_connection_candidate', candidate)
        if (this.peer_connection) {
            this.peer_connection.addIceCandidate(candidate && candidate.length ? JSON.parse(candidate) : null)
        }
    }

    private on_peer_connection_answer_ok() {
        console.log('on_peer_connection_answer_ok')
    }

    private on_peer_connection_answer_error(error: DomainError) {
        console.log('on_peer_connection_answer_error', error)
    }

    private send_event(message: DomainClientMessage) {
        this.message_queue.push(message)
        this.flush_pending_messages()
    }

    private update_web_rtc() {
        if (this.connected) {
            if (this.peer_connection) {
                const {connectionState} = this.peer_connection
                if (connectionState == 'failed' || connectionState == 'closed') {
                    this.peer_connection = null;
                }
            }

            if (!this.peer_connection && this.web_rtc_enabled) {
                this.data_channel = null
                this.peer_connection = new RTCPeerConnection({iceServers})
                this.peer_connection.onicecandidate = this.on_local_ice_candidate.bind(this)
                this.peer_connection.ondatachannel = this.on_data_channel_offered.bind(this)
                this.peer_connection.onconnectionstatechange = this.on_peer_connection_state_change.bind(this)

                this.request_peer_connection()
            }
        }
    }

    private request_peer_connection() {
        this.send_event({request_peer_connection: {request_id: nanoid()}})
    }

    private on_local_ice_candidate(event: RTCPeerConnectionIceEvent) {
        console.log('on_local_ice_candidate', event.candidate)
        const candidate = JSON.stringify(event.candidate)
        if (this.rtc_socket_id) {
            this.rtc_ice_candidates.push(candidate)
            this.flush_rtc_ice_candidates()
        }
    }

    private on_peer_connection_state_change() {
        if (this.peer_connection) {
            console.log('on_peer_connection_state_change', this.peer_connection.connectionState)
        }
    }

    private on_data_channel_offered(event: RTCDataChannelEvent) {
        console.log('on_data_channel_offered')
        this.data_channel = event.channel
        this.data_channel.onmessage = this.on_raw_message.bind(this)
        this.data_channel.onopen = this.on_data_channel_opened.bind(this)
        this.data_channel.onerror = this.on_data_channel_error.bind(this)
        this.data_channel.onclose = this.on_data_channel_closed.bind(this)
    }

    private on_data_channel_error(event: Event) {
        console.log('on_data_channel_error', event)
    }

    private on_data_channel_opened() {
        console.log('on_data_channel_opened')
    }

    private on_data_channel_closed(event: Event) {
        console.log('on_data_channel_closed', event)
    }

    private flush_rtc_ice_candidates() {
        while (this.connected && this.rtc_socket_id && this.rtc_ice_candidates.length > 0) {
            const candidate = this.rtc_ice_candidates.shift()!
            this.send_event({submit_peer_connection_candidate: {candidate, socket_id: this.rtc_socket_id, request_id: nanoid()}})
        }
    }
}

const client = new WebDomainClient('http://localhost:7200', true);
