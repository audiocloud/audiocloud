import React from 'react'
import CustomCard from '@/components/general/Card/CustomCard'

type Props = {}

const InstanceWarnings: React.FC<Props> = ({ }) => {
  return (
    <CustomCard label='Warnings' className='m-4 h-full'>
      No warnings.
    </CustomCard>
  )
}

export default InstanceWarnings