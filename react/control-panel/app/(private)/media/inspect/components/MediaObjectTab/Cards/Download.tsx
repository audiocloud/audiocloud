import React from 'react'
import { IMediaDownload } from '@/types'
import CustomCard from '@/components/general/Card/CustomCard'
import CardLine from '@/components/general/Card/CardLine'
import MediaDownloadStatus from '../../../../shared/MediaDownloadStatus'
import { getDownloadStatus } from '../../../../shared/getMediaStatuses'
import TextInput from '@/components/general/Card/TextInput'
import DownloadContextModal from './General/DownloadContextModal'

type Props = {
  download: IMediaDownload | undefined
}

const Download: React.FC<Props> = ({ download }) => {
  return (
    <CustomCard label='Download' className='w-[400px]'>
      <div className='w-full flex flex-col justify-start items-center gap-2'>

        <CardLine label='Status' item={<MediaDownloadStatus status={getDownloadStatus(download)} />} />

        { download && (
          <>
            <CardLine label='Progress'    item={download.progress} />
            <CardLine label='Attempts'    item={download.attempts} />
            <CardLine label='Error'       item={download.error ?? 'none'} />
            <CardLine label='Source URL'  item={<TextInput value={download.url}/>} />
            <CardLine label='Notify URL'  item={<TextInput value={download.notify_url ?? ''}/>} />
            <CardLine label='Context'     item={<DownloadContextModal originalContext={JSON.stringify(download.context) ?? ''}/>} />
          </>
        )}

      </div>
    </CustomCard>
  )
}

export default Download