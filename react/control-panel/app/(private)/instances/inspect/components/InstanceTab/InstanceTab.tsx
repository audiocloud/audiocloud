import React from 'react'
import { IInstance } from '@/types'
import General from './General/General'
import Configuration from './Configuration/Configuration'
import ObjectNotFoundWarning from '@/components/general/ObjectNotFoundWarning'

type Props = {
  instance: IInstance | undefined
}

const InstanceTab: React.FC<Props> = ({ instance }) => {
  return (
    <div className='p-4 flex flex-wrap gap-4'>
      { instance ? (<>
        <General instance={instance} />
        <Configuration instance={instance} />
      </>)
      : <ObjectNotFoundWarning objectName='Instance'/> }
    </div>
  )
}

export default InstanceTab