import React from 'react'
import Link from 'next/link'
import classnames from 'classnames'
import { tasks } from '@/data/tasks'
import { format } from 'date-fns'
import TaskActions from '../TaskActions'

const TasksTableDesktop: React.FC = () => {
  return (
    <div className="hidden sm:block">
      <div className="inline-block min-w-full align-middle"> {/* rounding corners does not work, overflow-hidden hides action dropdowns*/}
        <table>
          <thead className='border-b border-gray-200 text-gray-900 text-sm bg-gray-50'>
            <tr>
              <th scope="col" className="px-4 py-3 text-left whitespace-nowrap">Task ID</th>
              <th scope="col" className="px-4 py-3 text-left whitespace-nowrap">App ID</th>
              <th scope="col" className="hidden px-4 py-3 text-left whitespace-nowrap xl:table-cell">Start time</th>
              <th scope="col" className="hidden px-4 py-3 text-left whitespace-nowrap xl:table-cell">End time</th>
              <th scope="col" className="hidden px-4 py-3 text-left whitespace-nowrap 2xl:table-cell">Nodes</th>
              <th scope="col" className="hidden px-4 py-3 text-left whitespace-nowrap 2xl:table-cell">Mixers</th>
              <th scope="col" className="px-4 py-3 text-left whitespace-nowrap">Tracks</th>
              <th scope="col" className="px-4 py-3 text-right whitespace-nowrap">Actions</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-100 bg-white">
            { Object.values(tasks).map((task) => (
              <tr key={task.id} className='hover:bg-gray-100 text-sm text-gray-500'>
                <td className="w-full max-w-0 whitespace-nowrap px-4 py-3 font-medium text-gray-900">
                  <div className="flex items-center space-x-3 truncate">
                    <div className={classnames('flex-shrink-0 w-2.5 h-2.5 rounded-full', task.status !== 'error' ? 'bg-emerald-600' : 'bg-pink-600')} />
                    <Link href={`/tasks/${task.id}`} className="truncate hover:text-gray-600">
                      <span>
                        { task.id } <span className="font-normal text-gray-500 pl-2">({ task.status })</span>
                      </span>
                    </Link>
                  </div>
                </td>
                <td className="px-4 py-3 text-xs text-center font-medium whitespace-nowrap">                        { task.app_id }                                             </td>
                <td className="hidden px-4 py-3 text-xs text-center font-medium whitespace-nowrap xl:table-cell">   { format(new Date(task.start), 'dd-MM-yyyy @ HH:mm:ss') }   </td>
                <td className="hidden px-4 py-3 text-xs text-center font-medium whitespace-nowrap xl:table-cell">   { format(new Date(task.end), 'dd-MM-yyyy @ HH:mm:ss')}      </td>
                <td className="hidden px-4 py-3 text-xs text-center font-medium whitespace-nowrap 2xl:table-cell">  { task.nodes.length }                                       </td>
                <td className="hidden px-4 py-3 text-xs text-center font-medium whitespace-nowrap 2xl:table-cell">  { task.mixers.length }                                      </td>
                <td className="px-4 py-3 text-xs text-center font-medium whitespace-nowrap">                        { task.tracks.length }                                      </td>
                <td className="whitespace-nowrap px-4 py-3 text-right">                                             <TaskActions task={task} />                                 </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  )
}

export default TasksTableDesktop