'use client'

import React, { useState } from 'react'
import Link from 'next/link'
import { ChevronDownIcon } from '@heroicons/react/20/solid'
import { IAudioEngine } from '@/types'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'
import NewMaintenance from '../../../shared/Modals/NewMaintenance/NewMaintenanceModal'
import ForceRestartAudioEngineModal from '../../../shared/Modals/ForceRestartAudioEngineModal'
import ForceShutdownAudioEngineModal from '../../../shared/Modals/ForceShutdownAudioEngineModal'

type Props = {
  audio_engine: IAudioEngine
}

const AudioEngineActions: React.FC<Props> = ({ audio_engine }) => {

  const [newMaintenanceOpen, setNewMaintenanceOpen] = useState(false)
  const [forceRestartOpen, setForceRestartOpen] = useState(false)
  const [forceShutdownOpen, setForceShutdownOpen] = useState(false)

  return (
    <>
      <NewMaintenance engine_id={audio_engine.id} isOpen={newMaintenanceOpen} setOpen={setNewMaintenanceOpen} />
      <ForceRestartAudioEngineModal engine_id={audio_engine.id} isOpen={forceRestartOpen} setOpen={setForceRestartOpen} />
      <ForceShutdownAudioEngineModal engine_id={audio_engine.id} isOpen={forceShutdownOpen} setOpen={setForceShutdownOpen} />
      
      <DropdownMenu>
        <DropdownMenuTrigger className='p-1 bg-background hover:bg-secondary hover:text-white border hover:border-slate-600 rounded-md'>
          <ChevronDownIcon className='h-6 w-6' aria-hidden="false" />
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuLabel>Actions</DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuItem asChild><Link href={`/audio-engines/inspect?engine_id=${audio_engine.id}`}>Inspect</Link></DropdownMenuItem>
          <DropdownMenuItem onClick={() => setNewMaintenanceOpen(true)}>Schedule Maintenance</DropdownMenuItem>
          <DropdownMenuItem onClick={() => setForceRestartOpen(true)}>Force Restart</DropdownMenuItem>
          <DropdownMenuItem onClick={() => setForceShutdownOpen(true)}>Force Shutdown</DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </>
  )
}

export default AudioEngineActions