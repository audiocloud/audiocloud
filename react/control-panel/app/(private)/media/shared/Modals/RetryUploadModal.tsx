'use client'

import React from 'react'
import { ExclamationTriangleIcon } from '@heroicons/react/20/solid'
import { Button } from '@/components/ui/button'
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from '@/components/ui/dialog'

type Props = {
  media_id: string,
  isOpen: boolean,
  setOpen: React.Dispatch<React.SetStateAction<boolean>>
}

const RetryUploadModal: React.FC<Props> = ({ media_id, isOpen, setOpen }) => {
  return (
    <Dialog open={isOpen} onOpenChange={(e) => setOpen(e)} >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Confirm Upload Restart</DialogTitle>
        </DialogHeader>

        <DialogDescription className='text-base'>
          Are you sure you want to restart the upload of <span className='font-semibold'>{ media_id }</span>?
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
              console.log('Upload restart!')
              setOpen(false)
            }}
          >
            <ExclamationTriangleIcon className='w-5 h-5' aria-hidden="false" />
            <span>Restart upload</span>
          </Button>
        </div>

      </DialogContent>
    </Dialog>
  )
}

export default RetryUploadModal