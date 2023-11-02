'use client'

import React from 'react'
import { useRouter, useSearchParams } from 'next/navigation'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import TopBar from '@/components/layout/page/TopBar/TopBar'
import PageContent from '@/components/layout/page/PageContent'
import AudioEnginesTable from './components/AudioEnginesTable/AudioEnginesTable'
import AudioEnginesMaintenancesTable from '../shared/Maintenances/AudioEngineMaintenancesTable'
import Statistics from './components/Statistics/Statistics'

// TO-DO: real data
import maintenances from '../shared/Maintenances/maintenances' 

const AudioEnginesPage: React.FC = () => {

  const router = useRouter()
  const searchParams = useSearchParams()
  const tab = searchParams.get('tab')

  return (
    <Tabs defaultValue='list' value={tab ?? undefined} onValueChange={(e) => router.push(`?tab=${e}`)} className='w-full'>

      <TopBar title='Audio Engines' subtitle='<domain>'>
        <TabsList>
          <TabsTrigger value='list'>Index</TabsTrigger>
          <TabsTrigger value='maintenances'>Maintenances</TabsTrigger>
          <TabsTrigger value='statistics'>Statistics</TabsTrigger>
        </TabsList>
      </TopBar>

      <PageContent>

        <TabsContent className='mt-0' value='list'>
          <AudioEnginesTable />
        </TabsContent>

        <TabsContent className='mt-0' value='maintenances'>
          <AudioEnginesMaintenancesTable maintenances={maintenances} />
        </TabsContent>

        <TabsContent className='mt-0' value='statistics'>
          <Statistics/>
        </TabsContent>

      </PageContent>

    </Tabs>
  )
}

export default AudioEnginesPage