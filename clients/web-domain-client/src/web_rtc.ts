import { Socket, SocketsSupervisor, SocketState } from './supervisor'
import { DomainClientMessage } from '@audiocloud/domain-client'
import { pack, unpack } from 'msgpackr'

const iceServers = [{ urls: 'stun:stun.l.google.com:19302' }]

export class WebRtcSocket implements Socket {
    private data_channel: RTCDataChannel | null = null
    score = 2

    constructor(
        public id: string,
        private readonly peer_connection: RTCPeerConnection,
        private readonly supervisor: SocketsSupervisor
    ) {
        peer_connection.ondatachannel = this.on_data_channel.bind(this)
    }

    private on_data_channel(event: RTCDataChannelEvent) {
        console.log('RTC', this.id, 'new data_channel', event.channel.label)
        this.data_channel = event.channel
        this.data_channel.onmessage = this.on_data_message.bind(this)
    }

    private on_data_message(message: MessageEvent) {
        this.supervisor.on_message(this.id, unpack(new Uint8Array(message.data)))
    }

    sumbit_remote_candidate(candidate: string | null) {
        this.peer_connection.addIceCandidate(candidate ? JSON.parse(candidate) : undefined)
    }

    get_state(): SocketState {
        if (this.data_channel && this.data_channel.readyState == 'open') {
            return 'open'
        } else if (
            this.peer_connection.connectionState == 'failed' ||
            this.peer_connection.connectionState == 'disconnected'
        ) {
            return 'dead'
        } else {
            return 'closed'
        }
    }

    send(event: DomainClientMessage): void {
        if (this.data_channel) {
            this.data_channel.send(pack(event))
        }
    }

    static async new(supervisor: SocketsSupervisor) {
        try {
            const peer_connection = new RTCPeerConnection({ iceServers })
            const remote = await supervisor.request_peer_connection()

            console.log('got remote', { remote })

            if ('error' in remote) {
                throw new Error(remote.error.type)
            }

            let remote_sdp = JSON.parse(remote.ok.created.remote_description)
            let socket_id = remote.ok.created.socket_id.socket_id

            peer_connection.onicecandidate = (event) => {
                supervisor
                    .request_submit_peer_candidate(socket_id, event.candidate ? JSON.stringify(event.candidate) : null)
                    .catch((err) => {
                        console.error('RTC', socket_id, 'received error submitting peer', err)
                    })
            }
            peer_connection.setRemoteDescription(remote_sdp)

            const answer_sdp = await peer_connection.createAnswer(remote_sdp)

            console.log('created answer', { answer_sdp })

            await supervisor.request_answer_peer_connection(socket_id, JSON.stringify(answer_sdp))

            peer_connection.setLocalDescription(answer_sdp)

            supervisor.register_web_rtc_socket(new WebRtcSocket(socket_id, peer_connection, supervisor))
        } finally {
            console.log('WebRtc constructor ended')
        }
    }
}
