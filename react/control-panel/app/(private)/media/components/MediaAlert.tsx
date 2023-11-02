import React, { useState } from 'react'
import Link from 'next/link'
import { ArrowPathIcon, MagnifyingGlassIcon, TrashIcon } from '@heroicons/react/24/outline'
import DeleteConfirmation from '@/components/features/Media/Modals/DeleteConfirmation'
import { MediaDownload, MediaUpload } from '@/types'
import Alert from '@/components/general/Alerts/Alert'
import AlertActionButton from '@/components/general/Alerts/AlertActionButton'

type Props = {
  media_alert: {
    key: string,
    media_id: string,
    data: MediaUpload | MediaDownload
  }
}

const MediaAlert: React.FC<Props> = ({ media_alert }) => {

  const [deleteModalOpen, setDeleteModalOpen] = useState(false)

  return (
    <>
      <Alert
        subject={media_alert.media_id}
        status={media_alert.data.error ? media_alert.data.error : 'Unknown'}
        extra_info={`Attempts: ${media_alert.data.attempts}`}
        buttons={[
          <AlertActionButton
            key='Retry Action'
            onClickHandler={() => alert('Retry Action')}
            icon={<ArrowPathIcon className="w-4 h-4" aria-hidden="false"/>}
          />,
          <AlertActionButton
            key='Delete'
            onClickHandler={() => setDeleteModalOpen(true)}
            icon={<TrashIcon className="w-4 h-4" aria-hidden="false"/>}
          />,
          <Link key='Inspect' href={`/media/${media_alert.media_id}`}>
            <AlertActionButton
              onClickHandler={() => {return}}
              icon={<MagnifyingGlassIcon className="w-4 h-4" aria-hidden="false"/>}
            />
          </Link>
        ]}
      />
      <DeleteConfirmation media_id={media_alert.media_id} open={deleteModalOpen} setOpen={setDeleteModalOpen} />
    </>
  )
}

export default MediaAlert