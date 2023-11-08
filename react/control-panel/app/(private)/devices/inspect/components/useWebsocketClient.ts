import { useState, useEffect, useMemo } from 'react'
import { useCookies } from 'react-cookie'
import { addSeconds } from 'date-fns'
import { createWebSocketClient } from '@/services/domainClient'
import { DeviceReportsType } from '@/types'
import { InstancePlayState, InstancePowerState, InstanceSpec } from '@/services/domainClient/types'
import { ReceiveEvents } from '@/services/domainClient/shared_socket'
import { defaultReports } from './LiveControl/Faceplates/data/defaultReports'

export const useWebsocketClient = (ip: string, device_id: string | null) => {

  const [powerState, setPowerState] = useState<InstancePowerState | 'unknown'>('unknown')
  const [playState, setPlayState] = useState<InstancePlayState | 'unknown'>('unknown')
  const [reports, setReports] = useState<DeviceReportsType>({})

  const [connectionStatus, setConnectionStatus] = useState(false)
  const [cookies, setCookie, removeCookie] = useCookies(['token'])

  // Websocket Handler Functions

  const handler: ReceiveEvents = {
    connectionChanged: (connected: boolean) => {
      console.log('WS connection:', connected)
      setConnectionStatus(connected)
    },
    instanceReport: (device: string, name: string, channel: number, value: number): void => {
      // device is irrelevant for ac-dcp, because we never have more than 1 device in rack
      console.log('Reporting:', device, name, channel, value)
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
    if (connectionStatus && device_id) ws.sendEvents.subscribeToInstanceEvents(device_id)
    return () => {
      if (connectionStatus && device_id) ws.sendEvents.unsubscribeFromInstanceEvents(device_id)
    }
  }, [connectionStatus])

  // In theory, this is a valid cleanup for the webhook connection in the useMemo
  useEffect(() => {
    return () => {
      ws.sendEvents.close()
    }
  }, [ws])

  useEffect(() => {
    if (connectionStatus && device_id) {
      setPowerState('unknown')
      setPlayState('unknown')
      setReports({...defaultReports[device_id]})
    }
  }, [connectionStatus])

  const handlePower = (newPowerState: boolean) => {
    if (device_id) {
      if (newPowerState)  setPowerState('on')
      else                setPowerState('off')
  
      console.log(`Setting ${device_id} power to: ${newPowerState}`)
      
      ws.sendEvents.setInstancePowerControl(
        device_id,
        {
          desired: newPowerState ? 'on' : 'off',
          until: addSeconds(new Date(), 10)
        }
      )
    }
  }

  const handlePlay = (newPlayState: boolean) => {
    if (device_id) {
      if (newPlayState) setPlayState({ playing: { duration: 10, play_id: 123 }})
      else              setPlayState('idle')
  
      console.log(`Setting ${device_id} play to: ${newPlayState}`)
  
      ws.sendEvents.setInstancePlayControl(
        device_id,
        {
          desired: newPlayState ? { play: { duration: 10, play_id: 123 }} : 'stop',
          until: addSeconds(new Date(), 10)
        }
      )
    }
  }

  return { connectionStatus, ws, powerState, handlePower, playState, handlePlay, reports }
}