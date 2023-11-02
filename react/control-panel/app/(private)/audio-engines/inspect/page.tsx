'use client'

import React from 'react'
import { useRouter, useSearchParams } from 'next/navigation'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import TopBar from '@/components/layout/page/TopBar/TopBar'
import PageContent from '@/components/layout/page/PageContent'
import AudioEngineActionsBar from './components/AudioEngineActionsBar'
import EngineTab from './components/EngineTab/EngineTab'
import AudioEnginesMaintenancesTable from '../shared/Maintenances/AudioEngineMaintenancesTable'
import Statistics from './components/Statistics/Statistics'

// TO-DO: real data
import { audio_engines } from '@/data/audio-engines'
import maintenances from '../shared/Maintenances/maintenances' 
import AudioEngineTasksTable from './components/AudioEngineTasksTable/AudioEngineTasksTable'

const AudioEnginePage: React.FC = () => {

  const router = useRouter()
  const searchParams = useSearchParams()
  const tab = searchParams.get('tab')
  const engine_id = searchParams.get('engine_id')

  const handleTabChange = (newTab: string) => {
    const params = new URLSearchParams(searchParams)
    params.set('tab', newTab)
    if (engine_id) params.set('engine_id', engine_id)
    router.push(`?${params.toString()}`)
  }

  const audio_engine = Object.values(audio_engines).find(engine => engine.id === engine_id)
  const tasks = audio_engine?.engine_tasks

  return (
    <Tabs defaultValue='engine' value={tab ?? undefined} onValueChange={(e) => handleTabChange(e)} className='w-full'>

      <TopBar title={`domain/${audio_engine ? audio_engine.id : 'not-found'}`} subtitle={`(audio engine)`} backButton={true}>
        <TabsList>
          <TabsTrigger value='engine'>Engine</TabsTrigger>
          <TabsTrigger value='tasks'>Tasks</TabsTrigger>
          <TabsTrigger value='maintenances'>Maintenances</TabsTrigger>
          <TabsTrigger value='statistics'>Statistics</TabsTrigger>
        </TabsList>
      </TopBar>

      <PageContent>
        <AudioEngineActionsBar engine_id={audio_engine?.id} />

        <TabsContent className='mt-0' value='engine'>
          <EngineTab audioEngine={audio_engine} />
        </TabsContent>

        <TabsContent className='mt-0' value='tasks'>
          <AudioEngineTasksTable tasks={tasks} />
        </TabsContent>

        <TabsContent className='mt-0' value='maintenances'>
          <AudioEnginesMaintenancesTable maintenances={maintenances.filter(item => item.engine_id === engine_id)} />
        </TabsContent>

        <TabsContent className='mt-0' value='statistics'>
          <Statistics/>
        </TabsContent>

      </PageContent>

    </Tabs>
  )
}

export default AudioEnginePage