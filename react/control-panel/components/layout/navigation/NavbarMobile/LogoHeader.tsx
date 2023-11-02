import React from 'react'
import Link from 'next/link'
import Image from 'next/image'
import LogoSVG from '@/assets/svg/logo.svg'

const LogoHeader: React.FC = () => {
  return (
    <Link href='/' className="px-4 flex flex-shrink-0 items-center gap-5">
      <div className='relative w-12 h-12'>
        <Image fill src={LogoSVG} alt="AudioCloud Domain Control Panel"/>
      </div>
      <div>Domain Control Panel</div>
    </Link>
  )
}

export default LogoHeader