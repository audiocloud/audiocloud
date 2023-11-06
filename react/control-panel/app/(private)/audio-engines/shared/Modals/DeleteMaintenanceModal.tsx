'use client'

import React from 'react'
import { TrashIcon } from '@heroicons/react/24/outline'
import { Button } from '@/components/ui/button'
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from '@/components/ui/dialog'

type Props = {
  isOpen: boolean,
  setOpen: React.Dispatch<React.SetStateAction<boolean>>,
  engine_id: string
}

const DeleteMaintenanceModal: React.FC<Props> = ({ isOpen, setOpen, engine_id }) => {

  return (

    <Dialog open={isOpen} onOpenChange={(e) => setOpen(e)} >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Confirm Maintenance Delete</DialogTitle>
        </DialogHeader>

        <DialogDescription className='text-base'>
          Are you sure you want to delete this maintenance of <span className='font-semibold'>{ engine_id }</span>?
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
              console.log('Delete maintenance!')
              setOpen(false)
            }}
          >
            <TrashIcon className='w-5 h-5' aria-hidden="false" />
            <span>Delete maintenance</span>
          </Button>
        </div>

      </DialogContent>
    </Dialog>
  )
}

export default DeleteMaintenanceModal
