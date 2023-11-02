import React, { ReactNode } from 'react'
import NavbarDesktop from '@/components/layout/navigation/NavbarDesktop/NavbarDesktop'
import NavbarMobile from '@/components/layout/navigation/NavbarMobile/NavbarMobile'

type Props = {
  children: ReactNode
}

const MainLayout: React.FC<Props> = ({ children }) => {
  return (
    <div id='main-layout'>
      <NavbarDesktop />
      <NavbarMobile />
      <main id='page-container' className='h-screen lg:ml-64'>
        { children }
      </main>
    </div>
  )
}

export default MainLayout