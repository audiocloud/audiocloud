import React from 'react'
import TopBar from '@/components/layout/page/TopBar/TopBar'
import WidgetsContainerBig from '@/components/layout/page/widgets/big/WidgetsContainerBig'
import AudioEnginesWidget from './widgets/AudioEnginesWidget'
import InstancesWidget from './widgets/InstancesWidget'
import MediaWidget from './widgets/MediaWidget'
import TasksWidget from './widgets/TasksWidget'

const DashboardPage: React.FC = () => {
  return (
    <>
      <TopBar title='Dashboard'/>
      <WidgetsContainerBig>
        <AudioEnginesWidget className=''/>
        <InstancesWidget    className='border-t xl:border-t-0 border-slate-300'/>
        <MediaWidget        className='border-t border-slate-300'/>
        <TasksWidget        className='border-t border-slate-300'/>
      </WidgetsContainerBig>
    </>
  )
}

export default DashboardPage