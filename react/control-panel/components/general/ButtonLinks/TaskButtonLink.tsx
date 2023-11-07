import React from 'react'
import Link from 'next/link'
import { buttonVariants } from '@/components/ui/button'

type Props = {
  task_id: string
}

const MediaButtonLink: React.FC<Props> = ({ task_id }) => {
  return (
    <Link
      href={`/tasks/inspect?task_id=${task_id}&tab=task`}
      className={buttonVariants({ variant: 'tableButton', size: 'sm'})}
    >
      { task_id }
    </Link>
  )
}

export default MediaButtonLink