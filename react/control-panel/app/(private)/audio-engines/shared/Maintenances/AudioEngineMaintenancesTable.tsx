'use client'

import React, { useState } from 'react'// TO-DO: real data
import { Table, TableBody, TableCaption, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import MaintenanceTableRow from './MaintenanceTableRow'
import { IAudioEngineMaintenance } from '@/types'

type Props = {
  maintenances: IAudioEngineMaintenance[]
}
const AudioEngineMaintenancesTable: React.FC<Props> = ({ maintenances }) => {

  const [listLength, setListLength] = useState(5)

  return (
    <Table>
      
      { !maintenances.length && <TableCaption>No maintenances scheduled.</TableCaption> }

      <TableHeader>
        <TableRow>
          <TableHead>Type</TableHead>
          <TableHead className='whitespace-nowrap'>Engine ID</TableHead>
          <TableHead>Start</TableHead>
          <TableHead>End</TableHead>
          <TableHead className='hidden 2xl:table-cell'>Description</TableHead>
          <TableHead className='text-right'>Actions</TableHead>
        </TableRow>
      </TableHeader>

      <TableBody className='text-slate-400'>
        { maintenances.map((maintenance, index) => <MaintenanceTableRow key={index} maintenance={maintenance} />)}
      </TableBody>

    </Table>
  )
}

export default AudioEngineMaintenancesTable