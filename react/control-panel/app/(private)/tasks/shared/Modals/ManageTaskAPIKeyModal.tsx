'use client'

import React, { useEffect, useState } from 'react'
import { ExclamationTriangleIcon } from '@heroicons/react/24/outline'
import { Button } from '@/components/ui/button'
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'

type Props = {
  isOpen: boolean,
  setOpen: React.Dispatch<React.SetStateAction<boolean>>,
  task_id: string,
  current_api_key: string
}

const ManageTaskAPIKeyModal: React.FC<Props> = ({ isOpen, setOpen, task_id, current_api_key }) => {

  const [newAPIKey, setNewAPIKey] = useState(current_api_key)

  // TO-DO: update API key call

  useEffect(() => {
    setNewAPIKey(current_api_key)
  }, [isOpen])

  return (

    <Dialog open={isOpen} onOpenChange={(e) => setOpen(e)} >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Manage Task API Key</DialogTitle>
        </DialogHeader>

        <DialogDescription className='text-base'>
          <Input value={newAPIKey} onChange={(e) => setNewAPIKey(e.target.value)}/>
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
              console.log('Change API key!')
              setOpen(false)
            }}
            disabled={newAPIKey === current_api_key}
          >
            <ExclamationTriangleIcon className='w-5 h-5' aria-hidden="false" />
            <span>Change API key</span>
          </Button>
        </div>

      </DialogContent>
    </Dialog>
  )
}

export default ManageTaskAPIKeyModal