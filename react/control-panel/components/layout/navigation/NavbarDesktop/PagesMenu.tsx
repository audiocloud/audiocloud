'use client'

import React from 'react'
import { usePathname } from 'next/navigation'
import clsx from 'clsx'
import { pages } from '../pages'

const PagesMenu: React.FC = () => {

  const pathname = usePathname()

  return (
    <nav className="mt-6 px-3 space-y-1">
      { pages.map((page) => (
        <a
          key={page.name}
          href={page.href}
          className={clsx('group flex items-center px-2 py-2 text-sm font-medium rounded-md',
            (pathname.startsWith(page.href) && page.href !== '/') || (pathname === page.href) ? 'bg-slate-800 text-slate-400' : 'text-slate-600 hover:text-slate-300 hover:bg-slate-800 active:bg-slate-700'
          )}
          aria-current={(pathname.startsWith(page.href) && page.href !== '/') || (pathname === page.href) ? 'page' : undefined}
        >
          <page.icon
            className={clsx('mr-3 flex-shrink-0 h-6 w-6',
              (pathname.startsWith(page.href) && page.href !== '/') || (pathname === page.href) ? 'text-slate-400' : 'text-slate-600 group-hover:text-slate-300'
            )}
            aria-hidden="false"
          />
          { page.name }
        </a>
      ))}
    </nav>
  )
}

export default PagesMenu