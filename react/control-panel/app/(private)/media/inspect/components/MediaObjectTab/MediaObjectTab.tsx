import React from 'react'
import { IMedia } from '@/types'
import General from './Cards/General/General'
import Download from './Cards/Download'
import Upload from './Cards/Upload'
import ObjectNotFoundWarning from '@/components/general/ObjectNotFoundWarning'

type Props = {
  media: IMedia | undefined
}

const MediaObjectTab: React.FC<Props> = ({ media }) => {
  return (
    <div className='p-4 flex flex-wrap gap-4'>
      { media ? (<>
        <General media={media} />
        <Download download={media.download} />
        <Upload upload={media.upload} /> 
      </>)
      : <ObjectNotFoundWarning objectName='Media Object'/> }
    </div>
  )
}

export default MediaObjectTab