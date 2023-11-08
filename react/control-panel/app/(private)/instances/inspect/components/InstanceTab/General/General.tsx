import React from 'react'
import { IInstance } from '@/types'
import InstanceStatus from './InstanceStatus'
import CustomCard from '@/components/general/Card/CustomCard'
import CardLine from '@/components/general/Card/CardLine'

type Props = {
  instance: IInstance
}

const General: React.FC<Props> = ({ instance }) => {

  return (
    <CustomCard label='General' className='w-[400px]'>
      <div className='w-full flex flex-col justify-start items-center gap-1'>
        <CardLine label='Status'        item={<InstanceStatus status={instance.status}/>}/>
        <CardLine label='Last seen'     item={new Date(instance.last_seen).toLocaleString()} />
        <CardLine label='Instance ID'   item={instance.id} />
        <CardLine label='Domain'        item={'<domain>'} />
        <CardLine label='Model ID'      item={instance.model_id} />
        <CardLine label='Driver ID'     item={instance.driver_id} />
        <CardLine label='Engine ID'     item={instance.driver_id} />
        <CardLine label='Engine input'  item={instance.engine_input_at} />
        <CardLine label='Engine output' item={instance.engine_output_at} />
      </div>
    </CustomCard>
  )
}

export default General