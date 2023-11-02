import React from 'react'
import { IAudioEngine } from '@/types'
import General from './Cards/General/General'
import Configuration from './Cards/Configuration'
import Resources from './Cards/Resources'
import ObjectNotFoundWarning from '@/components/general/ObjectNotFoundWarning'

type Props = {
  audioEngine: IAudioEngine | undefined
}

const EngineTab: React.FC<Props> = ({ audioEngine }) => {
  return (
    <div className='p-4 flex flex-wrap gap-4'>
      { audioEngine ? (<>
        <General engine={audioEngine} />
        <Configuration engine={audioEngine} />
        <Resources engine={audioEngine} /> 
      </>)
      : <ObjectNotFoundWarning objectName='Audio Engine'/> }
    </div>
  )
}

export default EngineTab