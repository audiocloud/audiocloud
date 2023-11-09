import React from 'react'
import { Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import AudioEngineStatus from '@/components/general/Statuses/AudioEngineStatus'
import AudioEngineButtonLink from '@/components/general/ButtonLinks/AudioEngineButtonLink'
import AudioEngineActions from '@/components/general/Actions/AudioEngineActions'

// TO-DO: real data
import { audio_engines } from '@/data/audio-engines'

const AudioEngineWarningsTable: React.FC = () => {

  // const audioEnginesList = Object.values(audio_engines).filter(item => item.status === 'offline')
  const audioEnginesList = Object.values(audio_engines)

  return (
    <Table>
      
      { !audioEnginesList.length && <TableCaption>No audio engines found.</TableCaption> }

      <TableHeader>
        <TableRow>
          <TableHead>Status</TableHead>
          <TableHead>Engine ID</TableHead>
          <TableHead>Machine</TableHead>
          <TableHead>Last seen</TableHead>
          <TableHead>Reason</TableHead>
          <TableHead className='text-right'>Actions</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody className='text-foreground-secondary'>
        { audioEnginesList.map((audio_engine) => (
          <TableRow className='group/row' key={audio_engine.id}>
            <TableCell><AudioEngineStatus status={audio_engine.status} /></TableCell>
            <TableCell><AudioEngineButtonLink engine_id={audio_engine.id}/></TableCell>
            <TableCell>{ audio_engine.machine }</TableCell>
            <TableCell>{ new Date(audio_engine.last_seen).toLocaleString() }</TableCell>
            <TableCell>{'Something is wrong...'}</TableCell>
            <TableCell className='text-right'><AudioEngineActions audio_engine={audio_engine}/></TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  )
}

export default AudioEngineWarningsTable