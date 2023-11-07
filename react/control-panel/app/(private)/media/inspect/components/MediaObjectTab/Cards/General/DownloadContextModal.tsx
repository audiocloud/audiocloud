'use client'

import React, { useState } from 'react'
import { CheckIcon, XMarkIcon } from '@heroicons/react/20/solid'
import { PencilSquareIcon } from '@heroicons/react/24/outline'
import CodeMirror from '@uiw/react-codemirror'
import { vscodeDark } from '@uiw/codemirror-theme-vscode'
import { Button, buttonVariants } from '@/components/ui/button'
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger } from '@/components/ui/dialog'
import { cn } from '@/lib/utils'

type Props = {
  originalContext: string
}

const DownloadContextModal: React.FC<Props> = ({ originalContext }) => {

  const [code, setCode] = useState(originalContext)
  
  return (

    <Dialog>
      <DialogTrigger className={cn(buttonVariants({ variant: 'secondary', size: 'sm' }), 'w-20 flex justify-center items-center gap-2')}>
        <span>Edit</span>
        <PencilSquareIcon className='w-4 h-4' aria-hidden='false' />
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Download Context Editor</DialogTitle>
        </DialogHeader>
        <CodeMirror
          theme={vscodeDark}
          value={code}
          className='mt-1 rounded-lg overflow-hidden'
          onChange={(e: string) => setCode(e)}
          width='500px'
          height='200px'
        />
        <div className='pt-3 flex justify-center items-center gap-4'>
          <Button
            type='button'
            variant='objectActionButton'
            size='default'
            className='flex justify-center items-center gap-2'
            onClick={() => alert('Updating!')}
            disabled={originalContext === code}
          >
            <CheckIcon className='w-5 h-5' aria-hidden="false" />
            <span>Save</span>
          </Button>
          <Button
            type='button'
            variant='objectActionButton'
            size='default'
            className='flex justify-center items-center gap-2'
            onClick={() => setCode(originalContext)}
            disabled={originalContext === code}
          >
            <XMarkIcon className='w-5 h-5' aria-hidden="false" />
            <span>Reset</span>
          </Button>
        </div>

      </DialogContent>
    </Dialog>
  )
}

export default DownloadContextModal
