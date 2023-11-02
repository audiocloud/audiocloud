'use client'

import React, { useState } from 'react'
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue } from '@/components/ui/select'

const RefreshDropdown: React.FC = () => {

  const [value, setValue] = useState<number>(1) // TO-DO: global context
  
  return (
    <SelectGroup className='flex items-center'>
      <SelectLabel className='text-xs sm:text-sm'>Refresh rate:</SelectLabel>
      <Select onValueChange={(e) => setValue(parseInt(e))}>
        <SelectTrigger className='w-24'>
          <SelectValue placeholder={`${value} sec`} />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value='1'>1 sec</SelectItem>
          <SelectItem value='3'>3 sec</SelectItem>
          <SelectItem value='5'>5 sec</SelectItem>
          <SelectItem value='10'>10 sec</SelectItem>
        </SelectContent>
      </Select>
    </SelectGroup>
  )
}

export default RefreshDropdown