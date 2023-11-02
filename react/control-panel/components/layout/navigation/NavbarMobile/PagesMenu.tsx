import React from 'react'
import Link from 'next/link'
import { usePathname } from 'next/navigation'
import clsx from 'clsx'
import { pages } from '../pages'

const PagesMenu: React.FC = () => {

  const pathname = usePathname()
  
  return (
    <div className="mt-5 h-0 flex-1 overflow-y-auto">
      <nav className="px-2 space-y-1">
        { pages.map((page) => (
          <Link
            key={page.name}
            href={page.href}
            className={clsx(
              (pathname.startsWith(page.href) && page.href !== '/') || (pathname === page.href)
                ? 'bg-gray-100 text-gray-900'
                : 'text-gray-600 hover:text-gray-900 hover:bg-gray-50',
              'group flex items-center px-2 py-2 text-base leading-5 font-medium rounded-md'
            )}
            aria-current={(pathname.startsWith(page.href) && page.href !== '/') || (pathname === page.href) ? 'page' : undefined}
          >
            <page.icon
              className={clsx(
                (pathname.startsWith(page.href) && page.href !== '/') || (pathname === page.href) ? 'text-gray-500' : 'text-gray-400 group-hover:text-gray-500',
                'mr-3 flex-shrink-0 h-6 w-6'
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