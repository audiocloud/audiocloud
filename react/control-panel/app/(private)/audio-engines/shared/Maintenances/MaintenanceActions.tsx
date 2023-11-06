'use client'

import React, { useState } from 'react'
import { Button } from '@/components/ui/button'
import { IAudioEngineMaintenance } from '@/types'
import EditMaintenanceModal from '../Modals/EditMaintenance/EditMaintenanceModal'
import ExtendMaintenanceModal from '../Modals/ExtendMaintenance/ExtendMaintenanceModal'
import EndMaintenanceModal from '../Modals/EndMaintenanceModal'

type Props = {
  maintenance: IAudioEngineMaintenance
}

const MaintenanceActions: React.FC<Props> = ({ maintenance }) => {

  const [editModal, setEditModal] = useState(false)
  const [extendModal, setExtendModal] = useState(false)
  const [endModal, setEndModal] = useState(false)
  
  return (
    <>
      <div className='hidden group-hover/row:flex flex-col xl:flex-row justify-center items-end gap-2'>
        <Button size='sm' variant='tableButton' onClick={() => setEditModal(true)}>Edit</Button>
        <Button size='sm' variant='tableButton' onClick={() => setExtendModal(true)}>Extend</Button>
        <Button size='sm' variant='tableButton' onClick={() => setEndModal(true)}>End</Button>
      </div>

      <EditMaintenanceModal
        maintenance={maintenance}
        isOpen={editModal}
        setOpen={setEditModal}
      />
      <ExtendMaintenanceModal
        maintenance={maintenance}
        isOpen={extendModal}
        setOpen={setExtendModal}
      />
      <EndMaintenanceModal
        engine_id={maintenance.engine_id}
        isOpen={endModal}
        setOpen={setEndModal}
      />
    </>
  )
}

export default MaintenanceActions