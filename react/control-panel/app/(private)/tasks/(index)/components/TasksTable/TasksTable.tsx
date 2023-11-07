import React from 'react'
import { tasks } from '@/data/tasks'
import { Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import TaskButtonLink from '@/components/general/ButtonLinks/TaskButtonLink'
import TaskActions from './TaskActions'
import TaskStatus from './TaskStatus'

const TasksTable: React.FC = () => {

  const tasksList = Object.values(tasks)

  return (
    <Table>
        
      { !tasksList.length && <TableCaption>No tasks found.</TableCaption> }

      <TableHeader>
        <TableRow>
          <TableHead>Status</TableHead>
          <TableHead className='whitespace-nowrap'>Task ID</TableHead>
          <TableHead className='whitespace-nowrap'>App ID</TableHead>
          <TableHead>Start</TableHead>
          <TableHead>End</TableHead>
          <TableHead>Nodes</TableHead>
          <TableHead>Mixers</TableHead>
          <TableHead>Tracks</TableHead>
          <TableHead className='text-right'>Actions</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody className='text-slate-400'>
        { tasksList.map((task) => (
          <TableRow className='group/row' key={task.id}>
            <TableCell><TaskStatus status={task.status}/></TableCell>
            <TableCell><TaskButtonLink task_id={task.id}/></TableCell>
            <TableCell>{task.app_id}</TableCell>
            <TableCell>{new Date(task.start).toLocaleString()}</TableCell>
            <TableCell>{new Date(task.end).toLocaleString()}</TableCell>
            <TableCell>{task.nodes.length}</TableCell>
            <TableCell>{task.mixers.length}</TableCell>
            <TableCell>{task.tracks.length}</TableCell>
            <TableCell className='text-right'><TaskActions task={task}/></TableCell>
          </TableRow>
        ))}
      </TableBody>

    </Table>
  )
}

export default TasksTable