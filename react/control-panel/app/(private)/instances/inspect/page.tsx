'use client'

import React, { useState } from 'react'
import { useRouter, useSearchParams } from 'next/navigation'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import TopBar from '@/components/layout/page/TopBar/TopBar'
import PageContent from '@/components/layout/page/PageContent'
import InstanceActionsBar from './components/InstanceActionsBar'
import InstanceTab from './components/InstanceTab/InstanceTab'
import LiveControlTab from './components/LiveControl/LiveControlTab'
import InstanceMaintenancesTable from '../shared/Maintenances/InstanceMaintenancesTable'
import Statistics from './components/Statistics/Statistics'
import { useWebsocketClient } from './components/useWebsocketClient'
import { loadEnvVar } from '@/utils/loadEnvVar'

// TO-DO: real data
import { instances } from '@/data/instances'
import maintenances from '../shared/Maintenances/maintenances'

const DOMAIN_SERVER_IP = loadEnvVar(process.env.NEXT_PUBLIC_DOMAIN_SERVER_IP, 'NEXT_PUBLIC_DOMAIN_SERVER_IP')

const AudioEnginePage: React.FC = () => {

  const router = useRouter()
  const searchParams = useSearchParams()
  const tab = searchParams.get('tab')
  const instance_id = searchParams.get('instance_id')
  
  // TO-DO: real power state and handlePower API call
  // TO-DO: real IP base on what instance it is - or - get correct instance IP in useWebsocketClient with an API call
  const { connectionStatus, ws, powerState, handlePower, playState, handlePlay, reports } = useWebsocketClient(DOMAIN_SERVER_IP, instance_id)

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
          <TabsTrigger value='control'>Live Control</TabsTrigger>
          <TabsTrigger value='maintenances'>Maintenances</TabsTrigger>
          <TabsTrigger value='statistics'>Statistics</TabsTrigger>
        </TabsList>
      </TopBar>

      <PageContent>
        <InstanceActionsBar
          instance_id={instance?.id}
          powerState={powerState}
          handlePower={handlePower}
        />

        <TabsContent className='mt-0' value='instance'>
          <InstanceTab instance={instance} />
        </TabsContent>

        <TabsContent className='mt-0' value='control'>
          <LiveControlTab
            instance_id={instance?.id}
            connectionStatus={connectionStatus}
            powerState={powerState}
            handlePower={handlePower}
            playState={playState}
            handlePlay={handlePlay}
            reports={reports} 
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