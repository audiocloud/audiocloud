'use client'

import React, { useState } from 'react'
import Link from 'next/link'
import { ChevronDownIcon } from '@heroicons/react/20/solid'
import { IDevice } from '@/types'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'
import NewMaintenanceModal from '../../../shared/Modals/NewMaintenance/NewMaintenanceModal'
import RestartDeviceDriverModal from '../../../shared/Modals/RestartDeviceDriverModal'
import RestartDeviceModal from '../../../shared/Modals/RestartDeviceModal'
import ForceShutdownDeviceModal from '../../../shared/Modals/ForceShutdownDeviceModal'

type Props = {
  device: IDevice
}

const DeviceActions: React.FC<Props> = ({ device }) => {

  const [newMaintenanceOpen, setNewMaintenanceOpen] = useState(false)
  const [driverRestartOpen, setDriverRestartOpen] = useState(false)
  const [forceRestartOpen, setForceRestartOpen] = useState(false)
  const [forceShutdownOpen, setForceShutdownOpen] = useState(false)
  
  return (
    <>
      <NewMaintenanceModal        device_id={device.id} isOpen={newMaintenanceOpen} setOpen={setNewMaintenanceOpen} />
      <RestartDeviceDriverModal device_id={device.id} isOpen={driverRestartOpen} setOpen={setDriverRestartOpen} />
      <RestartDeviceModal       device_id={device.id} isOpen={forceRestartOpen} setOpen={setForceRestartOpen} />
      {/* TO-DO: update websocket, so it does not require device_id to instantiate? */}
      <ForceShutdownDeviceModal device_id={device.id} isOpen={forceShutdownOpen} setOpen={setForceShutdownOpen} handlePower={() => alert('Missing handlerPower, till websocket update.')} />

      <DropdownMenu>
        <DropdownMenuTrigger className='p-1 bg-background hover:bg-secondary hover:text-white border hover:border-slate-600 rounded-md'>
          <ChevronDownIcon className='h-6 w-6' aria-hidden="false" />
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuLabel>Actions</DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuItem asChild><Link href={`/devices/inspect?device_id=${device.id}`}>Inspect</Link></DropdownMenuItem>
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

export default DeviceActions