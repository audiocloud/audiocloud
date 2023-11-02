import React from 'react'
import { audio_engines } from '@/data/audio-engines'
import { Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import AudioEngineStatus from './AudioEngineStatus'
import AudioEngineButtonLink from '@/components/general/AudioEngineButtonLink'
import AudioEngineActions from './AudioEngineActions'

const AudioEnginesTable: React.FC = () => {

  const audioEnginesList = Object.values(audio_engines)

  return (
    <Table>
      
      { !audioEnginesList.length && <TableCaption>No audio engines found.</TableCaption> }

      <TableHeader>
        <TableRow>
          <TableHead>Status</TableHead>
          <TableHead>Engine ID</TableHead>
          <TableHead>Machine</TableHead>
          <TableHead>Buffer Size</TableHead>
          <TableHead>CPU</TableHead>
          <TableHead>Memory</TableHead>
          <TableHead>Disk</TableHead>
          <TableHead>Tasks</TableHead>
          <TableHead>Last seen</TableHead>
          <TableHead className='text-right'>Actions</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody className='text-slate-400'>
        { audioEnginesList.map((audio_engine) => (
          <TableRow className='group/row' key={audio_engine.id}>
            <TableCell><AudioEngineStatus status={audio_engine.status} /></TableCell>
            <TableCell><AudioEngineButtonLink engine_id={audio_engine.id}/></TableCell>
            <TableCell>{ audio_engine.machine }</TableCell>
            <TableCell>{ audio_engine.buffer_size }</TableCell>
            <TableCell>{ audio_engine.resources.cpu } MHz</TableCell>
            <TableCell>{ (audio_engine.resources.memory / 1000).toFixed(1) } GB</TableCell>
            <TableCell>{ (audio_engine.resources.disk / 1000).toFixed(1) } GB </TableCell>
            <TableCell>{ audio_engine.engine_tasks.length }</TableCell>
            <TableCell>{ new Date(audio_engine.last_seen).toLocaleString() }</TableCell>
            <TableCell className='text-right'><AudioEngineActions audio_engine={audio_engine}/></TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  )
}

export default AudioEnginesTable