'use client'

import React, { useState } from 'react'
import { InstancePlayState, InstancePowerState } from '@/services/domainClient/types'
import LiveControlActionsBar from './LiveControlActionsBar'
import LiveControlPanel from './LiveControlPanel'
import ObjectNotFoundWarning from '@/components/general/ObjectNotFoundWarning'
import { InstanceReportsType } from '@/types'
import { ExclamationTriangleIcon } from '@heroicons/react/24/outline'

type Props = {
  instance_id: string | undefined,
  connectionStatus: boolean,
  powerState: InstancePowerState | 'unknown',
  handlePower: (value: boolean) => void,
  playState: InstancePlayState | 'unknown',
  handlePlay: (value: boolean) => void,
  reports: InstanceReportsType
}

const LiveControlTab: React.FC<Props> = ({ instance_id, connectionStatus, powerState, handlePower, playState, handlePlay, reports }) => {

  if (!instance_id) return <div className='p-4 flex'><ObjectNotFoundWarning objectName='Instance' /></div>
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
        instance_id={instance_id}
        powerState={powerState}
        liveControlEnabled={liveControlEnabled}
        setLiveControlEnabled={setLiveControlEnabled}
        playState={playState}
        handlePlay={handlePlay}
      />
      <LiveControlPanel
        instance_id={instance_id}
        liveControlEnabled={liveControlEnabled}
        playState={playState}
        reports={reports}
      />
    </div>
  )
}

export default LiveControlTab