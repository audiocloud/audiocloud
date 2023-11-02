import React, { DetailedHTMLProps, HTMLAttributes } from 'react'
import Link from 'next/link'
import {
  Table,
  TableHead,
  TableRow,
  TableHeaderCell,
  TableBody,
  TableCell,
  Badge,
} from "@tremor/react"
import { RectangleStackIcon } from '@heroicons/react/24/outline'
import DashboardWidget from '../DashboardWidget'

const data = [
  {
    id: 'ae-0',
    status: 'online',
    cpu: '200 MHz',
    ram: '400 MB',
    disk: '500 MB',
    tasks: '3'
  },
  {
    id: 'ae-1',
    status: 'online',
    cpu: '200 MHz',
    ram: '400 MB',
    disk: '500 MB',
    tasks: '3'
  },
  {
    id: 'ae-2',
    status: 'offline',
    cpu: '200 MHz',
    ram: '400 MB',
    disk: '500 MB',
    tasks: '3'
  }
];

const AudioEnginesContents: React.FC<DetailedHTMLProps<HTMLAttributes<HTMLDivElement>, HTMLDivElement>> = ({ className }) => {

  return (
    <DashboardWidget title='Audio Engines' href='/audio-engines' className={className}>
      <Table className="w-full">
        <TableHead className='border-b border-slate-200'>
          <TableRow>
            <TableHeaderCell>ID</TableHeaderCell>
            <TableHeaderCell>Status</TableHeaderCell>
            <TableHeaderCell>Tasks</TableHeaderCell>
            <TableHeaderCell>CPU</TableHeaderCell>
            <TableHeaderCell>RAM</TableHeaderCell>
            <TableHeaderCell>Disk</TableHeaderCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {data.map((item) => (
            <TableRow key={item.id} className='hover:bg-slate-50'>
              <TableCell><Link className='hover:underline font-semibold text-lg' href={`/audio-engines/${item.id}`}>{item.id}</Link></TableCell>
              <TableCell><Badge color={item.status === 'online' ? 'green' : 'red'}>{item.status}</Badge></TableCell>
              <TableCell><Badge><div className='flex justify-center items-center gap-1'><RectangleStackIcon className='w-4 h-4' aria-hidden='false' />{item.tasks}</div></Badge></TableCell>
              <TableCell><Badge>{item.cpu}</Badge></TableCell>
              <TableCell><Badge>{item.ram}</Badge></TableCell>
              <TableCell><Badge>{item.disk}</Badge></TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </DashboardWidget>
  )
}

export default AudioEnginesContents