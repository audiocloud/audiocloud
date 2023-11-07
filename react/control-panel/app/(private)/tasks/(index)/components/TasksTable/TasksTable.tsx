import React from 'react'
import { tasks } from '@/data/tasks'
import { Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import TaskButtonLink from '@/components/general/TaskButtonLink'
import TaskActions from './TaskActions'

const TasksTable: React.FC = () => {

  const tasksList = Object.values(tasks)

  return (
    <Table>
        
      { !tasksList.length && <TableCaption>No tasks found.</TableCaption> }

      <TableHeader>
        <TableRow>
          <TableHead className='whitespace-nowrap'>Task ID</TableHead>
          <TableHead className='text-right'>Actions</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody className='text-slate-400'>
        { tasksList.map((task) => (
          <TableRow className='group/row' key={task.id}> 
            <TableCell><TaskButtonLink task_id={task.id}/></TableCell>
            <TableCell className='text-right'><TaskActions task={task}/></TableCell>
          </TableRow>
        ))}
      </TableBody>

    </Table>
  )
}

export default TasksTable