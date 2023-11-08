'use client'

import React from 'react'
import { useRouter, useSearchParams } from 'next/navigation'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import TopBar from '@/components/layout/page/TopBar/TopBar'
import PageContent from '@/components/layout/page/PageContent'
import MediaTable from './components/MediaTable/MediaTable'
import Statistics from './components/Statistics/Statistics'

const MediaPage: React.FC = () => {

  const router = useRouter()
  const searchParams = useSearchParams()
  const tab = searchParams.get('tab')

  return (
    <Tabs defaultValue='list' value={tab ?? undefined} onValueChange={(e) => router.push(`?tab=${e}`)} className='w-full'>

      <TopBar title='Media' subtitle='<domain>' backButton={true}>
        <TabsList>
          <TabsTrigger value='list'>Index</TabsTrigger>
          <TabsTrigger value='statistics'>Statistics</TabsTrigger>
        </TabsList>
      </TopBar>

      <PageContent>

        <TabsContent className='mt-0' value='list'>
          <MediaTable />
        </TabsContent>

        <TabsContent className='mt-0' value='statistics'>
          <Statistics/>
        </TabsContent>

      </PageContent>

    </Tabs>
  )
}

export default MediaPage