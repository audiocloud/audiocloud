import React from 'react'
import TopBar from '@/components/General/TopBar'

type Props = {
  task_id: string
}

const TaskTopBar: React.FC<Props> = ({ task_id }) => {
  return (
    <TopBar title={task_id} subtitle='(task)'>
    </TopBar>
  )
}

export default TaskTopBar