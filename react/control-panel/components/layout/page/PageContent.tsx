import React, { ReactNode } from 'react'

type Props = {
  children: ReactNode
}
const PageContent: React.FC<Props> = ({ children }) => {
  return (
    <div className=''>
      { children }
    </div>
  )
}

export default PageContent