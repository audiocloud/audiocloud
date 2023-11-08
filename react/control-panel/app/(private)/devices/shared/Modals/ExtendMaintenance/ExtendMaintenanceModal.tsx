'use client'

import React from 'react'
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { IDeviceMaintenance } from '@/types'
import ExtendMaintenanceForm from './ExtendMaintenanceForm'

type Props = {
  maintenance: IDeviceMaintenance,
  isOpen: boolean,
  setOpen: React.Dispatch<React.SetStateAction<boolean>>
}

const ExtendMaintenanceModal: React.FC<Props> = ({ maintenance, isOpen, setOpen }) => {

  return (
    <Dialog open={isOpen} onOpenChange={(e) => setOpen(e)} >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Extend Maintenance</DialogTitle>
        </DialogHeader>

        <ExtendMaintenanceForm maintenance={maintenance} setOpen={setOpen} />

      </DialogContent>
    </Dialog>
  )
}

export default ExtendMaintenanceModal