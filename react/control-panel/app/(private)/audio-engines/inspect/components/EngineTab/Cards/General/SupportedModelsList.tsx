import React from 'react'

type Props = {
  models: string[]
}

const SupportedModelsList: React.FC<Props> = ({ models }) => {
  return (
    <ul role='list' className='flex flex-col gap-1 text-primary'>
      { models.map((model) => <li key={model}>{ model }</li>) }
      { !models.length && <div className='text-primary'>- none -</div> }
    </ul>
  )
}

export default SupportedModelsList