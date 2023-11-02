import React from 'react'
import Widget from '@/components/layout/page/widgets/small/WidgetSmall'

type Props = {
  className: string
}

const PlaceholderWidgetSmall: React.FC<Props> = ({ className }) => {
  return (
    <Widget title='Placeholder' className={className}>
      <div className='w-full h-40 flex justify-center items-center bg-slate-200 text-slate-400'>
        Empty
      </div>
    </Widget>
  )
}

export default PlaceholderWidgetSmall