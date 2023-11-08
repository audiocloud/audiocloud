import React, { ReactNode } from 'react'

type Props = {
  children: ReactNode
}

const WidgetsContainerBig: React.FC<Props> = ({ children }) => {
  return (
    <div className='w-full flex flex-wrap justify-start items-start'>
      { children }
    </div>
  )
}

export default WidgetsContainerBig