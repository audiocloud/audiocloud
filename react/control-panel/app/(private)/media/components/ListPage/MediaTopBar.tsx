import React from 'react'
import TopBar from '../../General/TopBar'
import TopBarButton from '../../General/TopBarButton'
import { ArrowUpTrayIcon } from '@heroicons/react/20/solid'

const MediaTopBar: React.FC = () => {
  return (
    <TopBar title='Media'>

      <TopBarButton
        label='New Upload'
        onClickHandler={() => alert('New Upload')}
        icon={<ArrowUpTrayIcon className="h-4 w-4 mr-2" aria-hidden="false" />}
      />

    </TopBar>
  )
}

export default MediaTopBar