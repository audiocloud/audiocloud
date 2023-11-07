'use client'

import React, { useState } from 'react'
import { CheckIcon, XMarkIcon } from '@heroicons/react/20/solid'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'

type Props = {
  value: string
}

const TextInput: React.FC<Props> = (props) => {

  const [value, setValue] = useState(props.value)

  return (
    <div className='flex justify-center items-center gap-0.5'>
      { props.value !== value && (
        <>
          <Button
            type='button'
            variant='objectActionButton'
            className='rounded-r-none'
            size='smallIcon'
            onClick={() => alert('Updating!')}
          >
            <CheckIcon className='w-5 h-5' aria-hidden="false" />
          </Button>
          <Button
            type='button'
            variant='objectActionButton'
            className='rounded-l-none'
            size='smallIcon'
            onClick={() => setValue(props.value)}
          >
            <XMarkIcon className='w-5 h-5' aria-hidden="false" />
          </Button>
        </>
      )}
      <Input
        type='text'
        className='ml-1 w-44'
        value={value}
        onChange={(e) => setValue(e.target.value)}
        // TO-DO: disable this while updating
        disabled={false}
      />
    </div>
  )
}

export default TextInput