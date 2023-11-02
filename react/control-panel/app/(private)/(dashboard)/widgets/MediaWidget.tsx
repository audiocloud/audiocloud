'use client'

import React, { DetailedHTMLProps, HTMLAttributes, useState } from 'react'
import Link from 'next/link'
import DemoDonutChart from './Charts/DemoDonutChart'
// import MediaAlert from '@/components/general/Media/MediaAlert'
import WidgetBig from '@/components/layout/page/widgets/big/WidgetBig'
import { MediaAlertType } from '@/types'
import { media } from '@/data/media'

const MediaContents: React.FC<DetailedHTMLProps<HTMLAttributes<HTMLDivElement>, HTMLDivElement>> = ({ className }) => {

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
    <WidgetBig title='Media' href='/media' className={className}>
      <div className='w-full flex justify-between items-start gap-5'>

        <div className='w-1/3'>
          <DemoDonutChart /> 
        </div>

        <div className='w-2/3'>
          <h3 className='pb-3 text-sm text-slate-600'>Media errors ({ getAlerts().length})</h3>

          { getAlerts().length ? (
            <div className='flex flex-col gap-1'>
             
              {/* TO-DO: uncomment */}
              {/* { getAlerts().slice(0, listLength).map(alert => <MediaAlert key={alert.media_id} media_alert={alert} />)} */}

              { areThereMore ? <Link href='/media' className='pt-2 text-sm text-slate-500 hover:text-slate-600 active:text-slate-700 hover:underline'>(See all on media page)</Link> : undefined }
              
            </div>
          ) : (
            <div className='w-full flex justify-between items-center border border-slate-400 bg-slate-200 text-slate-500 text-xs pl-3 pr-2 py-2.5 rounded-md'>
              <div className='flex justify-start items-center gap-3'>
                <div className='font-medium whitespace-nowrap'>No errors found.</div>
              </div>
            </div>
          )}

        </div>

      </div>
    </WidgetBig>
  )
}

export default MediaContents