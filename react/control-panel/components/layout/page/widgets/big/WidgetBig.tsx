import React, { ReactNode } from 'react'
import Link from 'next/link'
import { ChevronRightIcon } from '@heroicons/react/24/outline'
import clsx from 'clsx'

type Props = {
  title: string,
  href: string,
  className?: string,
  children: ReactNode
}

const WidgetBig: React.FC<Props> = ({ title, href, className, children }) => {
  return (
    <div className={clsx(className,
      'w-full xl:w-1/2 h-fit',
      'flex flex-col justify-start items-center p-3'
    )}>
      
      <Link href={href} className='relative w-full flex justify-center items-center rounded-lg p-2 mb-4 text-slate-500 hover:text-slate-600 hover:bg-slate-100 active:bg-slate-200'>
        <h2 className="text-xl font-extrabold text-slate-600 hover:text-slate-700">{ title }</h2>
        <ChevronRightIcon className='w-5 h-5 right-2 absolute' aria-hidden='false' />
      </Link>
      
      <div className='w-full'>{ children }</div>

    </div>
  )
}

export default WidgetBig