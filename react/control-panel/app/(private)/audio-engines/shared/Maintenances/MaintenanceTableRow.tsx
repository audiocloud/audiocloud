import React from 'react'
import { TableCell, TableRow } from '@/components/ui/table'
import MaintenanceTypePill from './MaintenanceTypePill'
import AudioEngineButtonLink from '@/components/general/ButtonLinks/AudioEngineButtonLink'
import MaintenanceTimestamp from './MaintenanceTimestamp'
import MaintenanceDescription from './MaintenanceDescription'
import MaintenanceActions from './MaintenanceActions'
import { IAudioEngineMaintenance } from '@/types'

type Props = {
  maintenance: IAudioEngineMaintenance
}

const MaintenanceTableRow: React.FC<Props> = ({ maintenance }) => {

  
  return (
    <TableRow className='group/row'>
      <TableCell><MaintenanceTypePill type={maintenance.data.header}/></TableCell>
      <TableCell><AudioEngineButtonLink engine_id={maintenance.engine_id}/></TableCell>
      <TableCell><MaintenanceTimestamp value={maintenance.data.start}/></TableCell>
      <TableCell><MaintenanceTimestamp value={maintenance.data.end}/></TableCell>
      <TableCell className='hidden 2xl:table-cell'><MaintenanceDescription content={maintenance.data.body}/></TableCell>
      <TableCell className='text-right w-fit xl:w-64'><MaintenanceActions maintenance={maintenance} /></TableCell>
    </TableRow>
  )
}

export default MaintenanceTableRow