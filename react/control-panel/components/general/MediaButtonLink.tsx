import React from 'react'
import Link from 'next/link'
import { buttonVariants } from '@/components/ui/button'

type Props = {
  media_id: string
}

const MediaButtonLink: React.FC<Props> = ({ media_id }) => {
  return (
    <Link
      href={`/media/inspect?media_id=${media_id}&tab=media`}
      className={buttonVariants({ variant: 'tableButton', size: 'sm'})}
    >
      { media_id }
    </Link>
  )
}

export default MediaButtonLink