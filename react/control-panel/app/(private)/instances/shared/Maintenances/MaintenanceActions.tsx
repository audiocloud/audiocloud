'use client'

import React, { useState } from 'react'
import { isBefore } from 'date-fns'
import { Button } from '@/components/ui/button'
import { IInstanceMaintenance } from '@/types'
import EditMaintenanceModal from '../Modals/EditMaintenance/EditMaintenanceModal'
import ExtendMaintenanceModal from '../Modals/ExtendMaintenance/ExtendMaintenanceModal'
import DeleteMaintenanceModal from '../Modals/DeleteMaintenanceModal'
import EndMaintenanceModal from '../Modals/EndMaintenanceModal'

type Props = {
  maintenance: IInstanceMaintenance
}

const MaintenanceActions: React.FC<Props> = ({ maintenance }) => {
  
  const [editModal, setEditModal] = useState(false)
  const [extendModal, setExtendModal] = useState(false)
  const [deleteModal, setDeleteModal] = useState(false)
  const [endModal, setEndModal] = useState(false)

  return (
    <>
      <div className='hidden group-hover/row:flex flex-col xl:flex-row justify-center items-end gap-2'>

        <Button size='sm' variant='tableButton' onClick={() => setEditModal(true)}>Edit</Button>

        { !!maintenance.data.end
          && isBefore(new Date(maintenance.data.end), new Date())
          && <Button size='sm' variant='tableButton' onClick={() => setExtendModal(true)}>Extend</Button>
        }

        {/* Display if it has no end set, or has not ended yet. Display End/Delete based on if it has started already. */}
        { (
            !maintenance.data.end
            || (!maintenance.data.end && isBefore(new Date(maintenance.data.end), new Date()))
          )
          && isBefore(new Date(maintenance.data.start), new Date())
          ? <Button size='sm' variant='tableButton' onClick={() => setEndModal(true)}>End</Button>
          : <Button size='sm' variant='tableButton' onClick={() => setDeleteModal(true)}>Delete</Button>
        }

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
      <DeleteMaintenanceModal
        instance_id={maintenance.instance_id}
        isOpen={deleteModal}
        setOpen={setDeleteModal}
      />
      <EndMaintenanceModal
        instance_id={maintenance.instance_id}
        isOpen={endModal}
        setOpen={setEndModal}
      />
    </>
  )
}

export default MaintenanceActions