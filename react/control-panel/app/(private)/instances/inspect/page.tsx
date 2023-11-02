'use client'

import React, { useState } from 'react'
import { useRouter, useSearchParams } from 'next/navigation'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import TopBar from '@/components/layout/page/TopBar/TopBar'
import PageContent from '@/components/layout/page/PageContent'
import InstanceActionsBar from './components/InstanceActionsBar'
import InstanceTab from './components/InstanceTab/InstanceTab'
import InstanceMaintenancesTable from '../shared/Maintenances/InstanceMaintenancesTable'
import Statistics from './components/Statistics/Statistics'

// TO-DO: real data
import { instances } from '@/data/instances'
import maintenances from '../shared/Maintenances/maintenances' 
import { InstancePowerState } from '@/services/domainClient/types'

const AudioEnginePage: React.FC = () => {

  const [liveControlEnabled, setLiveControlEnabled] = useState(false)
  const powerState: InstancePowerState | 'unknown' = 'unknown'
  const handlePower: (value: boolean) => void = () => console.log('test') 

  const router = useRouter()
  const searchParams = useSearchParams()
  const tab = searchParams.get('tab')
  const instance_id = searchParams.get('instance_id')

  const handleTabChange = (newTab: string) => {
    const params = new URLSearchParams(searchParams)
    params.set('tab', newTab)
    if (instance_id) params.set('instance_id', instance_id)
    router.push(`?${params.toString()}`)
  }

  const instance = Object.values(instances).find(instance => instance.id === instance_id)

  return (
    <Tabs defaultValue='instance' value={tab ?? undefined} onValueChange={(e) => handleTabChange(e)} className='w-full'>

      <TopBar title={`domain/${instance ? instance.id : 'not-found'}`} subtitle={`(instance)`} backButton={true}>
        <TabsList>
          <TabsTrigger value='instance'>Instance</TabsTrigger>
          <TabsTrigger value='maintenances'>Maintenances</TabsTrigger>
          <TabsTrigger value='statistics'>Statistics</TabsTrigger>
        </TabsList>
      </TopBar>

      <PageContent>
        <InstanceActionsBar
          instance_id={instance?.id}
          powerState={'off'}
          handlePower={handlePower}
          liveControlEnabled={liveControlEnabled}
          setLiveControlEnabled={setLiveControlEnabled}
        />

        <TabsContent className='mt-0' value='instance'>
          <InstanceTab
            instance={instance}
            powerState={'off'}
            handlePower={(a) => console.log(a)}
            className={''}
          />
        </TabsContent>

        <TabsContent className='mt-0' value='maintenances'>
          <InstanceMaintenancesTable maintenances={maintenances.filter(item => item.instance_id === instance_id)} />
        </TabsContent>

        <TabsContent className='mt-0' value='statistics'>
          <Statistics/>
        </TabsContent>

      </PageContent>

    </Tabs>
  )
}

export default AudioEnginePage