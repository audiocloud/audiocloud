'use client'

import React from 'react'
import { useRouter, useSearchParams } from 'next/navigation'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import TopBar from '@/components/layout/page/TopBar/TopBar'
import PageContent from '@/components/layout/page/PageContent'
import MediaObjectActionsBar from './components/MediaObjectActionsBar'
import MediaObjectTab from './components/MediaObjectTab/MediaObjectTab'
import Statistics from './components/Statistics/Statistics'

// TO-DO: real data
import { media } from '@/data/media'

const MediaObjectPage: React.FC = () => {

  const router = useRouter()
  const searchParams = useSearchParams()
  const tab = searchParams.get('tab')
  const media_id = searchParams.get('media_id')

  const handleTabChange = (newTab: string) => {
    const params = new URLSearchParams(searchParams)
    params.set('tab', newTab)
    if (media_id) params.set('media_id', media_id)
    router.push(`?${params.toString()}`)
  }

  const media_object = Object.values(media).find(object => object.id === media_id)

  return (
    <Tabs defaultValue='media' value={tab ?? undefined} onValueChange={(e) => handleTabChange(e)} className='w-full'>

      <TopBar title={`domain/${media_object ? media_object.id : 'not-found'}`} subtitle={`(media object)`} backButton={true}>
        <TabsList>
          <TabsTrigger value='media'>Media</TabsTrigger>
          <TabsTrigger value='statistics'>Statistics</TabsTrigger>
        </TabsList>
      </TopBar>

      <PageContent>
        <MediaObjectActionsBar media_id={media_object?.id} />

        <TabsContent className='mt-0' value='media'>
          <MediaObjectTab media={media_object} />
        </TabsContent>

        <TabsContent className='mt-0' value='statistics'>
          <Statistics/>
        </TabsContent>

      </PageContent>

    </Tabs>
  )
}

export default MediaObjectPage