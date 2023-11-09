'use client'

import React from 'react'
import { XMarkIcon } from '@heroicons/react/24/outline'

type Props = {
  setSidebarOpen: React.Dispatch<React.SetStateAction<boolean>>
}

const CloseSidebarButton: React.FC<Props> = ({ setSidebarOpen }) => {
  return (
    <button
      type="button"
      className="ml-1 flex h-10 w-10 items-center justify-center rounded-full"
      onClick={() => setSidebarOpen(false)}
    >
      <span className="sr-only">Close sidebar</span>
      <XMarkIcon className="h-6 w-6 text-foreground" aria-hidden="false" />
    </button>
  )
}

export default CloseSidebarButton