import { cn } from '@/lib/utils'
import React, { ReactNode } from 'react'

type Props = {
  label: string,
  item: ReactNode,
  units?: string,
  itemsStart?: boolean
}

const CardLine: React.FC<Props> = ({ label, item, units, itemsStart = false }) => {
  return (
    <div className={cn('w-full flex justify-between gap-3', itemsStart ? 'items-start' : 'items-center')}>
      <span className='text-slate-400 whitespace-nowrap'>{ label }</span>
      <span className='text-primary flex justify-start items-center gap-2'>
        { item }
        { units !== undefined && <span className='w-7 truncate'>{ units }</span> }
      </span>
    </div>
  )
}

export default CardLine