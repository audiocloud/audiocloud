'use client'

import React, { useState } from 'react'
import { PlusCircleIcon, ArrowPathIcon, ExclamationTriangleIcon } from '@heroicons/react/20/solid'
import { Button } from '@/components/ui/button'
import NewMaintenance from '../../shared/Modals/NewMaintenance/NewMaintenanceModal'

type Props = {
  engine_id: string | undefined
}

const AudioEngineActionsBar: React.FC<Props> = ({ engine_id }) => {

  if (!engine_id) return undefined

  const [newMaintenance, setNewMaintenance] = useState(false)
  // const [restartModal, setRestartModal] = useState(false) // TO-DO: implement
  // const [shutdownModal, setShutdownModal] = useState(false) // TO-DO: implement

  // TO-DO: implement action response status

  return (
    <div className='w-full px-4 py-3 flex justify-start items-center gap-2 bg-slate-900/70 border-b'>

      <Button
        variant='objectActionButton'
        onClick={() => setNewMaintenance(true)}
      >
        <PlusCircleIcon className="h-4 w-4 mr-2" aria-hidden="false" />
        <span>New Maintenance</span>
      </Button>

      <Button
        variant='objectActionButton'
        onClick={() => alert('Force restart!')}
      >
        <ArrowPathIcon className="h-4 w-4 mr-2" aria-hidden="false" />
        <span>Force Restart</span>
      </Button>

      <Button
        variant='objectActionButton'
        onClick={() => alert('Force shutdown!')}
      >
        <ExclamationTriangleIcon className="h-4 w-4 mr-2" aria-hidden="false" />
        <span>Force Shutdown</span>
      </Button>

      <NewMaintenance engine_id={engine_id} open={newMaintenance} setOpen={setNewMaintenance} />
      {/* <RestartModal engine_id={engine_id} open={restartModal} setOpen={setRestartModal} />
      <ShutdownModal engine_id={engine_id} open={shutdownModal} setOpen={setShutdownModal} /> */}

    </div>
  )
}

export default AudioEngineActionsBar