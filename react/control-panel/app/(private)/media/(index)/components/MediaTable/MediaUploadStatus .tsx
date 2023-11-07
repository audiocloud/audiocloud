import React from 'react'
import { cn } from '@/lib/utils'
import { MediaDownloadUploadStatusType } from '@/types'

type Props = {
  status: MediaDownloadUploadStatusType
}

const MediaUploadStatus: React.FC<Props> = ({ status }) => {
  return (
    <div className='flex items-center gap-3'>
      <div className={cn(
        'flex-shrink-0 w-3 h-3 rounded-full',
        status.id === 'undefined' && 'bg-slate-500',
        status.id === 'error' && 'bg-red-800',
        status.id === 'in-progress' && 'bg-yellow-700',
        status.id === 'complete' && 'bg-green-800'
      )}/>

      <span>{ status.label }</span>
    </div>
  )
}

export default MediaUploadStatus