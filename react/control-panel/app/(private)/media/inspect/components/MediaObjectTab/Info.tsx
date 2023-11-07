import React from 'react'
import { Media } from '@/types'
import Widget from '@/components/General/Widgets/Widget'

type Props = {
  file: Media,
  className: string
}

const Info: React.FC<Props> = ({ file, className }) => {
  return (
    <Widget title={'Info'} className={`${className} text-slate-600 text-sm`}>

      <div className='flex justify-between items-center'>
        <span className=''>Media ID</span>
        <span className='text-black'>{ file.id }</span>
      </div>

      <div className='flex justify-between items-center'>
        <span className=''>App ID</span>
        <span className='text-black'>{ file.app_id }</span>
      </div>

      <div className='flex justify-between items-center'>
        <span className=''>Channels</span>
        <span className='text-black'>{ file.metadata?.channels }</span>
      </div>

      <div className='flex justify-between items-center'>
        <span className=''>Sample Rate</span>
        <span className='text-black'>{ file.metadata?.sample_rate && (file.metadata.sample_rate / 1000).toFixed(1) } kHz</span>
      </div>

      <div className='flex justify-between items-center'>
        <span className=''>Bit Depth</span>
        <span className='text-black'>{ file.metadata?.bit_depth && `${file.metadata.bit_depth}-bit ${file.metadata.bit_depth === 32 ? 'float' : ''}`}</span>
      </div>

      <div className='flex justify-between items-center'>
        <span className=''>Format</span>
        <span className='text-black'>{ 'missing format' }</span>
      </div>

      <div className='flex justify-between items-center'>
        <span className=''>Codec</span>
        <span className='text-black'>{ 'missing codec' }</span>
      </div>

      <div className='flex justify-between items-center'>
        <span className=''>Length</span>
        <span className='text-black'>{ file.metadata?.length && `${(file.metadata.length / 60).toFixed(0)} min ${file.metadata.length % 60} sec`}</span>
      </div>
      
    </Widget>
  )
}

export default Info