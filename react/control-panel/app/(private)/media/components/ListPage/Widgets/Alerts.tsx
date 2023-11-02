import React, { useState } from 'react'
import { media } from '@/data/media'
import MediaAlert from '../../MediaAlert'
import { MediaAlertType } from '@/types'
import Widget from '@/components/General/Widgets/Widget'

type Props = {
  className: string
}

const Alerts: React.FC<Props> = ({ className }) => {
  
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
    <Widget title={'Alerts'} className={className}>

      { getAlerts().length ? (
        <div className='flex flex-col gap-1'>

          { getAlerts().slice(0, listLength).map(alert => <MediaAlert key={alert.media_id} media_alert={alert} />)}

          { areThereMore ? (
            <button
              type='button'
              onClick={() => setListLength(listLength+5)}
              className='py-1 text-center text-sm text-slate-500 hover:underline'
            >(Show more)</button>
          ) : undefined }

        </div>
      ) : (
        <div className='text-center text-sm text-gray-500'>No alerts.</div>
      )}
      
    </Widget>
  )
}

export default Alerts