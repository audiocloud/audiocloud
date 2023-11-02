'use client'

import React from 'react'
import Link from 'next/link'
import { ChevronLeftIcon } from '@heroicons/react/20/solid'
import { buttonVariants } from '@/components/ui/button'
import { usePathname } from 'next/navigation'

const BackButton: React.FC = () => {

  const pathname = usePathname()

  const newPathname = () => {
    const pathParts = pathname.split('/')
    pathParts.pop()
    return pathParts.join('/')
  }

  return (
    <Link 
      href={newPathname()}
      className={buttonVariants({ variant: 'secondary', size: 'smallIcon'})}
    >
      <ChevronLeftIcon className='w-6 h-6' aria-hidden='false' />
    </Link>
  )
}

export default BackButton