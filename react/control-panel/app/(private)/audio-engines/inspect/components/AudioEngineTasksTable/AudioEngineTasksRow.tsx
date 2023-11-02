import React from 'react'
import { TableCell, TableRow } from '@/components/ui/table'
import { IEngineTaskNode } from '@/types'
import AudioEngineTasksActions from './AudioEngineTasksActions'

type Props = {
  node: IEngineTaskNode,
  task_index: number
}

const AudioEngineTasksRow: React.FC<Props> = ({ node, task_index }) => {
  return (
    <TableRow className='group/row h-20 text-base'>
      <TableCell><div className="flex items-center space-x-3 truncate">Task { task_index } / { node.id }</div></TableCell>
      <TableCell>{ node.model_id }</TableCell>
      <TableCell>{ node.resources.cpu } MHz</TableCell>
      <TableCell>{ node.resources.memory } MB</TableCell>
      <TableCell>{ node.resources.disk } MB</TableCell>
      <TableCell>{ node.resources.antelope_dsp } core(s)</TableCell>
      <TableCell>{ node.resources.cuda_dsp || 0 } core(s)</TableCell>
      <TableCell>{ node.resources.uad_dsp || 0 } core(s)</TableCell>
      <TableCell className='text-right w-fit xl:w-48'><AudioEngineTasksActions node_id={node.id} /></TableCell>
    </TableRow>
  )
}

export default AudioEngineTasksRow