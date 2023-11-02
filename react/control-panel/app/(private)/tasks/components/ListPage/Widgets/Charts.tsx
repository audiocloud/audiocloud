import React from 'react'
import Widget from '@/components/General/Widgets/Widget'

type Props = {
  className: string
}

const Charts: React.FC<Props> = ({ className }) => {
  return (
    <Widget title={'Charts'} className={className}>
      Empty
    </Widget>
  )
}

export default Charts