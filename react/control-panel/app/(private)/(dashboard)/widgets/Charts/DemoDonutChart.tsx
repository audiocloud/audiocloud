'use client'

import React from 'react'
import { Title, DonutChart } from "@tremor/react"

const data = [
  { name: "Full", value: 440 },
  { name: "Free", value: 560 }
]


type Props = {
  //
}

const DemoDonutChart: React.FC<Props> = () => {

  const valueFormatter = (number: number) => `${Intl.NumberFormat("us").format(number).toString()} GB`

  return (
    <div className='w-full flex flex-col justify-center items-center'>
      <Title>Disk space</Title>
      <DonutChart
        className="mt-3"
        data={data}
        category="value"
        index="name"
        valueFormatter={valueFormatter}
        colors={["violet", "emerald"]}
      />
    </div>
  )
}

export default DemoDonutChart