'use client'

import React, { useState } from 'react'
import { MagnifyingGlassIcon } from '@heroicons/react/20/solid'
import { Input } from '@/components/ui/input'

const Search: React.FC = () => {

  const [query, setQuery] = useState('')

  return (
    <div className='relative w-full'>
      <div className='pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3' aria-hidden='false'>
        <MagnifyingGlassIcon className='h-4 w-4 text-foreground-secondary' aria-hidden='false' />
      </div>
      <Input
        type='text'
        id='main-search'
        name='search'
        className='pl-9'
        placeholder='Search'
        value={query}
        onChange={(e) => setQuery(e.target.value)}
      />
    </div>
  )
}

export default Search