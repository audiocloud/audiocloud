'use client'

import React from 'react'
import { TrashIcon } from '@heroicons/react/24/outline'
import { Button } from '@/components/ui/button'
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from '@/components/ui/dialog'

type Props = {
  isOpen: boolean,
  setOpen: React.Dispatch<React.SetStateAction<boolean>>,
  task_node_id: string
}

const DeleteTaskModal: React.FC<Props> = ({ isOpen, setOpen, task_node_id }) => {

  return (

    <Dialog open={isOpen} onOpenChange={(e) => setOpen(e)} >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Confirm Task Node Delete</DialogTitle>
        </DialogHeader>

        <DialogDescription className='text-base'>
          Are you sure you want to delete task node <span className='font-semibold'>{ task_node_id }</span>?
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
              console.log('Delete task node!')
              setOpen(false)
            }}
          >
            <TrashIcon className='w-5 h-5' aria-hidden="false" />
            <span>Delete task node</span>
          </Button>
        </div>

      </DialogContent>
    </Dialog>
  )
}

export default DeleteTaskModal
