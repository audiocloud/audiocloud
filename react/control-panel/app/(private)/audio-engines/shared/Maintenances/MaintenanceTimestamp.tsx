import React from 'react'

type Props = {
  value: string | undefined
}

const MaintenanceTimestamp: React.FC<Props> = ({ value }) => {
  return (
    <div className='text-foreground whitespace-nowrap'>
      { value ? new Date(value).toLocaleString(undefined, {
        day: 'numeric',
        month: 'short',
        year: 'numeric',
        hour: 'numeric',
        minute: 'numeric',
      }) : 'not set' }
    </div>
  )
}

export default MaintenanceTimestamp