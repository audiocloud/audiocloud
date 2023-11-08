import { devices } from '@/data/devices'
import { IDeviceMaintenance } from '@/types'

const maintenances: IDeviceMaintenance[] = []

Object.values(devices).forEach(device => {
  device.maintenance.forEach(maintenance => {
    maintenances.push({
      key: `${device.id}/${maintenance.start}`,
      device_id: device.id,
      data: maintenance
    })
  })
})

maintenances.sort((a, b) => {
  if (a.data.start < b.data.start) return -1      // a should come before b
  else if (a.data.start > b.data.start) return 1  // a should come after b
  else return 0                                   // a and b are equal in terms of start value
})

export default maintenances