import React from 'react'
import Link from 'next/link'
import { buttonVariants } from '@/components/ui/button'

type Props = {
  engine_id: string
}

const AudioEngineButtonLink: React.FC<Props> = ({ engine_id }) => {
  return (
    <Link
      href={`/audio-engines/inspect?engine_id=${engine_id}&tab=engine`}
      className={buttonVariants({ variant: 'tableButton', size: 'sm'})}
    >
      { engine_id }
    </Link>
  )
}

export default AudioEngineButtonLink