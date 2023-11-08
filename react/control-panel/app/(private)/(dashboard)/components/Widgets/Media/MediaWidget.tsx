import React, { useState } from 'react'
import WidgetBig from '@/components/layout/page/widgets/WidgetBig'
import MediaWarnings from './MediaWarnings'
import { MediaAlertType } from '@/types'

// TO-DO: real data
import { media } from '@/data/media'

const MediaContents: React.FC = () => {

  const [listLength, setListLength] = useState(5)

  const getAlerts = (): MediaAlertType[] => {
    const alerts: MediaAlertType[] = []
    Object.values(media).forEach(media => {
      if (media.upload?.error) {
        alerts.push({
          key: `${media.id}/upload-error`,
          media_id: media.id,
          data: media.upload
        })
      }
      if (media.download?.error) {
        alerts.push({
          key: `${media.id}/download-error`,
          media_id: media.id,
          data: media.download
        })
      }
    })
    return alerts
  }

  const areThereMore = getAlerts().slice(listLength-1, getAlerts().length-1).length

  return (
    <WidgetBig title='Media' href='/media' titleRowItems={
      <>
        <div className='flex justify-center items-end gap-3'>
          <span className='text-slate-500'>Storage Utilization</span>
          <span className='text-5xl font-semibold'>28%</span>
        </div>
      </>
    }>

      <MediaWarnings />
      
    </WidgetBig>
  )
}

export default MediaContents