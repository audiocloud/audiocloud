import React from 'react'
import { ITask } from '@/types'

type Props = {
  task: ITask | undefined
}

const TaskTab: React.FC<Props> = ({ task }) => {
  return (
    <div>TaskTab</div>
  )
}

export default TaskTab