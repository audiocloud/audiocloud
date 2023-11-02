import { instances } from '@/data/instances'
import { IInstanceMaintenance } from '@/types'

const maintenances: IInstanceMaintenance[] = []

Object.values(instances).forEach(instance => {
  instance.maintenance.forEach(maintenance => {
    maintenances.push({
      key: `${instance.id}/${maintenance.start}`,
      instance_id: instance.id,
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