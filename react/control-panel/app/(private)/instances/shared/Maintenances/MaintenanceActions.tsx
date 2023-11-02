'use client'

import React from 'react'
import { Button } from '@/components/ui/button'

type Props = {
  setEditModal: React.Dispatch<React.SetStateAction<boolean>>
}

const MaintenanceActions: React.FC<Props> = ({ setEditModal }) => {

  return (
    <div className='hidden group-hover/row:flex flex-col xl:flex-row justify-center items-end gap-2'>
      <Button size='sm' variant='tableButton' onClick={() => setEditModal(true)}>Edit</Button>
      <Button size='sm' variant='tableButton'>Extend</Button>
      <Button size='sm' variant='tableButton'>End</Button>
    </div>
  )
}

export default MaintenanceActions