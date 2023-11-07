'use client'

import React, { useState } from 'react'
import Link from 'next/link'
import { ChevronDownIcon } from '@heroicons/react/20/solid'
import { IMedia } from '@/types'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'
import RetryDownloadModal from '../../../shared/Modals/RetryDownloadModal'
import RetryUploadModal from '../../../shared/Modals/RetryUploadModal'
import ForceDeleteMediaModal from '../../../shared/Modals/ForceDeleteMediaModal'

type Props = {
  media: IMedia
}

const MediaActions: React.FC<Props> = ({ media }) => {

  const [retryDownload, setRetryDownloadOpen] = useState(false)
  const [retryUpload, setRetryUploadOpen] = useState(false)
  const [forceDeleteOpen, setForceDeleteOpen] = useState(false)

  return (
    <>
      <RetryDownloadModal media_id={media.id} isOpen={retryDownload} setOpen={setRetryDownloadOpen} />
      <RetryUploadModal media_id={media.id} isOpen={retryUpload} setOpen={setRetryUploadOpen} />
      <ForceDeleteMediaModal media_id={media.id} isOpen={forceDeleteOpen} setOpen={setForceDeleteOpen} />
      
      <DropdownMenu>
        <DropdownMenuTrigger className='p-1 bg-background hover:bg-secondary hover:text-white border hover:border-slate-600 rounded-md'>
          <ChevronDownIcon className='h-6 w-6' aria-hidden="false" />
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuLabel>Actions</DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuItem asChild><Link href={`/media/inspect?media_id=${media.id}`}>Inspect</Link></DropdownMenuItem>
          <DropdownMenuItem onClick={() => setRetryDownloadOpen(true)}>Retry Download</DropdownMenuItem>
          <DropdownMenuItem onClick={() => setRetryUploadOpen(true)}>Retry Upload</DropdownMenuItem>
          <DropdownMenuItem onClick={() => setForceDeleteOpen(true)}>Force Delete</DropdownMenuItem>
          <DropdownMenuItem onClick={() => alert('Direct download!')}>Direct Download</DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </>
  )
}

export default MediaActions