'use client'

import React, { useState } from 'react'
import { PlusCircleIcon, ArrowPathIcon, ExclamationTriangleIcon } from '@heroicons/react/20/solid'
import { Button } from '@/components/ui/button'
import NewMaintenanceModal from '../../shared/Modals/NewMaintenance/NewMaintenanceModal'
import ForceRestartAudioEngineModal from '../../shared/Modals/ForceRestartAudioEngineModal'
import ForceShutdownAudioEngineModal from '../../shared/Modals/ForceShutdownAudioEngineModal'

type Props = {
  engine_id: string | undefined
}

const AudioEngineActionsBar: React.FC<Props> = ({ engine_id }) => {

  if (!engine_id) return undefined

  const [newMaintenanceOpen, setNewMaintenanceOpen] = useState(false)
  const [forceRestartOpen, setForceRestartOpen] = useState(false)
  const [forceShutdownOpen, setForceShutdownOpen] = useState(false)

  // TO-DO: implement action response status

  return (
    <div className='w-full px-4 py-3 flex justify-start items-center gap-2 bg-midground/90 border-b'>

      <NewMaintenanceModal            engine_id={engine_id} isOpen={newMaintenanceOpen} setOpen={setNewMaintenanceOpen} />
      <ForceRestartAudioEngineModal   engine_id={engine_id} isOpen={forceRestartOpen} setOpen={setForceRestartOpen} />
      <ForceShutdownAudioEngineModal  engine_id={engine_id} isOpen={forceShutdownOpen} setOpen={setForceShutdownOpen} />

      <Button
        variant='objectActionButton'
        onClick={() => setNewMaintenanceOpen(true)}
      >
        <PlusCircleIcon className="h-4 w-4 mr-2" aria-hidden="false" />
        <span>New Maintenance</span>
      </Button>

      <Button
        variant='objectActionButton'
        onClick={() => setForceRestartOpen(true)}
      >
        <ArrowPathIcon className="h-4 w-4 mr-2" aria-hidden="false" />
        <span>Force Restart</span>
      </Button>

      <Button
        variant='objectActionButton'
        onClick={() => setForceShutdownOpen(true)}
      >
        <ExclamationTriangleIcon className="h-4 w-4 mr-2" aria-hidden="false" />
        <span>Force Shutdown</span>
      </Button>

    </div>
  )
}

export default AudioEngineActionsBar