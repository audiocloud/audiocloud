'use client'

import React, { Fragment, useState } from 'react'
import { Dialog, Transition } from '@headlessui/react'
import LogoHeader from './LogoHeader'
import PagesMenu from './PagesMenu'
import Search from './Search'
import UserAccountDropDown from './UserAccountDropDown'
import OpenSidebarButton from './OpenSidebarButton'
import CloseSidebarButton from './CloseSidebarButton'
import { ThemeDropdown } from '@/components/theme/ThemeDropdown'

const NavbarMobile: React.FC = () => {

  const [sidebarOpen, setSidebarOpen] = useState(false)

  return (
    <>
      {/* Sticky Top */}
      <div className="sticky lg:hidden top-0 z-10 h-16 flex items-center flex-shrink-0 bg-midground border-b border-border">

        <OpenSidebarButton setSidebarOpen={setSidebarOpen} />

        <div className="px-3 flex flex-1 justify-between items-center gap-2">
          <Search/>
          <UserAccountDropDown/>
        </div>
        
      </div>

      {/* Sidebar */}
      <Transition.Root show={sidebarOpen} as={Fragment}>
        <Dialog as="div" className="relative z-40 lg:hidden" onClose={setSidebarOpen}>
          <Transition.Child
            as={Fragment}
            enter="transition-opacity ease-linear duration-300"
            enterFrom="opacity-0"
            enterTo="opacity-100"
            leave="transition-opacity ease-linear duration-300"
            leaveFrom="opacity-100"
            leaveTo="opacity-0"
          >
            <div className="fixed inset-0 bg-background/50" />
          </Transition.Child>

          <div className="fixed inset-0 z-40 flex">
            <Transition.Child
              as={Fragment}
              enter="transition ease-in-out duration-300 transform"
              enterFrom="-translate-x-full"
              enterTo="translate-x-0"
              leave="transition ease-in-out duration-300 transform"
              leaveFrom="translate-x-0"
              leaveTo="-translate-x-full"
            >
              <Dialog.Panel className="relative w-full max-w-xs pt-5 pb-3 px-3 flex flex-1 flex-col gap-3 bg-midground border-r border-border">
                <Transition.Child
                  as={Fragment}
                  enter="ease-in-out duration-300"
                  enterFrom="opacity-0"
                  enterTo="opacity-100"
                  leave="ease-in-out duration-300"
                  leaveFrom="opacity-100"
                  leaveTo="opacity-0"
                >
                  <div className="absolute top-0 right-0 -mr-12 pt-2">
                    <CloseSidebarButton setSidebarOpen={setSidebarOpen} />
                  </div>
                </Transition.Child>

                <LogoHeader/>
                <PagesMenu/>
                <div className='w-full mt-0 pt-2 border-t border-border'>
                  <ThemeDropdown />
                </div>

              </Dialog.Panel>
            </Transition.Child>
            <div className="w-14 flex-shrink-0" aria-hidden="false">
              {/* Dummy element to force sidebar to shrink to fit close icon */}
            </div>
          </div>
        </Dialog>
      </Transition.Root>
    </>
  )
}

export default NavbarMobile