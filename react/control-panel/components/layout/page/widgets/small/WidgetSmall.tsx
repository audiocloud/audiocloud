import React, { ReactNode } from 'react'
import clsx from 'clsx'

type Props = {
  title: string,
  className?: string,
  children: ReactNode
}

const WidgetSmall: React.FC<Props> = ({ title, className, children }) => {
  return (
    <div className={clsx(className,
      'w-full h-fit border-slate-300', // should always fit, if it doesnt, get this from className prop
      'flex flex-col justify-start items-center gap-3 p-4 overflow-y-auto'
      )}>

      <h2 className="text-xl font-extrabold text-slate-600 hover:text-slate-700">{ title }</h2>
      <div className='w-full'>{ children }</div>

    </div>
  )
}

export default WidgetSmall