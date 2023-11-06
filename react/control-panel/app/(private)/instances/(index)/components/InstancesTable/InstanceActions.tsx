'use client'

import React, { useState } from 'react'
import Link from 'next/link'
import { ChevronDownIcon } from '@heroicons/react/20/solid'
import { IInstance } from '@/types'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'
import NewMaintenanceModal from '../../../shared/Modals/NewMaintenance/NewMaintenanceModal'
import RestartInstanceDriverModal from '../../../shared/Modals/RestartInstanceDriverModal'
import RestartInstanceModal from '../../../shared/Modals/RestartInstanceModal'
import ForceShutdownInstanceModal from '../../../shared/Modals/ForceShutdownInstanceModal'

type Props = {
  instance: IInstance
}

const InstanceActions: React.FC<Props> = ({ instance }) => {

  const [newMaintenanceOpen, setNewMaintenanceOpen] = useState(false)
  const [driverRestartOpen, setDriverRestartOpen] = useState(false)
  const [forceRestartOpen, setForceRestartOpen] = useState(false)
  const [forceShutdownOpen, setForceShutdownOpen] = useState(false)
  
  return (
    <>
      <NewMaintenanceModal        instance_id={instance.id} isOpen={newMaintenanceOpen} setOpen={setNewMaintenanceOpen} />
      <RestartInstanceDriverModal instance_id={instance.id} isOpen={driverRestartOpen} setOpen={setDriverRestartOpen} />
      <RestartInstanceModal       instance_id={instance.id} isOpen={forceRestartOpen} setOpen={setForceRestartOpen} />
      {/* TO-DO: update websocket, so it does not require instance_id to instantiate? */}
      <ForceShutdownInstanceModal instance_id={instance.id} isOpen={forceShutdownOpen} setOpen={setForceShutdownOpen} handlePower={() => alert('Missing handlerPower, till websocket update.')} />

      <DropdownMenu>
        <DropdownMenuTrigger className='p-1 bg-background hover:bg-secondary hover:text-white border hover:border-slate-600 rounded-md'>
          <ChevronDownIcon className='h-6 w-6' aria-hidden="false" />
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuLabel>Actions</DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuItem asChild><Link href={`/instances/inspect?instance_id=${instance.id}`}>Inspect</Link></DropdownMenuItem>
          <DropdownMenuItem onClick={() => setNewMaintenanceOpen(true)}>Schedule Maintenance</DropdownMenuItem>
          {/* TO-DO: disable these based on status */}
          <DropdownMenuItem onClick={() => alert('Force play!')}>Force Play</DropdownMenuItem>
          <DropdownMenuItem onClick={() => alert('Force stop!')}>Force Stop</DropdownMenuItem>
          <DropdownMenuItem onClick={() => alert('Force rewind!')}>Force Rewind</DropdownMenuItem>
          <DropdownMenuItem onClick={() => setDriverRestartOpen(true)}>Restart Driver</DropdownMenuItem>
          <DropdownMenuItem onClick={() => setForceRestartOpen(true)}>Force Restart</DropdownMenuItem>
          {/* TO-DO: display these buttons based on status */}
          <DropdownMenuItem onClick={() => setForceShutdownOpen(true)}>Force Shutdown</DropdownMenuItem>
          <DropdownMenuItem onClick={() => alert('Power On!')}>Power On</DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </>
  )
}

export default InstanceActions