import React, { useState } from 'react'
import Link from 'next/link'
import { ChevronDownIcon } from '@heroicons/react/20/solid'
import { ITask } from '@/types'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'
import ManageTaskAPIKeysModal from '../../../shared/Modals/ManageTaskAPIKeyModal'
import RestartTaskModal from '../../../shared/Modals/RestartTaskModal'
import DeleteTaskModal from '../../../shared/Modals/DeleteTaskModal'

type Props = {
  task: ITask
}

const TaskActions: React.FC<Props> = ({ task }) => {

  const [manageAPIKeysOpen,setManageAPIKeysOpen] = useState(false)
  const [restartTaskOpen, setRestartTaskOpen] = useState(false)
  const [deleteTaskOpen, setDeleteTaskOpen] = useState(false)

  return (
    <>
      <ManageTaskAPIKeysModal task_id={task.id} isOpen={manageAPIKeysOpen} setOpen={setManageAPIKeysOpen} current_api_key={'some-key'} />
      <RestartTaskModal task_id={task.id} isOpen={restartTaskOpen} setOpen={setRestartTaskOpen} />
      <DeleteTaskModal task_id={task.id}  isOpen={deleteTaskOpen} setOpen={setDeleteTaskOpen} />

      <DropdownMenu>
        <DropdownMenuTrigger className='p-1 bg-background hover:bg-secondary hover:text-white border hover:border-slate-600 rounded-md'>
          <ChevronDownIcon className='h-6 w-6' aria-hidden="false" />
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuLabel>Actions</DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuItem asChild><Link href={`/tasks/inspect?task_id=${task.id}`}>Inspect</Link></DropdownMenuItem>
          <DropdownMenuItem onClick={() => alert('Force play!')}>Force Play</DropdownMenuItem>
          <DropdownMenuItem onClick={() => alert('Force stop!')}>Force Stop</DropdownMenuItem>
          <DropdownMenuItem onClick={() => setManageAPIKeysOpen(true)}>Manage API Key</DropdownMenuItem>
          <DropdownMenuItem onClick={() => setRestartTaskOpen(true)}>Restart</DropdownMenuItem>
          <DropdownMenuItem onClick={() => setDeleteTaskOpen(true)}>Delete</DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </>
  )
}

export default TaskActions