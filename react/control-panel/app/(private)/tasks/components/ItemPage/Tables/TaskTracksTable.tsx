import React from 'react'
import classnames from 'classnames'
import { ChevronRightIcon } from '@heroicons/react/20/solid'
import TaskTrackActions from '../TaskTrackActions'
import { TaskTrack } from '@/types'

type Props = {
  tracks: TaskTrack[]
}

const TaskTracksTable: React.FC<Props> = ({ tracks }) => {
  return (
    <>
      {/* MOBILE (only on smallest breakpoint) */}
      <div className="sm:hidden">

        <div className="px-4 sm:px-6 pt-3 border-t border-gray-200 flex justify-between">
          <h2 className="text-sm font-medium text-gray-900">Track ID</h2>
          <h2 className="text-sm font-medium text-gray-900 mr-10">Last seen</h2>
        </div>

        <ul role="list" className="mt-3 divide-y divide-gray-100 border-t border-gray-200">
          { tracks.map((track) => (
            <li key={track.id} className='hover:bg-gray-100'>
              <button className="w-full flex justify-between items-center px-4 py-4 sm:px-5">
                <div className="w-full flex justify-between items-center space-x-3 truncate">

                  <div className='flex justify-start items-center space-x-3 truncate'>
                    <span className={classnames('w-2.5 h-2.5 flex-shrink-0 rounded-full', true ? 'bg-emerald-600' : 'bg-pink-600')}/>
                    <span className="truncate text-sm font-medium leading-6">
                      { track.id }
                    </span>
                  </div>

                </div>
                <ChevronRightIcon className="ml-5 h-5 w-5 text-gray-400 group-hover:text-gray-500" aria-hidden="false" />
              </button>
            </li>
          ))}
        </ul>
      </div>

      {/* TABLET/DESKTOP (small breakpoint and up) */}
      <div className="hidden sm:block">
        <div className="inline-block min-w-full border-y border-gray-200 align-middle">
          <table className="min-w-full">
            <thead>
              <tr>

                <th scope="col" className="border-b border-gray-200 bg-gray-50 px-5 py-3 text-left text-sm font-semibold text-gray-900 whitespace-nowrap">
                  <span className="lg:pl-2">Track ID</span>
                </th>

                <th scope="col" className="border-b border-gray-200 bg-gray-50 py-3 pr-5 text-right text-sm font-semibold text-gray-900 whitespace-nowrap">
                  Actions
                </th>

              </tr>
            </thead>
            <tbody className="divide-y divide-gray-100 bg-white">
              { tracks.map((track) => (
                <tr key={track.id} className='hover:bg-gray-100'>

                  <td className="w-full max-w-0 whitespace-nowrap px-5 py-3 text-sm font-medium text-gray-900">
                    <div className="flex items-center space-x-3 lg:pl-2">
                      <div className={classnames('flex-shrink-0 w-2.5 h-2.5 rounded-full', true ? 'bg-emerald-600' : 'bg-pink-600')}/>
                      <span>{ track.id }</span>
                    </div>
                  </td>

                  <td className="whitespace-nowrap px-5 py-3 text-right text-sm">
                    <TaskTrackActions />
                  </td>

                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </>
  )
}

export default TaskTracksTable