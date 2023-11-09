import React from 'react'
import { ITaskNode } from '@/types'

type Props = {
  nodes: ITaskNode[]
}

const NodesList: React.FC<Props> = ({ nodes }) => {
  return (
    <ul role='list' className='flex flex-col gap-1 text-foreground'>
      { nodes.map((node) => <li key={node.id}>{ node.id }</li>) }
      { !nodes.length && <div className='text-foreground'>- none -</div> }
    </ul>
  )
}

export default NodesList