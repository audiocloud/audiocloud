import React from 'react'
import { IMediaUpload } from '@/types'
import CustomCard from '@/components/general/Card/CustomCard'
import CardLine from '@/components/general/Card/CardLine'
import MediaDownloadStatus from '../../../../shared/MediaDownloadStatus'
import { getUploadStatus } from '../../../../shared/getMediaStatuses'
import TextInput from '@/components/general/Card/TextInput'
import UploadContextModal from './General/UploadContextModal'

type Props = {
  upload: IMediaUpload | undefined
}

const Upload: React.FC<Props> = ({ upload }) => {
  return (
    <CustomCard label='Upload' className='w-[400px]'>
      
      <div className='w-full flex flex-col justify-center items-center gap-2'>
        <CardLine label='Status' item={<MediaDownloadStatus status={getUploadStatus(upload)} />} />

        { upload && (
          <>
            <CardLine label='Progress'        item={upload.progress} />
            <CardLine label='Attempts'        item={upload.attempts} />
            <CardLine label='Error'           item={upload.error ?? 'none'} />
            <CardLine label='Desination URL'  item={<TextInput value={upload.url}/>} />
            <CardLine label='Notify URL'      item={<TextInput value={upload.notify_url ?? ''}/>} />
            <CardLine label='Context'         item={<UploadContextModal originalContext={JSON.stringify(upload.context) ?? ''}/>} />
          </>
        )}
      </div>

    </CustomCard>
  )
}

export default Upload