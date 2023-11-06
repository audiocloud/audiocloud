'use client'

import React, { useState } from 'react'
import { PlusCircleIcon, ArrowPathIcon, ExclamationTriangleIcon, BoltIcon, QuestionMarkCircleIcon, SignalIcon, SignalSlashIcon } from '@heroicons/react/20/solid'
import { Button } from '@/components/ui/button'
import { InstancePowerState } from '@/services/domainClient/types'
import NewMaintenance from '../../shared/Modals/NewMaintenance'

type Props = {
  instance_id: string | undefined,
  powerState: InstancePowerState | 'unknown',
  handlePower: (value: boolean) => void
}

const InstanceActionsBar: React.FC<Props> = ({ instance_id, powerState, handlePower }) => {

  if (!instance_id) return undefined

  const [newMaintenance, setNewMaintenance] = useState(false)
  
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
        onClick={() => alert('Restart driver clicked!')}
      >
        <ArrowPathIcon className="h-4 w-4 mr-2" aria-hidden="false" />
        <span>Restart Driver</span>
      </Button>

      <Button
        variant='objectActionButton'
        onClick={() => alert('Restart instance clicked!')}
      >
        <ArrowPathIcon className="h-4 w-4 mr-2" aria-hidden="false" />
        <span>Restart Instance</span>
      </Button>
      
      { powerState === 'on' && (
        <Button
          variant='objectActionButton'
          onClick={() => handlePower(false)}
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

      <NewMaintenance instance_id={instance_id} open={newMaintenance} setOpen={setNewMaintenance} />
        
    </div>
  )
}

export default InstanceActionsBar