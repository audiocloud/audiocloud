import React, { useState } from 'react'
import { ChevronLeftIcon, ChevronDownIcon } from '@heroicons/react/24/outline'
import JSONPretty from 'react-json-pretty'

type Props = {
  data: any
}

const JSONDisplay: React.FC<Props> = ({ data }) => {

  const [show, setShow] = useState(false)
  
  return (
    <>
      <div className='mt-10 m-2 font-medium text-lg flex items-center gap-2'>
        <span>JSON</span>
        <button
          type='button'
          className='rounded-sm border border-slate-300 hover:border-slate-400 hover:bg-slate-300 active:bg-slate-400 p-1'
          onClick={() => setShow(!show)}
        >
          { show
            ? <ChevronDownIcon className='h-3 w-3'/>
            : <ChevronLeftIcon className='h-3 w-3'/>
          }
        </button>
      </div>
      { show && (
        <div className='m-2 p-3 bg-slate-800 text-gray-200 rounded-md'>
          <JSONPretty id="json-pretty" className='text-xs' data={data}></JSONPretty>
        </div>
      )}
    </>
    
  )
}

export default JSONDisplay