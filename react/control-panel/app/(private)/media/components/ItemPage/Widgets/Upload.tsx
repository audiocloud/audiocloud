import React from 'react'
import { Media } from '@/types'
import classnames from 'classnames'
import { getFileUploadStatus } from '@/utils/media/fileStatuses'
import TextInputWithEditButton from '@/components/Media/TextInputWithEditButton'
import Widget from '@/components/General/Widgets/Widget'

type Props = {
  file: Media,
  className: string
}

const Upload: React.FC<Props> = ({ file, className }) => {
  return (
    <Widget title={'Upload'} className={`${className} text-slate-600 text-sm`}>

      <div className='flex justify-between items-center'>
        <span className=''>Status</span>
        <span className='text-black flex items-center gap-2'>
          <span
            className={classnames('w-2.5 h-2.5 flex-shrink-0 rounded-full',
              getFileUploadStatus(file) === 'upload_complete' && 'bg-emerald-600', 
              getFileUploadStatus(file) === 'error' && 'bg-pink-600',
              getFileUploadStatus(file) === 'uploading' && 'bg-amber-600',
              getFileUploadStatus(file) === 'unknown' && 'bg-gray-600')}
          />
          <span>
            { getFileUploadStatus(file) === 'upload_complete' && 'Upload Complete' }
            { getFileUploadStatus(file) === 'error' && 'ERROR' }
            { getFileUploadStatus(file) === 'uploading' && 'Uploading' }
            { getFileUploadStatus(file) === 'unknown' && 'Unknown' }
          </span>
        </span>
      </div>

      { file.upload && (
        <>
          { file.upload.error && (
            <div className='flex justify-between items-center'>
              <div className=''>Error</div>
              <div className='text-black'>{ file.upload.error }</div>
            </div>
          )}

          <div className='flex justify-between items-center'>
            <div className=''>Attempts</div>
            <div className='text-black'>{ file.upload.attempts }</div>
          </div>

          <div className='flex justify-between items-center'>
            <div className=''>Progress</div>
            <div className='text-black'>{ file.upload.progress }</div>
          </div>

          <div className='flex justify-between items-center'>
            <span className=''>URL</span>
            <TextInputWithEditButton textValue={ file.upload.url } />
          </div>

          { file.upload.notify_url && (
            <div className='flex justify-between items-center'>
              <div className=''>Notify URL</div>
              <TextInputWithEditButton textValue={ file.upload.notify_url }  />
            </div>
          )}

          { file.upload.context && (
            <div className='flex justify-between items-center'>
              <div className=''>Context</div>
              <TextInputWithEditButton textValue={ file.upload.context }  />
            </div>
          )}
        </>
      )}

    </Widget>
  )
}

export default Upload