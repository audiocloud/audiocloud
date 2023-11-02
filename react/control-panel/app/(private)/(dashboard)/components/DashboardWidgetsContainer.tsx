import React, { DetailedHTMLProps, HTMLAttributes } from 'react'

const DashboardWidgetsContainer: React.FC<DetailedHTMLProps<HTMLAttributes<HTMLDivElement>, HTMLDivElement>> = ({ children }) => {
  return (
    <div className='w-full flex flex-wrap justify-start items-start divide-x-[1px] divide-slate-300 border-b-[1px] border-slate-300'>
      { children }
    </div>
  )
}

export default DashboardWidgetsContainer