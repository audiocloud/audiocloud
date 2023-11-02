'use client'

import React, { Fragment } from 'react'
import Link from 'next/link'
import clsx from 'clsx'
import { Menu, Transition } from '@headlessui/react'
import { ChevronUpDownIcon } from '@heroicons/react/20/solid'
import useAudioCloudAuth from '@/hooks/useAudioCloudAuth'

const UserAccountDropdown: React.FC = () => {

  const { id, logout } = useAudioCloudAuth()

  return (
    <Menu as="div" className="relative inline-block px-3 text-left">
      <Menu.Button className="group w-full rounded-md pl-3 pr-2 py-2 text-left text-sm text-slate-400 bg-slate-900 hover:bg-slate-800 active:bg-slate-700">
        <span className="w-full flex justify-between items-center">
          <span className="truncate">Logged in as: <span className="font-bold text-slate-300">{ id || 'placeholder'}</span></span>
          <ChevronUpDownIcon className="h-5 w-5 flex-shrink-0 text-slate-400 group-hover:text-slate-500" aria-hidden="false" />
        </span>
      </Menu.Button>

      <Transition
        as={Fragment}
        enter="transition ease-out duration-100"
        enterFrom="transform opacity-0 scale-95"
        enterTo="transform opacity-100 scale-100"
        leave="transition ease-in duration-75"
        leaveFrom="transform opacity-100 scale-100"
        leaveTo="transform opacity-0 scale-95"
      >
        <Menu.Items className="absolute right-0 left-0 z-10 mx-3 mt-1 origin-top divide-y divide-slate-200 rounded-md bg-slate-800 border border-slate-700 shadow-lg">
          <div className="py-1">
            <Menu.Item>
              {({ active }) => (
                <Link
                  href="/settings"
                  className={clsx('block px-4 py-2 text-sm', active ? 'bg-slate-700 active:bg-slate-600 text-slate-200' : 'text-slate-400')}
                >
                  Settings
                </Link>
              )}
            </Menu.Item>
            <Menu.Item>
              {({ active }) => (
                <button
                  type='button'
                  className={clsx('block px-4 py-2 text-sm w-full text-left', active ? 'bg-slate-700 active:bg-slate-600 text-slate-200' : 'text-slate-400')}
                  onClick={() => logout()}
                >
                  Logout
                </button>
              )}
            </Menu.Item>
          </div>
        </Menu.Items>
      </Transition>
    </Menu>
  )
}

export default UserAccountDropdown