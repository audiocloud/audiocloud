import React, { useState } from 'react'
import { instances } from '@/data/instances'
import InstanceAlert from '../../InstanceAlert'
import Widget from '@/components/General/Widgets/Widget'

type Props = {
  className: string
}

const Alerts: React.FC<Props> = ({ className }) => {
  
  const [amountToShow, setAmountToShow] = useState(5)

  const alerts = Object.values(instances).filter(instance => instance.status !== 'online')
  const areThereMore = alerts.slice(amountToShow-1, alerts.length-1).length

  return (
    <Widget title={'Alerts'} className={className}>

      { alerts.length ? (
        <div className='flex flex-col gap-1'>

          { alerts.slice(0, amountToShow).map(instance => <InstanceAlert key={instance.id} instance={instance} />)}
          
          { !!areThereMore && (
            <button
              type='button'
              onClick={() => setAmountToShow(amountToShow+5)}
              className='py-1 text-center text-sm text-slate-500 hover:underline'
            >(Show more)</button>
          )}

        </div>
      ) : (
        <div className='text-center text-sm text-gray-500'>No alerts.</div>
      )}
      
    </Widget>
  )
}

export default Alerts