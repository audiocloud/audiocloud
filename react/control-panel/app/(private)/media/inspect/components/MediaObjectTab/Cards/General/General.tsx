import React from 'react'
import { IMedia } from '@/types'
import CustomCard from '@/components/general/Card/CustomCard'
import CardLine from '@/components/general/Card/CardLine'

type Props = {
  media: IMedia
}

const General: React.FC<Props> = ({ media }) => {
  return (
    <CustomCard label='General' className='w-[400px]'>
      <CardLine label='Media ID'    item={media.id} />
      <CardLine label='App ID'      item={media.app_id} />
      <CardLine label='Channels'    item={media.metadata?.channels ?? ''} />
      <CardLine label='Sample Rate' item={media.metadata?.sample_rate ? `${(media.metadata.sample_rate / 1000).toFixed(1)} khz` : ''} />
      <CardLine label='Bit Depth'   item={media.metadata?.bit_depth ? `${media.metadata.bit_depth}${media.metadata.bit_depth === 32 ? '-bit float' : '-bit'}`: ''} />
      <CardLine label='Format'      item={media.metadata?.format ? `.${media.metadata.format}` : ''} />
      <CardLine label='Codec'       item={media.metadata?.codec ? `.${media.metadata.codec}` : ''} />
      <CardLine label='Length'      item={media.metadata?.length ? `${media.metadata.length / 60} min ${media.metadata.length % 60} sec` : ''} />
      <CardLine label='Size'        item={media.metadata?.size ? `${(media.metadata.size / 1000000).toFixed(1)} MB` : ''} />      
    </CustomCard>
  )
}

export default General