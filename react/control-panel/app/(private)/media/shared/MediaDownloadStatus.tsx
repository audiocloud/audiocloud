import React from 'react'
import { cn } from '@/lib/utils'
import { MediaDownloadUploadStatusType } from '@/types'

type Props = {
  status: MediaDownloadUploadStatusType
}

const MediaDownloadStatus: React.FC<Props> = ({ status }) => {
  return (
    <div className='flex items-center gap-3 text-foreground-secondary'>
      <div className={cn(
        'w-3 h-3 rounded-full',
        status.id === 'undefined' && 'bg-slate-500',
        status.id === 'error' && 'bg-red-700',
        status.id === 'in-progress' && 'bg-yellow-700',
        status.id === 'complete' && 'bg-green-700'
      )}/>
      <div className='whitespace-nowrap'>{ status.label }</div>
    </div>
  )
}

export default MediaDownloadStatus

