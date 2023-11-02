import React from 'react'
import DemoTasksLineChart from './Charts/DemoTasksLineChart'
import WidgetBig from '@/components/layout/page/widgets/big/WidgetBig'

type Props = {
  className?: string
}

const TasksContents: React.FC<Props> = ({ className }) => {

  return (
    <WidgetBig title='Tasks' href='/tasks' className={className}>
      <div className='w-full flex flex-wrap justify-center items-center gap-3'>
        <DemoTasksLineChart />
      </div>
    </WidgetBig>
  )
}

export default TasksContents