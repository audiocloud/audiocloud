import React from 'react'
import { ITaskTrack } from '@/types'

type Props = {
  tracks: ITaskTrack[]
}

const TracksList: React.FC<Props> = ({ tracks }) => {
  return (
    <ul role='list' className='flex flex-col gap-1 text-primary'>
      { tracks.map((track) => <li key={track.id}>{ track.id }</li>) }
      { !tracks.length && <div className='text-primary'>- none -</div> }
    </ul>
  )
}

export default TracksList