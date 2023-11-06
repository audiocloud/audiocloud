'use client'

import React from 'react'
import { ExclamationTriangleIcon } from '@heroicons/react/20/solid'
import { Button } from '@/components/ui/button'
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from '@/components/ui/dialog'

type Props = {
  isOpen: boolean,
  setOpen: React.Dispatch<React.SetStateAction<boolean>>,
  instance_id: string,
  handlePower: (value: boolean) => void
}

const ForceShutdownInstanceModal: React.FC<Props> = ({ isOpen, setOpen, instance_id, handlePower }) => {

  return (

    <Dialog open={isOpen} onOpenChange={(e) => setOpen(e)} >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Confirm Force Shutdown</DialogTitle>
        </DialogHeader>

        <DialogDescription className='text-base'>
          Are you sure you want to force shutdown <span className='font-semibold'>{ instance_id }</span>?
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
            variant='destructive'
            size='default'
            className='flex justify-center items-center gap-2'
            onClick={() => {
              console.log('Force shutdown!')
              handlePower(false)
              setOpen(false)
            }}
          >
            <ExclamationTriangleIcon className='w-5 h-5' aria-hidden="false" />
            <span>Shutdown</span>
          </Button>
        </div>

      </DialogContent>
    </Dialog>
  )
}

export default ForceShutdownInstanceModal
