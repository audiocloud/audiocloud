import React from 'react'
import { instances } from '@/data/instances'
import { Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import InstanceStatus from './InstanceStatus'
import InstanceButtonLink from '@/components/general/InstanceButtonLink'
import InstanceActions from './InstanceActions'

const InstancesTable: React.FC = () => {

  const instancesList = Object.values(instances)

  return (
    <Table>
      
      { !instancesList.length && <TableCaption>No instances found.</TableCaption> }

      <TableHeader>
        <TableRow>
          <TableHead>Status</TableHead>
          <TableHead>Instance ID</TableHead>
          <TableHead>Model ID</TableHead>
          <TableHead>Engine ID</TableHead>
          <TableHead>Driver ID</TableHead>
          <TableHead>Input at</TableHead>
          <TableHead>Output at</TableHead>
          <TableHead>Last seen</TableHead>
          <TableHead className='text-right'>Actions</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody className='text-slate-400'>
        { instancesList.map((instance) => (
          <TableRow className='group/row' key={instance.id}>
            <TableCell><InstanceStatus status={instance.status} /></TableCell>
            <TableCell><InstanceButtonLink instance_id={instance.id}/></TableCell>
            <TableCell>{ instance.model_id }</TableCell>
            <TableCell>{ instance.engine_id }</TableCell>
            <TableCell>{ instance.driver_id }</TableCell>
            <TableCell>{ instance.engine_input_at }</TableCell>
            <TableCell>{ instance.engine_output_at }</TableCell>
            <TableCell>{ new Date(instance.last_seen).toLocaleString() }</TableCell>
            <TableCell className='text-right'><InstanceActions instance={instance}/></TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  )
}

export default InstancesTable