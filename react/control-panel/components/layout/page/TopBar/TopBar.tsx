import React, { ReactNode } from 'react'
import BackButton from './BackButton'
import Titles from './Titles'
import RefreshDropdown from './RefreshDropdown'

type Props = {
  title: string,
  subtitle?: string,
  children?: ReactNode,
  backButton?: boolean
}

const TopBar: React.FC<Props> = ({ title, subtitle, children, backButton = false }) => {

  return (
    <div className='w-full h-16 px-4 flex justify-between items-center bg-primary-foreground border-b border-border'>

      <div className='flex justify-start items-center gap-4'>

        { backButton && <BackButton /> }
        
        <Titles title={title} subtitle={subtitle} />

        { children && <div className='flex justify-start items-center gap-2'>{ children }</div> }

      </div>

      <RefreshDropdown/>

    </div>
  )
}

export default TopBar