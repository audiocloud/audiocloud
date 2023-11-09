import React from 'react'
import { devices } from '@/data/devices'
import { Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import DeviceStatus from '../../../shared/DeviceStatus'
import DeviceButtonLink from '@/components/general/ButtonLinks/DeviceButtonLink'
import DeviceActions from './DeviceActions'

const DevicesTable: React.FC = () => {

  const devicesList = Object.values(devices)

  return (
    <Table>
      
      { !devicesList.length && <TableCaption>No devices found.</TableCaption> }

      <TableHeader>
        <TableRow>
          <TableHead>Status</TableHead>
          <TableHead>Device ID</TableHead>
          <TableHead>Model ID</TableHead>
          <TableHead>Engine ID</TableHead>
          <TableHead>Driver ID</TableHead>
          <TableHead>Input at</TableHead>
          <TableHead>Output at</TableHead>
          <TableHead>Last seen</TableHead>
          <TableHead className='text-right'>Actions</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody className='text-foreground-secondary'>
        { devicesList.map((device) => (
          <TableRow className='group/row' key={device.id}>
            <TableCell><DeviceStatus status={device.status} /></TableCell>
            <TableCell><DeviceButtonLink device_id={device.id}/></TableCell>
            <TableCell>{ device.model_id }</TableCell>
            <TableCell>{ device.engine_id }</TableCell>
            <TableCell>{ device.driver_id }</TableCell>
            <TableCell>{ device.engine_input_at }</TableCell>
            <TableCell>{ device.engine_output_at }</TableCell>
            <TableCell>{ new Date(device.last_seen).toLocaleString() }</TableCell>
            <TableCell className='text-right'><DeviceActions device={device}/></TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  )
}

export default DevicesTable