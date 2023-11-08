import React from 'react'
import WidgetBig from '@/components/layout/page/widgets/WidgetBig'
import AudioEngineWarnings from './AudioEngineWarnings'

// TO-DO: real data
const data = [
  {
    id: 'ae-0',
    status: 'online',
    cpu: '200 MHz',
    ram: '400 MB',
    disk: '500 MB',
    tasks: '3'
  },
  {
    id: 'ae-1',
    status: 'online',
    cpu: '200 MHz',
    ram: '400 MB',
    disk: '500 MB',
    tasks: '3'
  },
  {
    id: 'ae-2',
    status: 'offline',
    cpu: '200 MHz',
    ram: '400 MB',
    disk: '500 MB',
    tasks: '3'
  }
]

const AudioEnginesContents: React.FC = () => {

  return (
    <WidgetBig title='Audio Engines' href='/audio-engines' titleRowItems={
      <>
        <div className='flex justify-center items-end gap-3'>
          <span className='text-slate-500'>Online</span>
          <span className='text-5xl font-semibold'>4/4</span>
        </div>
      </>
    }>

      <AudioEngineWarnings />
      
    </WidgetBig>
  )
}

export default AudioEnginesContents