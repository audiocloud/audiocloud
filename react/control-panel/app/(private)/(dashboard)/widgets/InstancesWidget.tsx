import React from 'react'
import WidgetBig from '@/components/layout/page/widgets/big/WidgetBig'
import DemoInstancesLineChart from './Charts/DemoInstancesLineChart'

type Props = {
  className?: string
}

const InstancesContents: React.FC<Props> = ({ className }) => {

  return (
    <WidgetBig title='Instances' href='/instances' className={className}>
      <div className='w-full flex flex-wrap justify-center items-center gap-3'>
        <DemoInstancesLineChart />
      </div>
    </WidgetBig>
  )
}

export default InstancesContents