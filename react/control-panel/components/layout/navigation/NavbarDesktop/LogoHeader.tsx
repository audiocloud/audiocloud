import React from 'react'
import Link from 'next/link'
import Image from 'next/image'
import LogoSVG from '@/assets/svg/logo.svg'

const LogoHeader: React.FC = () => {
  return (
    <Link href='/' className="mb-2 px-3 flex flex-col justify-center items-center gap-2">
      <div className='relative w-12 h-12'>
        <Image fill src={LogoSVG} alt="AudioCloud Domain Control Panel"/>
      </div>
      <div className='flex flex-col justify-center items-center text-foreground'>
        <div>AudioCloud</div>
        <div className='text-sm'>Domain Control Panel</div>
        <div className='text-sm text-foreground-secondary'>(v0.1.0-alpha)</div>
      </div>
    </Link>
  )
}

export default LogoHeader