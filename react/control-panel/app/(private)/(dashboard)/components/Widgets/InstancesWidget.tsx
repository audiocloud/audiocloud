import React, { DetailedHTMLProps, HTMLAttributes } from 'react'
import DemoInstancesLineChart from './Charts/DemoInstancesLineChart'
import DashboardWidget from '../DashboardWidget'

const InstancesContents: React.FC<DetailedHTMLProps<HTMLAttributes<HTMLDivElement>, HTMLDivElement>> = ({ className }) => {

  return (
    <DashboardWidget title='Instances' href='/instances' className={className}>
      <div className='w-full flex flex-wrap justify-center items-center gap-3'>
        <DemoInstancesLineChart />
      </div>
    </DashboardWidget>
  )
}

export default InstancesContents