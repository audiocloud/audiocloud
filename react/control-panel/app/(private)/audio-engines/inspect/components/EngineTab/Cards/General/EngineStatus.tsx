import React from 'react'
import capitalize from 'lodash.capitalize'
import { cn } from '@/lib/utils'
import { AudioEngineStatusType } from '@/types'

type Props = {
  status: AudioEngineStatusType
}

const EngineStatus: React.FC<Props> = ({ status }) => {
  return (
    <span className='flex items-center gap-2 text-foreground'>
      <span className={cn(
        'w-2.5 h-2.5 rounded-full',
        status === 'online' && 'bg-green-700',
        status === 'offline' && 'bg-red-700',
        // status === 'unknown' && 'bg-slate-500'
      )}/>
      <span>{ capitalize(status) }</span>
    </span>
  )
}

export default EngineStatus