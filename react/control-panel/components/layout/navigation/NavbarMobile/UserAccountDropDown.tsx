import React, { Fragment } from 'react'
import Link from 'next/link'
import clsx from 'clsx'
import { Menu, Transition } from '@headlessui/react'
import { ChevronUpDownIcon } from '@heroicons/react/20/solid'
import useAudioCloudAuth from '@/hooks/useAudioCloudAuth'

const UserAccountDropDown: React.FC = () => {

  const { id, email, logout } = useAudioCloudAuth()

  return (
    <div className="flex items-center">
      <Menu as="div" className="relative ml-3">
        <div>
          <Menu.Button className="pl-3 pr-2 py-1 flex max-w-xs items-center bg-primary-foreground hover:bg-background active:bg-background rounded-lg">
            <span className="flex w-full items-center justify-between">
              <span className="flex min-w-0 items-center justify-between space-x-3">
                <span className="flex min-w-0 flex-1 flex-col text-right mr-2">
                  <span className="truncate text-sm font-medium">{ email || 'placeholder@holder.com' }</span>
                  <span className="truncate text-sm text-gray-500">ID: { id || 'placeholder_id' }</span>
                </span>
              </span>
              <ChevronUpDownIcon
                className="h-5 w-5 flex-shrink-0 text-gray-400 group-hover:text-gray-500"
                aria-hidden="false"
              />
            </span>
          </Menu.Button>
        </div>
        <Transition
          as={Fragment}
          enter="transition ease-out duration-100"
          enterFrom="transform opacity-0 scale-95"
          enterTo="transform opacity-100 scale-100"
          leave="transition ease-in duration-75"
          leaveFrom="transform opacity-100 scale-100"
          leaveTo="transform opacity-0 scale-95"
        >
          <Menu.Items className="absolute right-0 z-10 mt-2 w-48 origin-top-right divide-y divide-gray-200 rounded-md bg-white shadow-lg">
            <div className="py-1">
              <Menu.Item>
                {({ active }) => (
                  <Link
                    href="/settings"
                    className={clsx(
                      active ? 'bg-gray-100 text-gray-900' : 'text-gray-700',
                      'block px-4 py-2 text-sm'
                    )}
                  >
                    Settings
                  </Link>
                )}
              </Menu.Item>
              <Menu.Item>
                {({ active }) => (
                  <button
                    type='button'
                    onClick={() => logout()}
                    className={clsx(
                      active ? 'bg-gray-100 text-gray-900' : 'text-gray-700',
                      'block px-4 py-2 text-sm'
                    )}
                  >
                    Logout
                  </button>
                )}
              </Menu.Item>
            </div>
          </Menu.Items>
        </Transition>
      </Menu>
    </div>
  )
}

export default UserAccountDropDown