import React from 'react'
import { LineChart } from "@tremor/react"

type Props = {
  //
}


const chartdata = [
  {
    timestamp: '15:00',
    'Successful': 64,
    'Failed': 5,
  },
  {
    timestamp: '15:15',
    'Successful': 45,
    'Failed': 2,
  },
  {
    timestamp: '15:30',
    'Successful': 73,
    'Failed': 6,
  },
  {
    timestamp: '15:45',
    'Successful': 46,
    'Failed': 1,
  },
  {
    timestamp: '16:00',
    'Successful': 88,
    'Failed': 8,
  }
]

const dataFormatter = (number: number) =>
  `${Intl.NumberFormat("us").format(number).toString()}`;

const DemoTasksLineChart: React.FC<Props> = () => (
  <div className='w-full'>
    <LineChart
      className=""
      data={ chartdata }
      index="timestamp"
      categories={ ['Successful', 'Failed'] }
      colors={ ['emerald', 'red'] }
      valueFormatter={ dataFormatter }
      yAxisWidth={ 40 }

    />
  </div>
)

export default DemoTasksLineChart