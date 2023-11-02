'use client'

import React, { useState } from 'react'
import { MagnifyingGlassIcon } from '@heroicons/react/20/solid'
import { Input } from '@/components/ui/input'

const Search: React.FC = () => {

  const [query, setQuery] = useState('')
  
  return (
    <div className='relative w-full h-full flex justify-center items-center text-gray-400'>
      <div className='pointer-events-none absolute inset-y-0 left-2 flex items-center'>
        <MagnifyingGlassIcon className='h-4 w-4' aria-hidden='false' />
      </div>
      <Input
        type='search'
        id='search-field'
        name='search-field'
        className='pl-7'
        placeholder='Search'
        value={query}
        onChange={(e) => setQuery(e.target.value)}
      />
    </div>
  )
}

export default Search