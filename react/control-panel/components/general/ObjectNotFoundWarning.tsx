import React from 'react'
import { ExclamationTriangleIcon } from '@heroicons/react/24/outline'

type Props = {
  objectName: string
}

const ObjectNotFoundWarning: React.FC<Props> = ({ objectName }) => {
  return (
    <div className='flex justify-center items-center gap-3 text-lg'>
      <ExclamationTriangleIcon className='w-8 h-8' aria-hidden='false'/>
      <span>{ objectName } not found.</span>
    </div>
  )
}

export default ObjectNotFoundWarning