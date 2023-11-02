import React from 'react'
import TopBar from '../../General/TopBar'
import { ArrowPathIcon, ArrowDownTrayIcon, ExclamationTriangleIcon } from '@heroicons/react/20/solid'
import TopBarButton from '@/components/General/TopBarButton'

type Props = {
  file_id: string
}

const MediaItemTopBar: React.FC<Props> = ({ file_id }) => {
  return (
    <TopBar title={file_id} subtitle='(media)'>

      <TopBarButton
        label={'Retry download (from S3'}
        onClickHandler={() => alert('Retry download')}
        icon={<ArrowPathIcon className="h-4 w-4 mr-2" aria-hidden="false" />}
      />
      <TopBarButton
        label={'Retry upload (to S3'}
        onClickHandler={() => alert('Retry upload')}
        icon={<ArrowPathIcon className="h-4 w-4 mr-2" aria-hidden="false" />}
      />
      <TopBarButton
        label={'Download to browser'}
        onClickHandler={() => alert('Download to browser')}
        icon={<ArrowDownTrayIcon className="h-4 w-4 mr-2" aria-hidden="false" />}
      />
      <TopBarButton
        label={'Delete'}
        onClickHandler={() => alert('Delete')}
        icon={<ExclamationTriangleIcon className="h-4 w-4 mr-2" aria-hidden="false" />}
      />

    </TopBar>
  )
}

export default MediaItemTopBar