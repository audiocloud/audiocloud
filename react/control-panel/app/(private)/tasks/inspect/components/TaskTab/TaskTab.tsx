import React from 'react'
import { ITask } from '@/types'
import General from './General/General'
import ObjectNotFoundWarning from '@/components/general/ObjectNotFoundWarning'

type Props = {
  task: ITask | undefined
}

const TaskTab: React.FC<Props> = ({ task }) => {
  return (
    <div className='p-4 flex flex-wrap gap-4'>
      { task ? (<>
        <General task={task} />
      </>)
      : <ObjectNotFoundWarning objectName='Task'/> }
    </div>
  )
}

export default TaskTab