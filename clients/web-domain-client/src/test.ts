import { SocketsSupervisor } from './supervisor'

const socketSupervisor = new SocketsSupervisor('http://localhost:7200', {
    on_open() {
        let num_web_rtc_sockets = socketSupervisor.num_web_rtc_sockets
        console.log('opened', { num_web_rtc_sockets })
        if (!num_web_rtc_sockets) {
            socketSupervisor.add_web_rtc_socket()
        }
    },
    on_close() {
        console.log('closed')
    },
})