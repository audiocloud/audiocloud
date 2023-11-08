import React from 'react'
import { IDevice } from '@/types'
import General from './General/General'
import Configuration from './Configuration/Configuration'
import ObjectNotFoundWarning from '@/components/general/ObjectNotFoundWarning'

type Props = {
  device: IDevice | undefined
}

const DeviceTab: React.FC<Props> = ({ device }) => {
  return (
    <div className='p-4 flex flex-wrap gap-4'>
      { device ? (<>
        <General device={device} />
        <Configuration device={device} />
      </>)
      : <ObjectNotFoundWarning objectName='Device'/> }
    </div>
  )
}

export default DeviceTab