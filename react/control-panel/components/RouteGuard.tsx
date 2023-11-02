'use client'

import React, { useState, useEffect, ReactNode } from 'react'
import { usePathname, useRouter } from 'next/navigation'
import { useCookies } from 'react-cookie'

const PUBLIC_PATHS = ['/auth/', '/404', '/500']

type Props = {
  children: ReactNode
}

const RouteGuard: React.FC<Props> = ({ children }) => {
  
  const [cookies, setCookie, removeCookie] = useCookies(['token'])
  
  const [authorized, setAuthorized] = useState(false)
  
  const pathname = usePathname()
  const router = useRouter()

  useEffect(() => {
    
    const authCheck = () => {

      // if authenticated -> '/'
      if (cookies.token && pathname.startsWith('/auth/')) router.push('/')
  
      // if NOT authenticated -> '/auth/signin
      if (!cookies.token && PUBLIC_PATHS.filter(option => pathname.startsWith(option)).length === 0) {
        setAuthorized(false)
        router.push('/auth/signin')
      } else {
        setAuthorized(true)
      }
    }
    authCheck()
  }, [cookies.token, pathname, router])

  return (authorized ? children : null)
}

export default RouteGuard