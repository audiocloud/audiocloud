import React from 'react'
import classnames from 'classnames'
import { ChevronRightIcon } from '@heroicons/react/20/solid'
import { tasks } from '@/data/tasks'
import { format } from 'date-fns'

const TasksTableMobile: React.FC = () => {
  return (
    <div className="sm:hidden">

      <div className="p-3 bg-gray-50 border border-slate-300 border-b-gray-200 rounded-t-md flex justify-between text-sm font-medium text-gray-900">
        <h2 className="">Task ID</h2>
        <h2 className="mr-10">Start time</h2>
      </div>

      <ul role="list" className="divide-y divide-gray-100 border-b border-x border-slate-300 bg-white rounded-b-md">
        { Object.values(tasks).map((task) => (
          <li key={task.id} className='hover:bg-gray-100'>
            <button type='button' className="w-full flex justify-between items-center px-4 py-4 sm:px-5">
              <div className="w-full flex justify-between items-center space-x-3 truncate">

                <div className='flex justify-start items-center space-x-3 truncate'>
                  <span className={classnames('w-2.5 h-2.5 flex-shrink-0 rounded-full', task.status !== 'error' ? 'bg-emerald-600' : 'bg-pink-600')}/>
                  <span className="truncate text-sm font-medium leading-6">
                    { task.id } <span className="truncate font-normal text-gray-500 pl-2">({ task.status })</span>
                  </span>

                  <span className="truncate text-xs font-medium leading-6 text-gray-500">
                    { task.app_id }
                  </span>
                </div>

                <div className="truncate text-xs font-medium leading-6 text-gray-500">
                  { format(new Date(task.start), 'dd-MM-yyyy @ HH:mm:ss') }
                </div>

              </div>
              <ChevronRightIcon className="ml-5 h-5 w-5 text-gray-400 group-hover:text-gray-500" aria-hidden="false" />
            </button>
          </li>
        ))}
      </ul>
    </div>
  )
}

export default TasksTableMobile