'use client'

import React from 'react'
import { LineChart } from "@tremor/react"

type Props = {
  //
}


const chartdata = [
  {
    timestamp: '15:00',
    'Online': 0,
    'Offline': 17,
  },
  {
    timestamp: '15:15',
    'Online': 17,
    'Offline': 0,
  },
  {
    timestamp: '15:30',
    'Online': 17,
    'Offline': 1,
  },
  {
    timestamp: '15:45',
    'Online': 17,
    'Offline': 0,
  },
  {
    timestamp: '16:00',
    'Online': 17,
    'Offline': 0,
  }
]

const dataFormatter = (number: number) =>
  `${Intl.NumberFormat("us").format(number).toString()}`;

const DemoInstancesLineChart: React.FC<Props> = () => (
  <div className='w-full'>
    <LineChart
      className=""
      data={ chartdata }
      index="timestamp"
      categories={ ['Online', 'Offline'] }
      colors={ ['emerald', 'red'] }
      valueFormatter={ dataFormatter }
      yAxisWidth={ 40 }
    />
  </div>
)

export default DemoInstancesLineChart