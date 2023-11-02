'use client'

import React, { Fragment, useState } from 'react'
import { Dialog, Transition } from '@headlessui/react'
import LogoHeader from './LogoHeader'
import PagesMenu from './PagesMenu'
import Search from './Search'
import UserAccountDropDown from './UserAccountDropDown'
import OpenSidebarButton from './OpenSidebarButton'
import CloseSidebarButton from './CloseSidebarButton'

const NavbarMobile: React.FC = () => {

  const [sidebarOpen, setSidebarOpen] = useState(false)

  return (
    <>
      {/* Sticky Top */}
      <div className="sticky lg:hidden top-0 z-10 flex h-16 flex-shrink-0 bg-primary-foreground border-b border-border">

        <OpenSidebarButton setSidebarOpen={setSidebarOpen} />

        <div className="flex flex-1 justify-between px-4 sm:px-6 lg:px-8">
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
            <div className="fixed inset-0 bg-gray-600 bg-opacity-75" />
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
              <Dialog.Panel className="relative flex w-full max-w-xs flex-1 flex-col bg-white pt-5 pb-4">
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