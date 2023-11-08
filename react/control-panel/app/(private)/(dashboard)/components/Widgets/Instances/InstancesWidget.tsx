import React from 'react'
import WidgetBig from '@/components/layout/page/widgets/WidgetBig'
import InstanceWarnings from './InstanceWarnings'

// TO-DO: real data

const InstancesContents: React.FC = () => {

  return (
    <WidgetBig title='Instances' href='/instances' titleRowItems={
      <>
        <div className='flex justify-center items-end gap-3'>
          <span className='text-slate-500'>Online</span>
          <span className='text-5xl font-semibold'>18/22</span>
        </div>
      </>
    }>

      <InstanceWarnings />

    </WidgetBig>
  )
}

export default InstancesContents