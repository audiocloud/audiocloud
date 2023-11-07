import React from 'react'
import { ITask } from '@/types'
import classnames from 'classnames'

type Props = {
  task: ITask
}

const Info: React.FC<Props> = ({ task }) => {
  return (
    <Widget title={'Info'} className={`${className} text-slate-600 text-sm`}>

      <div className='flex justify-between items-center'>
        <span className=''>Task ID</span>
        <span className='text-black'>{ task.id }</span>
      </div>

      <div className='flex justify-between items-center'>
        <span className=''>Status</span>
        <span className='text-black flex items-center gap-2'>
          <span className={classnames('w-2.5 h-2.5 rounded-full', task.status === 'online' ? 'bg-emerald-600' : 'bg-pink-600')} />
          <span>{ capitalize(task.status) }</span>
        </span>
      </div>

      <div className='flex justify-between items-center'>
        <span className=''>App ID</span>
        <span className='text-black'>{ task.app_id }</span>
      </div>

      <div className='flex justify-between items-center'>
        <span className=''>Start time</span>
        <span className='text-black'>{ format(new Date(task.start), 'dd-MM-yyyy @ hh:MM') }</span>
      </div>

      <div className='flex justify-between items-center'>
        <span className=''>End time</span>
        <span className='text-black'>{ format(new Date(task.end), 'dd-MM-yyyy @ hh:MM') }</span>
      </div>

    </Widget>
  )
}

export default Info