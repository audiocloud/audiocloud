import React from 'react'
import { cn } from '@/lib/utils'
import { InstanceStatusType } from '@/types'

type Props = {
  status: InstanceStatusType
}

const InstanceStatus: React.FC<Props> = ({ status }) => {
  return (
    <div className='flex items-center gap-3'>
      <div className={cn(
        'flex-shrink-0 w-3 h-3 rounded-full',
        status === 'online' && 'bg-green-800',
        status === 'offline' && 'bg-red-800',
        // status === 'unknown' && 'bg-slate-500'
      )}/>
      <span>({ status })</span>
    </div>
  )
}

export default InstanceStatus