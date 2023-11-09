import React from 'react'
import Link from 'next/link'
import { usePathname } from 'next/navigation'
import clsx from 'clsx'
import { pages } from '../pages'

const PagesMenu: React.FC = () => {

  const pathname = usePathname()
  
  return (
    <div className="mt-5 h-0 flex-1 overflow-y-auto">
      <nav className="space-y-1">
        { pages.map((page) => (
          <Link
            key={page.name}
            href={page.href}
            className={clsx('group flex items-center px-2 py-2 text-base leading-5 font-medium rounded-md',
              (pathname.startsWith(page.href) && page.href !== '/') || (pathname === page.href)
                ? 'bg-secondary text-foreground'
                : 'hover:bg-secondary text-foreground-secondary hover:text-foreground'
            )}
            aria-current={(pathname.startsWith(page.href) && page.href !== '/') || (pathname === page.href) ? 'page' : undefined}
          >
            <page.icon
              className={clsx('mr-3 flex-shrink-0 h-6 w-6',
                (pathname.startsWith(page.href) && page.href !== '/') || (pathname === page.href)
                ? 'text-foreground'
                : 'text-foreground-secondary group-hover:text-foreground'
              )}
              aria-hidden="false"
            />
            { page.name }
          </Link>
        ))}
      </nav>
    </div>
  )
}

export default PagesMenu