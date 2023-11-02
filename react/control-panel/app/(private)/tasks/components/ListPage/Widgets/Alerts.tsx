import React, { useState } from 'react'
import { tasks } from '@/data/tasks'
import { EllipsisHorizontalIcon } from '@heroicons/react/24/outline'
import { format } from 'date-fns'
import Widget from '@/components/General/Widgets/Widget'

type Props = {
  className: string
}

const Alerts: React.FC<Props> = ({ className }) => {
  
  const [listLength, setListLength] = useState(5)

  const alerts = Object.values(tasks).filter(task => task.status === 'error')

  return (
    <Widget title={'Alerts'} className={className}>

      { alerts.length ? (
        <div className='flex flex-col gap-1'>
          { alerts.slice(0, listLength).map(task => (
            <div key={task.id} className='flex justify-between items-center border border-red-400 bg-red-200 text-red-800 text-xs px-3 py-[1px] rounded-md'>
              <div className='flex justify-start items-center gap-3'>
                <div className='font-medium truncate whitespace-nowrap'>{ task.id }</div>
                <div className='uppercase truncate whitespace-nowrap'>{ task.status }</div>
                <div className='truncate whitespace-nowrap'>{ task.app_id }</div>
              </div>
              <div className='flex justify-end items-center gap-3'>
                <div className='truncate whitespace-nowrap'>{ format(new Date(task.start), 'dd-MM-yyyy @ HH:mm')}</div>
                  <button
                    type='button'
                    className="rounded-md flex justify-center items-center my-1 px-1 text-red-600 hover:text-red-200 hover:bg-red-600"
                  >
                    <EllipsisHorizontalIcon className="h-6 w-6" aria-hidden="false"/>
                  </button>
              </div>
            </div>
          ))}
          { alerts.slice(listLength-1, alerts.length-1).length ? (
            <button
              type='button'
              onClick={() => setListLength(listLength+5)}
              className='py-1 text-center text-sm text-slate-500 hover:underline'
            >(Show more)</button>
          ) : undefined }
        </div>
      ) : <div className='text-center text-sm text-gray-500'>No alerts.</div> }
      
    </Widget>
  )
}

export default Alerts