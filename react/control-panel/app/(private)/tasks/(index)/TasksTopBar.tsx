import React from 'react'
import TopBar from '@/components/General/TopBar'
import TopBarButton from '@/components/General/TopBarButton'
import { PlusCircleIcon } from '@heroicons/react/20/solid'

const TasksTopBar: React.FC = () => {
  return (
    <TopBar title='Tasks'>

      <TopBarButton
        label='New Task'
        onClickHandler={() => alert('New Task')}
        icon={<PlusCircleIcon className="h-4 w-4 mr-2" aria-hidden="false" />}
      />

    </TopBar>
  )
}

export default TasksTopBar