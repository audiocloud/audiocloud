import React from 'react'
import Link from 'next/link'
import classnames from 'classnames'
import { media } from '@/data/media'
import MediaActions from '../MediaActions'
import { getFileStatus } from '@/utils/media/fileStatuses'

const MediaTableDesktop: React.FC = () => {
  return (
    <div className="hidden sm:block">
      <div className="inline-block min-w-full align-middle"> {/* rounding corners does not work, overflow-hidden hides action dropdowns*/}
        <table>
          <thead className='border-b border-gray-200 text-gray-900 text-sm bg-gray-50'>
            <tr>
              <th scope="col" className="px-4 py-3 text-left whitespace-nowrap">Media ID</th>
              <th scope="col" className="px-4 py-3 text-left whitespace-nowrap">App ID</th>
              <th scope="col" className="hidden px-4 py-3 text-left whitespace-nowrap md:table-cell">Channels</th>
              <th scope="col" className="hidden px-4 py-3 text-left whitespace-nowrap xl:table-cell">Sample Rate</th>
              <th scope="col" className="hidden px-4 py-3 text-left whitespace-nowrap xl:table-cell">Bit Depth</th>
              <th scope="col" className="hidden px-4 py-3 text-left whitespace-nowrap md:table-cell">Length</th>
              <th scope="col" className="px-4 py-3 text-right whitespace-nowrap">Actions</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-100 bg-white">
            { Object.values(media).map((file) => (
              <tr key={file.id} className='hover:bg-gray-100 text-sm text-gray-500'>

                <td className="w-full max-w-0 whitespace-nowrap px-4 py-3 font-medium text-gray-900">
                  <div className="flex items-center space-x-3 truncate">
                    <div className={classnames('flex-shrink-0 w-2.5 h-2.5 rounded-full',
                      (getFileStatus(file) === 'download_complete' || getFileStatus(file) === 'upload_complete') && 'bg-emerald-600', 
                      (getFileStatus(file) === 'error' || getFileStatus(file) === 'its_complicated') && 'bg-pink-600',
                      (getFileStatus(file) === 'downloading' || getFileStatus(file) === 'uploading') && 'bg-amber-600',
                      getFileStatus(file) === 'unknown' && 'bg-gray-600')}
                    />
                    <Link href={`/media/${file.id}`} className="truncate hover:text-gray-600">
                      <span>{ file.id }</span>
                    </Link>
                  </div>
                </td>
                <td className="px-4 py-3 text-xs text-center font-medium whitespace-nowrap">                      { file.app_id }                                                                                           </td>
                <td className="hidden px-4 py-3 text-xs text-center font-medium whitespace-nowrap md:table-cell"> { file.metadata ?  <span>{ file.metadata.channels }</span> : '' }                                         </td>
                <td className="hidden px-4 py-3 text-xs text-center font-medium whitespace-nowrap xl:table-cell"> { file.metadata ?  <span>{( file.metadata?.sample_rate / 1000).toFixed(1) } kHz</span> : '' }             </td>
                <td className="hidden px-4 py-3 text-xs text-center font-medium whitespace-nowrap xl:table-cell"> { file.metadata ? <span>{ file.metadata?.bit_depth }-bit</span> : '' }                                    </td>
                <td className="hidden px-4 py-3 text-xs text-center font-medium whitespace-nowrap md:table-cell"> { file.metadata ? <span>{ file.metadata.length / 60 } min { file.metadata.length % 60 } sec</span> : '' } </td>
                <td className="whitespace-nowrap px-4 py-3 text-right">                                           <MediaActions media={file} />                                                                             </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  )
}

export default MediaTableDesktop