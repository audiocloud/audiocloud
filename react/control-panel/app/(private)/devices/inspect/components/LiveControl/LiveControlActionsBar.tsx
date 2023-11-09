'use client'

import React, { useState } from 'react'
import { SignalIcon, SignalSlashIcon, PlayIcon, PauseIcon, BackwardIcon } from '@heroicons/react/20/solid'
import { Button } from '@/components/ui/button'
import { InstancePlayState, InstancePowerState } from '@/services/domainClient/types'

type Props = {
  device_id: string,
  powerState: InstancePowerState | 'unknown',
  liveControlEnabled: boolean,
  setLiveControlEnabled: React.Dispatch<React.SetStateAction<boolean>>,
  playState: InstancePlayState | 'unknown',
  handlePlay: (value: boolean) => void
}

const DeviceActionsBar: React.FC<Props> = ({ device_id, powerState, liveControlEnabled, setLiveControlEnabled, playState, handlePlay }) => {

  // TO-DO: implement a modal to confirm starting live control
  const [confirmLiveControlModal, setConfirmLiveControlModal] = useState(false)

  // TO-DO: implement action response status
  
  return (
    <div className='w-full px-4 py-3 flex justify-start items-center gap-2 bg-midground/90 border-b'>
    
      { liveControlEnabled ? (
        <Button
          variant='objectActionButton'
          onClick={() => setLiveControlEnabled(false)}
        >
          <SignalSlashIcon className="h-4 w-4 mr-2" aria-hidden="false" />
          <span>Disable live control</span>
        </Button>
      ) : (
        <Button
          variant='objectActionButton'
          onClick={() => setLiveControlEnabled(true)}
          disabled={powerState !== 'on'}
        >
          <SignalIcon className="h-4 w-4 mr-2" aria-hidden="false" />
          <span>Enable live control</span>
        </Button>
      )}

      { playState ? (
        <Button
          variant='objectActionButton'
          onClick={() => alert('Pause!')}
          disabled={powerState !== 'on' || !liveControlEnabled}
        >
          <PauseIcon className="h-4 w-4 mr-2" aria-hidden="false" />
          <span>Pause</span>
        </Button>
      ) : (
        <Button
          variant='objectActionButton'
          onClick={() => alert('Play!')}
          disabled={powerState !== 'on' || !liveControlEnabled}
        >
          <PlayIcon className="h-4 w-4 mr-2" aria-hidden="false" />
          <span>Play</span>
        </Button>
      )}

      {/* TO-DO: show only for tape machines */}
      { device_id !== '' && (
        <Button
          variant='objectActionButton'
          onClick={() => alert('Rewind!')}
          disabled={powerState !== 'on' || !liveControlEnabled}
        >
          <PlayIcon className="h-4 w-4 mr-2" aria-hidden="false" />
          <span>Rewind</span>
        </Button>
      )}
        
    </div>
  )
}

export default DeviceActionsBar