import React from 'react'
import WidgetBig from '@/components/layout/page/widgets/WidgetBig'
import DeviceWarnings from './DeviceWarnings'

// TO-DO: real data

const DevicesContents: React.FC = () => {

  return (
    <WidgetBig title='Devices' href='/devices' titleRowItems={
      <>
        <div className='flex justify-center items-end gap-3'>
          <span className='text-foreground-secondary'>Online</span>
          <span className='text-foreground text-5xl font-semibold'>18/22</span>
        </div>
      </>
    }>

      <DeviceWarnings />

    </WidgetBig>
  )
}

export default DevicesContents