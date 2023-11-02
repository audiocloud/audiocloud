import React from 'react'
import { IAudioEngine } from '@/types'
import EngineStatus from './EngineStatus'
import SupportedModelsList from './SupportedModelsList'
import CustomCard from '@/components/general/Card/CustomCard'
import CardLine from '@/components/general/Card/CardLine'

type Props = {
  engine: IAudioEngine
}

const General: React.FC<Props> = ({ engine }) => {

  return (
    <CustomCard label='General' className='w-[400px]'>
      <CardLine label='Status' item={<EngineStatus status={engine.status}/>}/>
      <CardLine label='Last seen' item={new Date(engine.last_seen).toLocaleString()} />
      <CardLine label='Engine ID' item={engine.id} />
      <CardLine label='Domain' item={'<domain>'} />
      <CardLine label='Machine' item={engine.machine} />
      <CardLine label='Supported Models'
        item={<SupportedModelsList models={engine.models} />}
        itemsStart={true}
      />
    </CustomCard>
  )
}

export default General