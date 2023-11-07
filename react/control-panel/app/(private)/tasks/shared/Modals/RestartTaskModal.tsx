'use client'

import React from 'react'
import { ExclamationTriangleIcon } from '@heroicons/react/20/solid'
import { Button } from '@/components/ui/button'
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from '@/components/ui/dialog'

type Props = {
  isOpen: boolean,
  setOpen: React.Dispatch<React.SetStateAction<boolean>>,
  task_id: string
}

const RestartTaskModal: React.FC<Props> = ({ isOpen, setOpen, task_id }) => {

  return (

    <Dialog open={isOpen} onOpenChange={(e) => setOpen(e)} >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Confirm Task Restart</DialogTitle>
        </DialogHeader>

        <DialogDescription className='text-base'>
          Are you sure you want to restart task <span className='font-semibold'>{ task_id }</span>?
        </DialogDescription>
        
        <div className='pt-3 flex justify-center items-center gap-4'>
          <Button
            type='button'
            variant='objectActionButton'
            size='default'
            className='flex justify-center items-center gap-2'
            onClick={() => setOpen(false)}
          >
            <span>Cancel</span>
          </Button>
          <Button
            type='button'
            variant='warning'
            size='default'
            className='flex justify-center items-center gap-2'
            onClick={() => {
              console.log('Task restart!')
              setOpen(false)
            }}
          >
            <ExclamationTriangleIcon className='w-5 h-5' aria-hidden="false" />
            <span>Restart task</span>
          </Button>
        </div>

      </DialogContent>
    </Dialog>
  )
}

export default RestartTaskModal
