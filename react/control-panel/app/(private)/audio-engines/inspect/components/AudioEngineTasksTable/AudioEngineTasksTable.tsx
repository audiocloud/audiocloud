import React, { useState, Fragment } from 'react'
import ObjectNotFoundWarning from '@/components/general/ObjectNotFoundWarning'
import { Table, TableBody, TableCaption, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { IEngineTask } from '@/types'
import AudioEngineTasksRow from './AudioEngineTasksRow'

type Props = {
  tasks: IEngineTask[] | undefined
}

const AudioEngineTasksTable: React.FC<Props> = ({ tasks }) => {
  
  const [listLength, setListLength] = useState(5)

  if (tasks === undefined) return <div className='p-3 flex'><ObjectNotFoundWarning objectName='Engine tasks' /></div>

  return (
    <Table>
      
      { !tasks.length && <TableCaption>No existing tasks.</TableCaption> }

      <TableHeader>
        <TableRow>
          <TableHead className='whitespace-nowrap'>Task/Node ID</TableHead>
          <TableHead className='whitespace-nowrap'>Model ID</TableHead>
          <TableHead>CPU</TableHead>
          <TableHead>Memory</TableHead>
          <TableHead>Disk</TableHead>
          <TableHead>Antelope DPS</TableHead>
          <TableHead>Cuda DSP</TableHead>
          <TableHead>UAD DSP</TableHead>
          <TableHead className='text-right'>Actions</TableHead>
        </TableRow>
      </TableHeader>

      <TableBody className='text-foreground-secondary'>
        { tasks.map((task, task_index) => (
          // TO-DO: task index might not be the best thing to use here, they will change when they tasks are completed and get off the list
          <Fragment key={task_index}>
            { Object.values(task.nodes).map((node, node_index) => <AudioEngineTasksRow node={node} task_index={task_index} key={`${task_index}/${node_index}`} />) }
          </Fragment>
        ))}
      </TableBody>

    </Table>
  )
}

export default AudioEngineTasksTable