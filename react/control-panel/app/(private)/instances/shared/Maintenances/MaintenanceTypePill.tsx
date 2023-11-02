import React from 'react'
import { Badge } from '@/components/ui/badge'

type Props = {
  type: string
}

const MaintenanceTypePill: React.FC<Props> = ({ type }) => {

  const getVariant = () => {
    if (type === 'Weekly checkup') return 'maintenance_ae_weekly_checkup'
    if (type === 'Software update') return 'maintenance_ae_software_update'
    if (type === 'Urgent service') return 'maintenance_ae_urgent_service'
    if (type === 'Ticket checkup') return 'maintenance_ae_ticket_checkup'
    return 'default'
  }

  return (
    <Badge variant={getVariant()}>
      { type }
    </Badge>
  )
}

export default MaintenanceTypePill