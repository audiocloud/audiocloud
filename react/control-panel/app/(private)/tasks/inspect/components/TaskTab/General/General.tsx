import React from 'react'
import { ITask } from '@/types'
import CustomCard from '@/components/general/Card/CustomCard'
import CardLine from '@/components/general/Card/CardLine'
import TaskStatus from '../../../../shared/TaskStatus'

type Props = {
  task: ITask
}

const General: React.FC<Props> = ({ task }) => {
  return (
    <CustomCard label='General' className='w-[400px]'>
      <div className='w-full flex flex-col justify-start items-center gap-2'>
        <CardLine label='Status'      item={<TaskStatus status={task.status}/>}/>
        <CardLine label='Task ID'     item={task.id} />
        <CardLine label='App ID'      item={task.app_id} />
        <CardLine label='Start time'  item={new Date(task.start).toLocaleString()} />
        <CardLine label='End time'    item={new Date(task.end).toLocaleString()} />
      </div>
    </CustomCard>
  )
}

export default General