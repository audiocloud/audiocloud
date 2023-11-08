import React from 'react'
import { IDevice } from '@/types'
import DeviceStatus from '../../../../shared/DeviceStatus'
import CustomCard from '@/components/general/Card/CustomCard'
import CardLine from '@/components/general/Card/CardLine'

type Props = {
  device: IDevice
}

const General: React.FC<Props> = ({ device }) => {

  return (
    <CustomCard label='General' className='w-[400px]'>
      <div className='w-full flex flex-col justify-start items-center gap-1'>
        <CardLine label='Status'        item={<DeviceStatus status={device.status}/>}/>
        <CardLine label='Last seen'     item={new Date(device.last_seen).toLocaleString()} />
        <CardLine label='Device ID'     item={device.id} />
        <CardLine label='Domain'        item={'<domain>'} />
        <CardLine label='Model ID'      item={device.model_id} />
        <CardLine label='Driver ID'     item={device.driver_id} />
        <CardLine label='Engine ID'     item={device.driver_id} />
        <CardLine label='Engine input'  item={device.engine_input_at} />
        <CardLine label='Engine output' item={device.engine_output_at} />
      </div>
    </CustomCard>
  )
}

export default General