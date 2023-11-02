import React from 'react'
import Link from 'next/link'
import { ArrowPathIcon, BoltIcon, BoltSlashIcon, MagnifyingGlassIcon } from '@heroicons/react/24/outline'
import { Instance } from '@/types'
import Alert from '../General/Alerts/Alert'
import AlertActionButton from '../General/Alerts/AlertActionButton'

type Props = {
  instance: Instance
}

const InstanceAlert: React.FC<Props> = ({ instance }) => {

  return (
    <Alert
      subject={instance.id}
      status={instance.status}
      extra_info={new Date(instance.last_seen).toLocaleString()}
      buttons={[
        <AlertActionButton
          key='Restart'
          onClickHandler={() => alert('Restart')}
          icon={<ArrowPathIcon className="w-4 h-4" aria-hidden="false"/>}
        />,
        <AlertActionButton
          key='Power Off'
          onClickHandler={() => alert('Power Off')}
          icon={<BoltSlashIcon className="w-4 h-4" aria-hidden="false"/>}
        />,
        <AlertActionButton
          key='Power On'
          onClickHandler={() => alert('Power On')}
          icon={<BoltIcon className="w-4 h-4" aria-hidden="false"/>}
        />,
        <Link key='Inspect' href={`/instances/${instance.id}`}>
          <AlertActionButton
            onClickHandler={() => {return}}
            icon={<MagnifyingGlassIcon className="w-4 h-4" aria-hidden="false"/>}
          />
        </Link>
      ]}
    />
  )
}

export default InstanceAlert