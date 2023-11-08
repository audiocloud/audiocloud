import React, { ReactNode } from 'react'
import Link from 'next/link'
import clsx from 'clsx'

type Props = {
  title: string,
  href: string,
  titleRowItems: ReactNode,
  children: ReactNode
}

const WidgetBig: React.FC<Props> = ({ title, href, titleRowItems, children }) => {
  return (
    <div className={clsx('w-full xl:w-1/2 h-[398px] flex flex-col justify-start items-center overflow-hidden')}>

      <div className='w-full h-20 pt-4 px-4 flex justify-between items-end'>

        <Link href={href} className='text-3xl font-bold hover:underline upper'>{ title }</Link>

        <div className='flex justify-center items-center gap-6'>
          { titleRowItems }
        </div>

      </div>

      <div className='w-full'>
        { children }
      </div>

    </div>
  )
}

export default WidgetBig