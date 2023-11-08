'use client'

import React from 'react'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'

type Props = {
  label: string,
  options: { value: string, label: string }[]
}

const ConfigDropdown: React.FC<Props> = ({ label, options }) => {

  // TO-DO: add state and confirm/reset

  return (
    <Select>
      <SelectTrigger className='w-[140px]'>
        <SelectValue placeholder={label} />
      </SelectTrigger>
      <SelectContent>
        { options.map(option => (
          <SelectItem value={option.value}>{ option.label }</SelectItem>
        ))}
      </SelectContent>
    </Select>
  )
}

export default ConfigDropdown