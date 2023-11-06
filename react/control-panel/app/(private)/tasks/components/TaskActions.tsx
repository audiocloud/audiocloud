import React, { Fragment } from 'react'
import Link from 'next/link'
import { Menu, Transition } from '@headlessui/react'
import { ChevronDownIcon } from '@heroicons/react/20/solid'
import classnames from 'classnames'
import { Task } from '@/types'

type Props = {
  task: Task
}

const TaskActions: React.FC<Props> = ({ task }) => {
  return (
    <Menu as="div" className="relative inline-block text-left">
      <div>
        <Menu.Button className="inline-flex w-full justify-center rounded-md border border-gray-300 hover:border-gray-400 active:border-gray-500 bg-gray-100 hover:bg-gray-200 active:bg-gray-300 px-2 py-2 text-sm text-gray-500 hover:text-gray-700 active:text-gray-900 shadow-sm">
          <ChevronDownIcon className="h-4 w-4" aria-hidden="false" />
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
        <Menu.Items className="absolute right-0 z-10 mt-2 w-56 origin-top-right rounded-md bg-white border border-gray-100 shadow-lg">
          <div className="py-1">
            <Menu.Item>
              {({ active }) => (
                <Link
                  href={`/tasks/${task.id}`}
                  className={classnames(
                    active ? 'bg-gray-100 active:bg-gray-200 text-gray-900' : 'text-gray-700',
                    'block px-4 py-2 text-sm'
                  )}
                >
                  Inspect
                </Link>
              )}
            </Menu.Item>
            <Menu.Item>
              {({ active }) => (
                <a
                  href="#"
                  className={classnames(
                    active ? 'bg-gray-100 active:bg-gray-200 text-gray-900' : 'text-gray-700',
                    'block px-4 py-2 text-sm'
                  )}
                >
                  Force play
                </a>
              )}
            </Menu.Item>
            <Menu.Item>
              {({ active }) => (
                <a
                  href="#"
                  className={classnames(
                    active ? 'bg-gray-100 active:bg-gray-200 text-gray-900' : 'text-gray-700',
                    'block px-4 py-2 text-sm'
                  )}
                >
                  Force stop
                </a>
              )}
            </Menu.Item>
            <Menu.Item>
              {({ active }) => (
                <a
                  href="#"
                  className={classnames(
                    active ? 'bg-gray-100 active:bg-gray-200 text-gray-900' : 'text-gray-700',
                    'block px-4 py-2 text-sm'
                  )}
                >
                  Manage API keys
                </a>
              )}
            </Menu.Item>
            <Menu.Item>
              {({ active }) => (
                <button
                  type="submit"
                  className={classnames(
                    active ? 'bg-gray-100 active:bg-gray-200 text-gray-900' : 'text-gray-700',
                    'block w-full px-4 py-2 text-left text-sm'
                  )}
                >
                  Delete from engine
                </button>
              )}
            </Menu.Item>
            <Menu.Item>
              {({ active }) => (
                <button
                  type="submit"
                  className={classnames(
                    active ? 'bg-gray-100 active:bg-gray-200 text-gray-900' : 'text-gray-700',
                    'block w-full px-4 py-2 text-left text-sm'
                  )}
                >
                  Delete from domain
                </button>
              )}
            </Menu.Item>
          </div>
        </Menu.Items>
      </Transition>
    </Menu>
  )
}

export default TaskActions