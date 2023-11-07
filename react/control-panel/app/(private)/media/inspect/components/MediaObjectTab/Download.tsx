import React from 'react'
import { Media } from '@/types'
import classnames from 'classnames'
import { getFileDownloadStatus } from '@/utils/media/fileStatuses'
import TextInputWithEditButton from '@/components/Media/TextInputWithEditButton'
import Widget from '@/components/General/Widgets/Widget'

type Props = {
  file: Media,
  className: string
}

const Download: React.FC<Props> = ({ file, className }) => {
  
  return (
    <Widget title={'Download'} className={`${className} text-slate-600 text-sm`}>
      
      <div className='flex justify-between items-center'>
        <div className=''>Status</div>
        <div className='text-black flex items-center gap-2'>
          <span
            className={classnames('w-2.5 h-2.5 flex-shrink-0 rounded-full',
              getFileDownloadStatus(file) === 'download_complete' && 'bg-emerald-600', 
              getFileDownloadStatus(file) === 'error' && 'bg-pink-600',
              getFileDownloadStatus(file) === 'downloading' && 'bg-amber-600',
              getFileDownloadStatus(file) === 'unknown' && 'bg-gray-600')}
          />
          <span>
            { getFileDownloadStatus(file) === 'download_complete' && 'Download Complete' }
            { getFileDownloadStatus(file) === 'error' && 'ERROR' }
            { getFileDownloadStatus(file) === 'downloading' && 'Downloading' }
            { getFileDownloadStatus(file) === 'unknown' && 'Unknown' }
          </span>
        </div>
      </div>

      { file.download && (
        <>
          { file.download.error && (
            <div className='flex justify-between items-center'>
              <div className=''>Error</div>
              <div className='text-black'>{ file.download.error }</div>
            </div>
          )}

          <div className='flex justify-between items-center'>
            <div className=''>Attempts</div>
            <div className='text-black'>{ file.download.attempts }</div>
          </div>

          <div className='flex justify-between items-center'>
            <div className=''>Progress</div>
            <div className='text-black'>{ file.download.progress }</div>
          </div>

          <div className='flex justify-between items-center'>
            <div className=''>URL</div>
            <TextInputWithEditButton textValue={ file.download.url }  />
          </div>

          { file.download.notify_url && (
            <div className='flex justify-between items-center'>
              <div className=''>Notify URL</div>
              <TextInputWithEditButton textValue={ file.download.notify_url }  />
            </div>
          )}

          { file.download.context && (
            <div className='flex justify-between items-center'>
              <div className=''>Context</div>
              <TextInputWithEditButton textValue={ file.download.context }  />
            </div>
          )}
        </>
      )}

    </Widget>
  )
}

export default Download