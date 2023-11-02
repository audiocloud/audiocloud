'use client'

import React, { useState } from 'react'
import Link from 'next/link'
import { ChevronDownIcon } from '@heroicons/react/20/solid'
import { IInstance } from '@/types'
import NewMaintenance from '../../../shared/Modals/NewMaintenance'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuShortcut, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'

type Props = {
  instance: IInstance
}

const InstanceActions: React.FC<Props> = ({ instance }) => {

  const [newMaintenanceOpen, setNewMaintenanceOpen] = useState(false)
  
  return (
    <>
      <NewMaintenance instance_id={instance.id} open={newMaintenanceOpen} setOpen={setNewMaintenanceOpen} />

      <DropdownMenu>
        <DropdownMenuTrigger className='p-1 bg-background hover:bg-secondary hover:text-white border hover:border-slate-600 rounded-md'>
          <ChevronDownIcon className='h-6 w-6' aria-hidden="false" />
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuLabel>Actions</DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuItem asChild><Link href={`/instances/inspect?instance_id=${instance.id}`}>Inspect</Link></DropdownMenuItem>
          <DropdownMenuItem onClick={() => setNewMaintenanceOpen(true)}>Schedule Maintenance</DropdownMenuItem>
          <DropdownMenuItem onClick={() => alert('Force play!')}>Force Play</DropdownMenuItem>
          <DropdownMenuItem onClick={() => alert('Force stop!')}>Force Stop</DropdownMenuItem>
          <DropdownMenuItem onClick={() => alert('Force rewind!')}>Force Rewind</DropdownMenuItem>
          <DropdownMenuItem onClick={() => alert('Restart Driver!')}>Restart Driver</DropdownMenuItem>
          <DropdownMenuItem onClick={() => alert('Power On!')}>Power On</DropdownMenuItem>
          <DropdownMenuItem onClick={() => alert('Shut Down!')}>Shut Down</DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </>
  )
}

export default InstanceActions