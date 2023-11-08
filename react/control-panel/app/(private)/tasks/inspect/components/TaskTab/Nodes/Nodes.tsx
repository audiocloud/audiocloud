import React from 'react'
import { ITask } from '@/types'
import CustomCard from '@/components/general/Card/CustomCard'
import CardLine from '@/components/general/Card/CardLine'
import NodesList from './NodesList'

type Props = {
  task: ITask
}

const Nodes: React.FC<Props> = ({ task }) => {
  return (
    <CustomCard label='Nodes' className='w-[300px]'>
      <CardLine label='List'
        item={<NodesList nodes={task.nodes} />}
        itemsStart={true}
      />
    </CustomCard>
  )
}

export default Nodes