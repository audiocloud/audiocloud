import React from 'react'
import { ITaskMixer } from '@/types'

type Props = {
  mixers: ITaskMixer[]
}

const MixersList: React.FC<Props> = ({ mixers }) => {
  return (
    <ul role='list' className='flex flex-col gap-1 text-foreground'>
      { mixers.map((mixer) => <li key={mixer.id}>{ mixer.id }</li>) }
      { !mixers.length && <div className='text-foreground'>- none -</div> }
    </ul>
  )
}

export default MixersList