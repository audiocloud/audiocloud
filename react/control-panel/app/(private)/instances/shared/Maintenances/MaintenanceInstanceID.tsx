import React from 'react'

type Props = {
  instance_id: string
}

const MaintenanceInstanceID: React.FC<Props> = ({ instance_id }) => {
  return (
    <div className='text-white text-lg whitespace-nowrap'>{ instance_id }</div>
  )
}

export default MaintenanceInstanceID