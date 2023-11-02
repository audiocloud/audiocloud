import React, { DetailedHTMLProps, HTMLAttributes } from 'react'
import DemoTasksLineChart from './Charts/DemoTasksLineChart'
import DashboardWidget from '../DashboardWidget'

const TasksContents: React.FC<DetailedHTMLProps<HTMLAttributes<HTMLDivElement>, HTMLDivElement>> = ({ className }) => {

  return (
    <DashboardWidget title='Tasks' href='/tasks' className={className}>
      <div className='w-full flex flex-wrap justify-center items-center gap-3'>
        <DemoTasksLineChart />
      </div>
    </DashboardWidget>
  )
}

export default TasksContents