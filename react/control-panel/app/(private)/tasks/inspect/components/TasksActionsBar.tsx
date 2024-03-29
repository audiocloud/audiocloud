'use client'

import React, { useState } from 'react'
import { TrashIcon, PencilSquareIcon, PlayIcon, StopIcon, ArrowPathIcon } from '@heroicons/react/20/solid'
import { Button } from '@/components/ui/button'
import ManageTaskAPIKeyModal from '../../shared/Modals/ManageTaskAPIKeyModal'
import RestartTaskModal from '../../shared/Modals/RestartTaskModal'
import DeleteTaskModal from '../../shared/Modals/DeleteTaskModal'

type Props = {
  task_id: string | undefined,
}

const TasksActionsBar: React.FC<Props> = ({ task_id }) => {

  if (!task_id) return undefined

  const [manageAPIKeysOpen,setManageAPIKeysOpen] = useState(false)
  const [restartTaskOpen, setRestartTaskOpen] = useState(false)
  const [deleteTaskOpen, setDeleteTaskOpen] = useState(false)
  
  // TO-DO: implement action response status
  
  return (
    <div className='w-full px-4 py-3 flex justify-start items-center gap-2 bg-midground/90 border-b'>

      <ManageTaskAPIKeyModal task_id={task_id} isOpen={manageAPIKeysOpen} setOpen={setManageAPIKeysOpen} current_api_key={'some-key'} />
      <RestartTaskModal task_id={task_id} isOpen={restartTaskOpen} setOpen={setRestartTaskOpen} />
      <DeleteTaskModal task_id={task_id}  isOpen={deleteTaskOpen} setOpen={setDeleteTaskOpen} />

      <Button
        variant='objectActionButton'
        onClick={() => alert('Force play!')}
      >
        <PlayIcon className="h-4 w-4 mr-2" aria-hidden="false" />
        <span>Force Play</span>
      </Button>

      <Button
        variant='objectActionButton'
        onClick={() => alert('Force stop!')}
      >
        <StopIcon className="h-4 w-4 mr-2" aria-hidden="false" />
        <span>Force Stop</span>
      </Button>

      <Button
        variant='objectActionButton'
        onClick={() => setManageAPIKeysOpen(true)}
      >
        <PencilSquareIcon className="h-4 w-4 mr-2" aria-hidden="false" />
        <span>Manage API Key</span>
      </Button>

      <Button
        variant='objectActionButton'
        onClick={() => setRestartTaskOpen(true)}
      >
        <ArrowPathIcon className="h-4 w-4 mr-2" aria-hidden="false" />
        <span>Restart</span>
      </Button>

      <Button
        variant='objectActionButton'
        onClick={() => setDeleteTaskOpen(true)}
      >
        <TrashIcon className="h-4 w-4 mr-2" aria-hidden="false" />
        <span>Delete</span>
      </Button>
        
    </div>
  )
}

export default TasksActionsBar