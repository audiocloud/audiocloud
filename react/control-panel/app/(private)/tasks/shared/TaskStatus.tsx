import React from 'react'
import capitalize from 'lodash.capitalize'
import { cn } from '@/lib/utils'
import { TaskStatusType } from '@/types'

type Props = {
  status: TaskStatusType
}

const TaskStatus: React.FC<Props> = ({ status }) => {
  return (
    <div className='flex items-center gap-3 text-foreground-secondary'>
      <div className={cn(
        'w-3 h-3 rounded-full',
        status === 'error' && 'bg-red-700',
        status === 'queued' && 'bg-yellow-700',
        status === 'running' && 'bg-sky-700',
        status === 'complete' && 'bg-green-700'
      )}/>
      <div>{ capitalize(status) }</div>
    </div>
  )
}

export default TaskStatus