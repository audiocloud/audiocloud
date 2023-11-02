import React, { useState } from 'react'
import classnames from 'classnames'
import { PencilSquareIcon, CheckIcon } from '@heroicons/react/20/solid'

type Props = {
  textValue: string
}

const TextInputWithEditButton: React.FC<Props> = ({ textValue }) => {

  const [editLock, setEditLock] = useState(true)
  const [value, setValue] = useState(textValue)

  return (
    <div className='flex items-center'>
      { editLock ? (
        <button
          type='button'
          className='w-8 h-8 flex justify-center items-center mr-1 rounded-md border border-slate-300 hover:border-slate-400 active:border-slate-500 bg-slate-200 hover:bg-slate-300 active:bg-slate-400'
          onClick={() => setEditLock(!editLock)}
        >
          <PencilSquareIcon className='w-5 h-5' aria-hidden="false" />
        </button>
      ) : (
        <button
          type='button'
          className='w-8 h-8 flex justify-center items-center mr-1 rounded-md border border-slate-300 hover:border-slate-400 active:border-slate-500 bg-slate-200 hover:bg-slate-300 active:bg-slate-400'
          onClick={() => setEditLock(!editLock)}
        >
          <CheckIcon className='w-5 h-5' aria-hidden="false" />
        </button>
      )}
      <input
        type='text'
        className={classnames('w-40 h-8 text-sm truncate rounded-md border bg-slate-200', editLock ? 'text-gray-400 border-slate-300 focus:bg-slate-200' : 'text-gray-600 border-slate-400 focus:bg-slate-200')}
        defaultValue={ textValue }
        value={value}
        onChange={(e) => setValue(e.target.value)}
        disabled={editLock}
      />
    </div>
  )
}

export default TextInputWithEditButton