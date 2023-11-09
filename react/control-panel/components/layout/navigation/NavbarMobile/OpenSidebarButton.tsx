'use client'

import React from 'react'
import { Bars3CenterLeftIcon } from '@heroicons/react/20/solid'

type Props = {
  setSidebarOpen: React.Dispatch<React.SetStateAction<boolean>>
}

const OpenSidebarButton: React.FC<Props> = ({ setSidebarOpen }) => {
  return (
    <button
      type="button"
      className="lg:hidden h-full px-4 hover:bg-accent border-r border-border"
      onClick={() => setSidebarOpen(true)}
    >
      <span className="sr-only">Open sidebar</span>
      <Bars3CenterLeftIcon className="h-6 w-6 text-foreground" aria-hidden="false" />
    </button>
  )
}

export default OpenSidebarButton