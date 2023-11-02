import React, { useState } from 'react'
import { format } from 'date-fns'
import EditMaintenance from './Modals/EditMaintenance'
import { MaintenanceInfo } from '@/types'
import Maintenance from '../General/Maintenances/Maintenance'
import MaintenanceActionButton from '../General/Maintenances/MaintenanceActionButton'
import { PencilSquareIcon } from '@heroicons/react/24/outline'

type Props = {
  instance_id: string,
  data: MaintenanceInfo
}

const InstanceMaintenance: React.FC<Props> = ({ instance_id, data }) => {

  const [open, setOpen] = useState(false)
  
  return (
    <>
      <Maintenance
        subject={instance_id}
        title={data.header}
        timestamp={format(new Date(data.start), 'dd-MM-yyy @ HH:mm')}
        buttons={[
          <MaintenanceActionButton
            key='Edit'
            onClickHandler={() => setOpen(true)}
            icon={<PencilSquareIcon className='w-4 h-4' aria-hidden='false'/>}
          />
        ]}
      />
      <EditMaintenance instance_id={instance_id} maintenance={data} open={open} setOpen={setOpen} />
    </>
  )
}

export default InstanceMaintenance