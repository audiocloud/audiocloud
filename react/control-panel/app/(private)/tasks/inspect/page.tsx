'use client'

import React from 'react'
import { useRouter, useSearchParams } from 'next/navigation'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import TopBar from '@/components/layout/page/TopBar/TopBar'
import PageContent from '@/components/layout/page/PageContent'
import TasksActionsBar from './components/TasksActionsBar'
import TaskTab from './components/TaskTab/TaskTab'
import Statistics from './components/Statistics/Statistics'

// TO-DO: real data
import { tasks } from '@/data/tasks'

const MediaObjectPage: React.FC = () => {

  const router = useRouter()
  const searchParams = useSearchParams()
  const tab = searchParams.get('tab')
  const task_id = searchParams.get('task_id')

  const handleTabChange = (newTab: string) => {
    const params = new URLSearchParams(searchParams)
    params.set('tab', newTab)
    if (task_id) params.set('task_id', task_id)
    router.push(`?${params.toString()}`)
  }

  const task = Object.values(tasks).find(object => object.id === task_id)

  return (
    <Tabs defaultValue='task' value={tab ?? undefined} onValueChange={(e) => handleTabChange(e)} className='w-full'>

      <TopBar title={`domain/${task ? task.id : 'not-found'}`} subtitle={`(task)`} backButton={true}>
        <TabsList>
          <TabsTrigger value='task'>Task</TabsTrigger>
          <TabsTrigger value='statistics'>Statistics</TabsTrigger>
        </TabsList>
      </TopBar>

      <PageContent>
        <TasksActionsBar task_id={task?.id} />

        <TabsContent className='mt-0' value='task'>
          <TaskTab task={task} />
        </TabsContent>

        <TabsContent className='mt-0' value='statistics'>
          <Statistics/>
        </TabsContent>

      </PageContent>

    </Tabs>
  )
}

export default MediaObjectPage