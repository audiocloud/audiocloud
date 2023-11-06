'use client'

import React from 'react'
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { IInstanceMaintenance } from '@/types'
import EditMaintenanceForm from './EditMaintenanceForm'

type Props = {
  maintenance: IInstanceMaintenance,
  isOpen: boolean,
  setOpen: React.Dispatch<React.SetStateAction<boolean>>
}

const EditMaintenanceModal: React.FC<Props> = ({ maintenance, isOpen, setOpen }) => {

  return (
    <Dialog open={isOpen} onOpenChange={(e) => setOpen(e)} >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Edit Maintenance</DialogTitle>
        </DialogHeader>

        <EditMaintenanceForm maintenance={maintenance} setOpen={setOpen} />

      </DialogContent>
    </Dialog>
  )
}

export default EditMaintenanceModal