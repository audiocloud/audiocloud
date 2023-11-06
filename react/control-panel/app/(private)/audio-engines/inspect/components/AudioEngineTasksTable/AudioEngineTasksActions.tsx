'use client'

import React, { useState } from 'react'
import { Button } from '@/components/ui/button'
import RestartTaskModal from '../../../shared/Modals/RestartTaskModal'
import DeleteTaskModal from '../../../shared/Modals/DeleteTaskModal'

type Props = {
  node_id: string
}

const AudioEngineTasksActions: React.FC<Props> = ({ node_id }) => {

  const [restartTaskModal, setRestartTaskModal] = useState(false)
  const [deleteTaskModal, setDeleteTaskModal] = useState(false)

  return (
    <div className='hidden group-hover/row:flex flex-col xl:flex-row justify-center items-end gap-2'>

      <RestartTaskModal task_node_id={node_id} isOpen={restartTaskModal} setOpen={setRestartTaskModal} />
      <DeleteTaskModal task_node_id={node_id}  isOpen={deleteTaskModal} setOpen={setDeleteTaskModal} />

      <Button size='sm' variant='tableButton' onClick={() => setRestartTaskModal(true)}>Restart</Button>
      <Button size='sm' variant='tableButton' onClick={() => setDeleteTaskModal(true)}>Delete</Button>

    </div>
  )
}

export default AudioEngineTasksActions