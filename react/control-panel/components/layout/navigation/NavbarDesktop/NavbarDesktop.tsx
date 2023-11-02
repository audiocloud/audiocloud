import React from 'react'
import LogoHeader from './LogoHeader'
import UserAccountDropdown from './UserAccountDropdown'
import Search from './Search'
import PagesMenu from './PagesMenu'
import { ThemeDropdown } from '@/components/theme/ThemeDropdown'

const NavbarDesktop: React.FC = () => {

  return (
    <div className='hidden fixed w-64 pt-5 pb-4 lg:flex flex-col bg-primary-foreground border-r border-border inset-y-0'>

      <LogoHeader/>

      <div className='mt-5 flex h-0 flex-1 flex-col overflow-y-auto pt-1'>
        <UserAccountDropdown/>
        <Search/>
        <PagesMenu/>
      </div>

      <div className='w-full px-3'>
        <ThemeDropdown/>
      </div>
      
    </div>
  )
}

export default NavbarDesktop