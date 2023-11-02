import React from 'react'
import Link from 'next/link'
import { buttonVariants } from "@/components/ui/button"

type Props = {
  instance_id: string
}

const InstanceButtonLink: React.FC<Props> = ({ instance_id }) => {
  return (
    <Link
      href={`/instances/inspect?instance_id=${instance_id}&tab=instance`}
      className={buttonVariants({ variant: 'tableButton', size: 'sm'})}
    >
      { instance_id }
    </Link>
  )
}

export default InstanceButtonLink