'use client'

import React, { useState } from 'react'
import { ArrowUpTrayIcon, ArrowDownTrayIcon, TrashIcon } from '@heroicons/react/20/solid'
import { Button } from '@/components/ui/button'
import RetryDownloadModal from '../../shared/Modals/RetryDownloadModal'
import RetryUploadModal from '../../shared/Modals/RetryUploadModal'
import ForceDeleteMediaModal from '../../shared/Modals/ForceDeleteMediaModal'

type Props = {
  media_id: string | undefined
}

const MediaObjectActionsBar: React.FC<Props> = ({ media_id }) => {

  if (!media_id) return undefined

  const [retryDownload, setRetryDownloadOpen] = useState(false)
  const [retryUpload, setRetryUploadOpen] = useState(false)
  const [forceDeleteOpen, setForceDeleteOpen] = useState(false)

  return (
    <div className='w-full px-4 py-3 flex justify-start items-center gap-2 bg-slate-900/70 border-b'>

      <RetryDownloadModal media_id={media_id} isOpen={retryDownload} setOpen={setRetryDownloadOpen} />
      <RetryUploadModal media_id={media_id} isOpen={retryUpload} setOpen={setRetryUploadOpen} />
      <ForceDeleteMediaModal media_id={media_id} isOpen={forceDeleteOpen} setOpen={setForceDeleteOpen} />

      <Button
        variant='objectActionButton'
        onClick={() => setRetryDownloadOpen(true)}
      >
        <ArrowDownTrayIcon className="h-4 w-4 mr-2" aria-hidden="false" />
        <span>Retry Download (domain)</span>
      </Button>

      <Button
        variant='objectActionButton'
        onClick={() => setRetryUploadOpen(true)}
      >
        <ArrowUpTrayIcon className="h-4 w-4 mr-2" aria-hidden="false" />
        <span>Retry Upload (S3) Restart</span>
      </Button>

      <Button
        variant='objectActionButton'
        onClick={() => setForceDeleteOpen(true)}
      >
        <TrashIcon className="h-4 w-4 mr-2" aria-hidden="false" />
        <span>Force Delete</span>
      </Button>

      <Button
        variant='objectActionButton'
        onClick={() => alert('Direct download!')}
      >
        <ArrowDownTrayIcon className="h-4 w-4 mr-2" aria-hidden="false" />
        <span>Direct Download</span>
      </Button>

    </div>
  )
}

export default MediaObjectActionsBar