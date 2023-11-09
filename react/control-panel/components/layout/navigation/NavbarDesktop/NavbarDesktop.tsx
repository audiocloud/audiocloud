import React from 'react'
import LogoHeader from './LogoHeader'
import UserAccountDropdown from './UserAccountDropdown'
import Search from './Search'
import PagesMenu from './PagesMenu'
import { ThemeDropdown } from '@/components/theme/ThemeDropdown'

const NavbarDesktop: React.FC = () => {

  return (
    <div className='hidden fixed w-64 pt-5 pb-4 lg:flex flex-col justify-between gap-2 bg-midground border-r border-border inset-y-0 overflow-y-auto'>

      <div className='w-full px-3'>

        <LogoHeader/>

        <div className='w-full mt-5 flex flex-col justify-center items-center gap-3'>
          <UserAccountDropdown/>
          <Search/>
          <PagesMenu/>
        </div>

      </div>

      <div className='w-full px-3'>
        <ThemeDropdown/>
      </div>
      
    </div>
  )
}

export default NavbarDesktop