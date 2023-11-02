import React from 'react'

type Props = {
  subject: string,
  status: string,
  extra_info: string,
  buttons: JSX.Element[]
}

const Alert: React.FC<Props> = ({ subject, status, extra_info, buttons }) => {
  return (
    <div className='w-full flex justify-between items-center border border-red-400 bg-red-200 text-red-800 text-xs pl-2 pr-1 py-1 rounded-md'>
      <div className='flex justify-start items-center gap-3'>
        <div className='font-medium whitespace-nowrap'>{ subject }</div>
        <div className='truncate whitespace-nowrap uppercase'>{ status }</div>
      </div>
      <div className='flex justify-end items-center gap-3'>
        <div className='truncate whitespace-nowrap'>{ extra_info }</div>
        <div className='flex justify-center items-center gap-1'>
          { buttons }
        </div>
      </div>
    </div>
  )
}

export default Alert