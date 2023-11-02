'use client'

import React, { useState } from 'react'
import { MagnifyingGlassIcon } from '@heroicons/react/20/solid'

const Search: React.FC = () => {

  const [query, setQuery] = useState('')

  return (
    <div className="mt-5 px-3">
      <div className="relative mt-1 rounded-md shadow-sm">
        <div className="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3" aria-hidden="false">
          <MagnifyingGlassIcon className="mr-3 h-4 w-4 text-slate-400" aria-hidden="false" />
        </div>
        <input
          type="text"
          id="main-search"
          name="search"
          className="block w-full py-3 pl-9 text-slate-300 sm:text-sm border border-slate-700 bg-slate-800 rounded-lg"
          placeholder="Search"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
        />
      </div>
    </div>
  )
}

export default Search