import React from 'react'
import { ITask } from '@/types'
import CustomCard from '@/components/general/Card/CustomCard'
import CardLine from '@/components/general/Card/CardLine'
import MixersList from './MixersList'

type Props = {
  task: ITask
}

const Mixers: React.FC<Props> = ({ task }) => {
  return (
    <CustomCard label='Mixers' className='w-[300px]'>
      <CardLine label='List'
        item={<MixersList mixers={task.mixers} />}
        itemsStart={true}
      />
    </CustomCard>
  )
}

export default Mixers