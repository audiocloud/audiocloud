import React from 'react'
import Link from 'next/link'
import { buttonVariants } from '@/components/ui/button'

type Props = {
  device_id: string
}

const DeviceButtonLink: React.FC<Props> = ({ device_id }) => {
  return (
    <Link
      href={`/devices/inspect?device_id=${device_id}&tab=device`}
      className={buttonVariants({ variant: 'tableButton', size: 'sm'})}
    >
      { device_id }
    </Link>
  )
}

export default DeviceButtonLink