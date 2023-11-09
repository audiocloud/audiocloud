'use client'

import React from 'react'
import Link from 'next/link'
import { ChevronDownIcon, ArrowLeftOnRectangleIcon, WrenchScrewdriverIcon  } from '@heroicons/react/20/solid'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'
import useAudioCloudAuth from '@/hooks/useAudioCloudAuth'

const UserAccountDropdown: React.FC = () => {

  const { id, logout } = useAudioCloudAuth()

  return (
    <DropdownMenu>

      <DropdownMenuTrigger className='group w-full px-3 py-2 flex justify-between items-center hover:bg-background text-sm border border-transparent hover:border-border rounded-md'>
        <span className='truncate'>
          <span className='text-foreground-secondary'>Logged in: </span>
          <span className='text-foreground font-medium'>{ id || 'placeholder'}</span>
        </span>
        <ChevronDownIcon className='h-5 w-5 flex-shrink-0 text-foreground-secondary group-hover:text-foreground' aria-hidden='false' />
      </DropdownMenuTrigger>

      <DropdownMenuContent className='shadow-md'>

        <DropdownMenuItem asChild>
          <Link href='/settings' className='w-56 flex justify-start items-center gap-2'>
            <WrenchScrewdriverIcon className='h-4 w-4 flex-shrink-0' aria-hidden='false' />
            <span>Settings</span>
          </Link>
        </DropdownMenuItem>

        <DropdownMenuItem asChild>
          <button
            type='button'
            className='w-56 flex justify-start items-center gap-2'
            onClick={() => logout()}
          >
            <ArrowLeftOnRectangleIcon className='h-4 w-4 flex-shrink-0' aria-hidden='false' />
            <span>Logout</span>
          </button>
        </DropdownMenuItem>

      </DropdownMenuContent>

    </DropdownMenu>
  )
}

export default UserAccountDropdown