import React from 'react'
import { cn } from '@/lib/utils'
import { TaskStatusType } from '@/types'

type Props = {
  status: TaskStatusType
}

const TaskStatus: React.FC<Props> = ({ status }) => {
  return (
    <div className='flex items-center gap-3'>
      <div className={cn(
        'flex-shrink-0 w-3 h-3 rounded-full',
        status === 'error' && 'bg-red-800',
        status === 'queued' && 'bg-yellow-700',
        status === 'running' && 'bg-sky-700',
        status === 'complete' && 'bg-green-800'
      )}/>
      <span>{ status }</span>
    </div>
  )
}

export default TaskStatus