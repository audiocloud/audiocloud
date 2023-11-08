import React from 'react'
import { ITask } from '@/types'
import CustomCard from '@/components/general/Card/CustomCard'
import CardLine from '@/components/general/Card/CardLine'
import TracksList from './TracksList'

type Props = {
  task: ITask
}

const Tracks: React.FC<Props> = ({ task }) => {
  return (
    <CustomCard label='Tracks' className='w-[300px]'>
      <CardLine label='List'
        item={<TracksList tracks={task.tracks} />}
        itemsStart={true}
      />
    </CustomCard>
  )
}

export default Tracks