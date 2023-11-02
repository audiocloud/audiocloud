'use client'

import React from 'react'
import { Button } from '@/components/ui/button'

type Props = {
  node_id: string
}

const AudioEngineTasksActions: React.FC<Props> = ({ node_id }) => {
  return (
    <div className='hidden group-hover/row:flex flex-col xl:flex-row justify-center items-end gap-2'>
      <Button size='sm' variant='tableButton' onClick={() => alert('Restart task (delete from engine)!')}>Restart</Button>
      <Button size='sm' variant='tableButton' onClick={() => alert('Delete task (from domain)!')}>Delete</Button>
    </div>
  )
}

export default AudioEngineTasksActions