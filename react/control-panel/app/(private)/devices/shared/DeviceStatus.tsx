import React from 'react'
import capitalize from 'lodash.capitalize'
import { cn } from '@/lib/utils'
import { DeviceStatusType } from '@/types'

type Props = {
  status: DeviceStatusType
}

const DeviceStatus: React.FC<Props> = ({ status }) => {
  return (
    <div className='flex items-center gap-3 text-foreground-secondary'>
      <div className={cn(
        'w-3 h-3 rounded-full',
        status === 'online' && 'bg-green-700',
        status === 'offline' && 'bg-red-700',
        // status === 'unknown' && 'bg-slate-500'
      )}/>
      <div>{ capitalize(status) }</div>
    </div>
  )
}

export default DeviceStatus