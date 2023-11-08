import React from 'react'

type Props = {
  content: string
}

const MaintenanceDescription: React.FC<Props> = ({ content }) => {

  return (
    <div className='whitespace-normal truncate'>
      <span>{content.slice(0, 330)}</span>
      { content.length > 330 && <span>...</span> }
    </div>
  )
}

export default MaintenanceDescription