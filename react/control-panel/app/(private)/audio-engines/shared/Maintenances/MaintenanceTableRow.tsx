'use client'

import React, { useState } from 'react'
import { TableCell, TableRow } from '@/components/ui/table'
import MaintenanceTypePill from './MaintenanceTypePill'
import MaintenanceTimestamp from './MaintenanceTimestamp'
import MaintenanceDescription from './MaintenanceDescription'
import MaintenanceActions from './MaintenanceActions'
import { IAudioEngineMaintenance } from '@/types'
import EditMaintenance from '../Modals/EditMaintenance'
import AudioEngineButtonLink from '@/components/general/AudioEngineButtonLink'

type Props = {
  maintenance: IAudioEngineMaintenance
}

const MaintenanceTableRow: React.FC<Props> = ({ maintenance }) => {

  const [editModal, setEditModal] = useState(false)
  
  return (
    <TableRow className='group/row'>
      <TableCell><MaintenanceTypePill type={maintenance.data.header}/></TableCell>
      <TableCell><AudioEngineButtonLink engine_id={maintenance.engine_id}/></TableCell>
      <TableCell><MaintenanceTimestamp value={maintenance.data.start}/></TableCell>
      <TableCell><MaintenanceTimestamp value={maintenance.data.end}/></TableCell>
      <TableCell className='hidden 2xl:table-cell'><MaintenanceDescription content={maintenance.data.body}/></TableCell>
      <TableCell className='text-right w-fit xl:w-56'><MaintenanceActions setEditModal={setEditModal} /></TableCell>
      
      <EditMaintenance
        maintenance={maintenance}
        open={editModal}
        setOpen={setEditModal}
      />
    </TableRow>
  )
}

export default MaintenanceTableRow