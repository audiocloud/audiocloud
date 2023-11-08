'use client'

import React, { useState } from 'react'
import { PlusCircleIcon, ArrowPathIcon, ExclamationTriangleIcon, BoltIcon, QuestionMarkCircleIcon, SignalIcon, SignalSlashIcon } from '@heroicons/react/20/solid'
import { Button } from '@/components/ui/button'
import { InstancePowerState } from '@/services/domainClient/types'
import NewMaintenanceModal from '../../shared/Modals/NewMaintenance/NewMaintenanceModal'
import RestartDeviceDriverModal from '../../shared/Modals/RestartDeviceDriverModal'
import RestartDeviceModal from '../../shared/Modals/RestartDeviceModal'
import ForceShutdownDeviceModal from '../../shared/Modals/ForceShutdownDeviceModal'

type Props = {
  device_id: string | undefined,
  powerState: InstancePowerState | 'unknown',
  handlePower: (value: boolean) => void
}

const DeviceActionsBar: React.FC<Props> = ({ device_id, powerState, handlePower }) => {

  if (!device_id) return undefined

  const [newMaintenanceOpen, setNewMaintenanceOpen] = useState(false)
  const [driverRestartOpen, setDriverRestartOpen] = useState(false)
  const [forceRestartOpen, setForceRestartOpen] = useState(false)
  const [forceShutdownOpen, setForceShutdownOpen] = useState(false)
  
  // TO-DO: implement action response status
  
  return (
    <div className='w-full px-4 py-3 flex justify-start items-center gap-2 bg-slate-900/70 border-b'>

      <NewMaintenanceModal        device_id={device_id} isOpen={newMaintenanceOpen} setOpen={setNewMaintenanceOpen} />
      <RestartDeviceDriverModal device_id={device_id} isOpen={driverRestartOpen} setOpen={setDriverRestartOpen} />
      <RestartDeviceModal       device_id={device_id} isOpen={forceRestartOpen} setOpen={setForceRestartOpen} />
      <ForceShutdownDeviceModal device_id={device_id} isOpen={forceShutdownOpen} setOpen={setForceShutdownOpen} handlePower={handlePower} />

      <Button
        variant='objectActionButton'
        onClick={() => setNewMaintenanceOpen(true)}
      >
        <PlusCircleIcon className="h-4 w-4 mr-2" aria-hidden="false" />
        <span>New Maintenance</span>
      </Button>

      <Button
        variant='objectActionButton'
        onClick={() => setDriverRestartOpen(true)}
      >
        <ArrowPathIcon className="h-4 w-4 mr-2" aria-hidden="false" />
        <span>Restart Driver</span>
      </Button>

      <Button
        variant='objectActionButton'
        onClick={() => setForceRestartOpen(true)}
      >
        <ArrowPathIcon className="h-4 w-4 mr-2" aria-hidden="false" />
        <span>Force Restart</span>
      </Button>
      
      { powerState === 'on' && (
        <Button
          variant='objectActionButton'
          onClick={() => setForceShutdownOpen(true)}
        >
          <ExclamationTriangleIcon className="h-4 w-4 mr-2" aria-hidden="false" />
          <span>Force Shutdown</span>
        </Button>
      )}

      { powerState === 'off' && (
        <Button
          variant='objectActionButton'
          onClick={() => handlePower(true)}
        >
          <BoltIcon className="h-4 w-4 mr-2" aria-hidden="false" />
          <span>Power Up</span>
        </Button>
      )}

      { powerState === 'unknown' && (
        <Button
          variant='objectActionButton'
          onClick={() => console.log('Power state unknown')}
          disabled={true}
        >
          <QuestionMarkCircleIcon className="h-4 w-4 mr-2" aria-hidden="false" />
          <span>Power state unknown</span>
        </Button>
      )}
        
    </div>
  )
}

export default DeviceActionsBar