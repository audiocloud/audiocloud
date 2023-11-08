'use client'

import React from 'react'
import { useRouter, useSearchParams } from 'next/navigation'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import TopBar from '@/components/layout/page/TopBar/TopBar'
import PageContent from '@/components/layout/page/PageContent'
import DeviceActionsBar from './components/DeviceActionsBar'
import DeviceTab from './components/DeviceTab/DeviceTab'
import LiveControlTab from './components/LiveControl/LiveControlTab'
import DeviceMaintenancesTable from '../shared/Maintenances/DeviceMaintenancesTable'
import Statistics from './components/Statistics/Statistics'
import { useWebsocketClient } from './components/useWebsocketClient'
import { loadEnvVar } from '@/utils/loadEnvVar'

// TO-DO: real data
import { devices } from '@/data/devices'
import maintenances from '../shared/Maintenances/maintenances'

const DOMAIN_SERVER_IP = loadEnvVar(process.env.NEXT_PUBLIC_DOMAIN_SERVER_IP, 'NEXT_PUBLIC_DOMAIN_SERVER_IP')

const AudioEnginePage: React.FC = () => {

  const router = useRouter()
  const searchParams = useSearchParams()
  const tab = searchParams.get('tab')
  const device_id = searchParams.get('device_id')
  
  // TO-DO: real power state and handlePower API call
  // TO-DO: real IP base on what device it is - or - get correct device IP in useWebsocketClient with an API call
  const { connectionStatus, ws, powerState, handlePower, playState, handlePlay, reports } = useWebsocketClient(DOMAIN_SERVER_IP, device_id)

  const handleTabChange = (newTab: string) => {
    const params = new URLSearchParams(searchParams)
    params.set('tab', newTab)
    if (device_id) params.set('device_id', device_id)
    router.push(`?${params.toString()}`)
  }

  const device = Object.values(devices).find(device => device.id === device_id)

  return (
    <Tabs defaultValue='device' value={tab ?? undefined} onValueChange={(e) => handleTabChange(e)} className='w-full'>

      <TopBar title={`domain/${device ? device.id : 'not-found'}`} subtitle={`(device)`} backButton={true}>
        <TabsList>
          <TabsTrigger value='device'>Device</TabsTrigger>
          <TabsTrigger value='control'>Live Control</TabsTrigger>
          <TabsTrigger value='maintenances'>Maintenances</TabsTrigger>
          <TabsTrigger value='statistics'>Statistics</TabsTrigger>
        </TabsList>
      </TopBar>

      <PageContent>
        <DeviceActionsBar
          device_id={device?.id}
          powerState={powerState}
          handlePower={handlePower}
        />

        <TabsContent className='mt-0' value='device'>
          <DeviceTab device={device} />
        </TabsContent>

        <TabsContent className='mt-0' value='control'>
          <LiveControlTab
            device_id={device?.id}
            connectionStatus={connectionStatus}
            powerState={powerState}
            handlePower={handlePower}
            playState={playState}
            handlePlay={handlePlay}
            reports={reports} 
          />
        </TabsContent>

        <TabsContent className='mt-0' value='maintenances'>
          <DeviceMaintenancesTable maintenances={maintenances.filter(item => item.device_id === device_id)} />
        </TabsContent>

        <TabsContent className='mt-0' value='statistics'>
          <Statistics/>
        </TabsContent>

      </PageContent>

    </Tabs>
  )
}

export default AudioEnginePage