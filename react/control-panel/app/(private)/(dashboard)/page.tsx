'use client'

import React from 'react'
import { useRouter, useSearchParams } from 'next/navigation'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import TopBar from '@/components/layout/page/TopBar/TopBar'
import WidgetsContainerBig from '@/components/layout/page/widgets/WidgetsContainerBig'
import AudioEnginesWidget from './components/Widgets/AudioEngines/AudioEnginesWidget'
import InstancesWidget from './components/Widgets/Instances/InstancesWidget'
import MediaWidget from './components/Widgets/Media/MediaWidget'
import TasksWidget from './components/Widgets/Tasks/TasksWidget'

const DashboardPage: React.FC = () => {

  const router = useRouter()
  const searchParams = useSearchParams()
  const tab = searchParams.get('tab')

  return (
    <Tabs defaultValue='status' value={tab ?? undefined} onValueChange={(e) => router.push(`?tab=${e}`)} className='w-full'>

      <TopBar title='Dashboard' subtitle='<domain>'>
        <TabsList>
          <TabsTrigger value='status'>Status</TabsTrigger>
          <TabsTrigger value='configuration'>Configuration</TabsTrigger>
        </TabsList>
      </TopBar>


      <TabsContent className='mt-0' value='status'>
        <WidgetsContainerBig>
          <AudioEnginesWidget />
          <InstancesWidget />
          <MediaWidget />
          <TasksWidget />
        </WidgetsContainerBig>
      </TabsContent>

      <TabsContent className='mt-0' value='configuration'>
        <div className='p-4'>What data and configuration options do we want here?</div>
      </TabsContent>

    </Tabs>
  )
}

export default DashboardPage