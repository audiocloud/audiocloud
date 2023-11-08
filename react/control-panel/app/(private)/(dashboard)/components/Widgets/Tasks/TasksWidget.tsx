import React from 'react'
import WidgetBig from '@/components/layout/page/widgets/WidgetBig'
import TaskWarnings from './TaskWarnings'

// TO-DO: real data

const TasksContents: React.FC = () => {

  return (
    <WidgetBig title='Tasks' href='/tasks' titleRowItems={
      <>
        <div className='flex flex-col justify-center items-center'>
          <span className='text-2xl font-bold'>6</span>
          <span className='text-slate-500 text-sm'>Queued</span>
        </div>

        <div className='flex flex-col justify-center items-center'>
          <span className='text-2xl font-bold'>6</span>
          <span className='text-slate-500 text-sm'>Running</span>
        </div>

        <div className='flex flex-col justify-center items-center'>
          <span className='text-2xl font-bold'>456</span>
          <span className='text-slate-500 text-sm'>Completed (last 24h)</span>
        </div>

        <div className='flex flex-col justify-center items-center'>
          <span className='text-2xl font-bold'>12</span>
          <span className='text-slate-500 text-sm'>Errors (last 24h)</span>
        </div>
      </>
    }>

      <TaskWarnings />

    </WidgetBig>
  )
}

export default TasksContents