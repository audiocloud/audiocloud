'use client'

import React from 'react'
import { useRouter, useSearchParams } from 'next/navigation'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import TopBar from '@/components/layout/page/TopBar/TopBar'
import PageContent from '@/components/layout/page/PageContent'
import DevicesTable from './components/DevicesTable/DevicesTable'
import DeviceMaintenancesTable from '../shared/Maintenances/DeviceMaintenancesTable'
import Statistics from './components/Statistics/Statistics'

// TO-DO: real data
import maintenances from '../shared/Maintenances/maintenances' 

const DevicesPage: React.FC = () => {

  const router = useRouter()
  const searchParams = useSearchParams()
  const tab = searchParams.get('tab')

  return (
    <Tabs defaultValue='list' value={tab ?? undefined} onValueChange={(e) => router.push(`?tab=${e}`)} className='w-full'>

      <TopBar title='Devices' subtitle='<domain>' backButton={true}>
        <TabsList>
          <TabsTrigger value='list'>Index</TabsTrigger>
          <TabsTrigger value='maintenances'>Maintenances</TabsTrigger>
          <TabsTrigger value='statistics'>Statistics</TabsTrigger>
        </TabsList>
      </TopBar>

      <PageContent>

        <TabsContent className='mt-0' value='list'>
          <DevicesTable />
        </TabsContent>

        <TabsContent className='mt-0' value='maintenances'>
          <DeviceMaintenancesTable maintenances={maintenances} />
        </TabsContent>

        <TabsContent className='mt-0' value='statistics'>
          <Statistics/>
        </TabsContent>

      </PageContent>

    </Tabs>
  )
}

export default DevicesPage