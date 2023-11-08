import React from 'react'
import CustomCard from '@/components/general/Card/CustomCard'
import AudioEngineWarningsTable from './AudioEngineWarningsTable'

type Props = {}

const AudioEngineWarnings: React.FC<Props> = ({ }) => {
  return (
    <CustomCard label='Warnings' className='m-4 h-72' contentPadding={false} >
      <AudioEngineWarningsTable />
    </CustomCard>
  )
}

export default AudioEngineWarnings