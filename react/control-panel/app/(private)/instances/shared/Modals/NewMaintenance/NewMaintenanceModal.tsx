'use client'

import React from 'react'
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import NewMaintenanceForm from './NewMaintenanceForm'

type Props = {
  instance_id: string,
  isOpen: boolean,
  setOpen: React.Dispatch<React.SetStateAction<boolean>>
}

const NewMaintenanceModal: React.FC<Props> = ({ instance_id, isOpen, setOpen }) => {

  return (
    <Dialog open={isOpen} onOpenChange={(e) => setOpen(e)} >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>New Maintenance</DialogTitle>
        </DialogHeader>

        <NewMaintenanceForm instance_id={instance_id} setOpen={setOpen} />

      </DialogContent>
    </Dialog>
  )
}

export default NewMaintenanceModal