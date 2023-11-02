import React, { ReactNode } from 'react'

type Props = {
  children: ReactNode
}

const WidgetsContainerSmall: React.FC<Props> = ({ children }) => {
  return (
    <div className='w-full flex flex-wrap justify-start items-start divide-x-[1px] divide-slate-300 border-b-[1px] border-slate-300'>
      { children }
    </div>
  )
}

export default WidgetsContainerSmall