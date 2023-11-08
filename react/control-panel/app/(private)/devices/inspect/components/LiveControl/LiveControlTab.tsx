'use client'

import React, { useState } from 'react'
import { InstancePlayState, InstancePowerState } from '@/services/domainClient/types'
import LiveControlActionsBar from './LiveControlActionsBar'
import LiveControlPanel from './LiveControlPanel'
import ObjectNotFoundWarning from '@/components/general/ObjectNotFoundWarning'
import { DeviceReportsType } from '@/types'
import { ExclamationTriangleIcon } from '@heroicons/react/24/outline'

type Props = {
  device_id: string | undefined,
  connectionStatus: boolean,
  powerState: InstancePowerState | 'unknown',
  handlePower: (value: boolean) => void,
  playState: InstancePlayState | 'unknown',
  handlePlay: (value: boolean) => void,
  reports: DeviceReportsType
}

const LiveControlTab: React.FC<Props> = ({ device_id, connectionStatus, powerState, handlePower, playState, handlePlay, reports }) => {

  if (!device_id) return <div className='p-4 flex'><ObjectNotFoundWarning objectName='Instance' /></div>
  if (!connectionStatus) {
    return (
      <div className='p-4 flex justify-start items-center gap-3 text-lg'>
        <ExclamationTriangleIcon className='w-8 h-8' aria-hidden='false'/>
        <span>Unable to establish domain websocket connection.</span>
      </div>
  )}

  const [liveControlEnabled, setLiveControlEnabled] = useState(false)
  
  // TO-DO: replace with real play state

  return (
    <div>
      <LiveControlActionsBar
        device_id={device_id}
        powerState={powerState}
        liveControlEnabled={liveControlEnabled}
        setLiveControlEnabled={setLiveControlEnabled}
        playState={playState}
        handlePlay={handlePlay}
      />
      <LiveControlPanel
        device_id={device_id}
        liveControlEnabled={liveControlEnabled}
        playState={playState}
        reports={reports}
      />
    </div>
  )
}

export default LiveControlTab