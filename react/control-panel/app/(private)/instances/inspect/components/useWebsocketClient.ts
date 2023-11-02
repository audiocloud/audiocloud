import { useState, useEffect, useMemo } from "react"
import { createWebSocketClient } from "@/utils/domainClient"
import { InstanceReports } from "@/types"
import { InstancePlayState, InstancePowerState, InstanceSpec } from "@/utils/domainClient/types"
import { ReceiveEvents } from "@/utils/domainClient/shared_socket"
import { useCookies } from "react-cookie"

export const useWebsocketClient = (
  ip: string,
  instance_id: string,
  setPowerState: React.Dispatch<React.SetStateAction<InstancePowerState | 'unknown'>>,
  setPlayState: React.Dispatch<React.SetStateAction<InstancePlayState | 'unknown'>>,
  setReports: React.Dispatch<React.SetStateAction<InstanceReports>>
) => {

  const [connectionStatus, setConnectionStatus] = useState(false)
  const [cookies, setCookie, removeCookie] = useCookies(['token'])

  // Websocket Handler Functions

  const handler: ReceiveEvents = {
    connectionChanged: (connected: boolean) => {
      console.log('WS connection:', connected)
      setConnectionStatus(connected)
    },
    instanceReport: (instance: string, name: string, channel: number, value: number): void => {
      // instance is irrelevant for ac-dcp, because we never have more than 1 instance in rack
      console.log('Reporting:', instance, name, channel, value)
      setReports(prevState => {
        const reportedParameter = prevState[name] || []
        const updatedParameter = [...reportedParameter]
        updatedParameter[channel] = value
        return { ...prevState, [name]: updatedParameter }
      })
    },
    instanceSpec: (instanceId: string, spec: InstanceSpec | null): void => {
      console.log('Instance spec:', spec)
    },
    instanceConnectionChanged: (instanceId: string, connected: boolean): void => {
      console.log('Instance connection changed. Status:', connected)
      // ???
    },
    instancePlayStateChanged: (instanceId: string, state: InstancePlayState): void => {
      console.log('Instance play state changed. Status:', state)
      setPlayState(state)
    },
    instancePowerStateChanged: (instanceId: string, state: InstancePowerState): void => {
      console.log('Instance power state changed. Status:', state)
      setPowerState(state)
    },
  }

  const ws = useMemo(() => {
    const [sendEvents, sendRequest] = createWebSocketClient(`http://${ip}:7200/event?token=${cookies.token}`, handler)
    return { sendEvents, sendRequest }
  }, [ip])

  useEffect(() => {
    if (connectionStatus) ws.sendEvents.subscribeToInstanceEvents(instance_id)
    return () => {
      if (connectionStatus) ws.sendEvents.unsubscribeFromInstanceEvents(instance_id)
    }
  }, [connectionStatus])

  // In theory, this is a valid cleanup for the webhook connection in the useMemo
  useEffect(() => {
    return () => {
      ws.sendEvents.close()
    }
  }, [ws])

  return { connectionStatus, ws }
}