'use client'

import React, { useState } from 'react'
import Link from 'next/link'
import { ChevronDownIcon } from '@heroicons/react/20/solid'
import { IAudioEngine } from '@/types'
import NewMaintenance from '../../../shared/Modals/NewMaintenance'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuShortcut, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'

type Props = {
  audio_engine: IAudioEngine
}

const AudioEngineActions: React.FC<Props> = ({ audio_engine }) => {

  const [newMaintenanceOpen, setNewMaintenanceOpen] = useState(false)

  return (
    <>
      <NewMaintenance engine_id={audio_engine.id} open={newMaintenanceOpen} setOpen={setNewMaintenanceOpen} />
      
      <DropdownMenu>
        <DropdownMenuTrigger className='p-1 bg-background hover:bg-secondary hover:text-white border hover:border-slate-600 rounded-md'>
          <ChevronDownIcon className='h-6 w-6' aria-hidden="false" />
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuLabel>Actions</DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuItem asChild><Link href={`/audio-engines/inspect?engine_id=${audio_engine.id}`}>Inspect</Link></DropdownMenuItem>
          <DropdownMenuItem onClick={() => setNewMaintenanceOpen(true)}>Schedule Maintenance</DropdownMenuItem>
          <DropdownMenuItem onClick={() => alert('Restart!')}>Force Restart</DropdownMenuItem>
          <DropdownMenuItem onClick={() => alert('Shutdown!')}>Force Shutdown</DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </>
  )
}

export default AudioEngineActions