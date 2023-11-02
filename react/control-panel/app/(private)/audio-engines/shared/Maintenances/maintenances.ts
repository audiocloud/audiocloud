import { audio_engines } from '@/data/audio-engines'
import { IAudioEngineMaintenance } from '@/types'

const maintenances: IAudioEngineMaintenance[] = []

Object.values(audio_engines).forEach(engine => {
  engine.maintenance.forEach(maintenance => {
    maintenances.push({
      key: `${engine.id}/${maintenance.start}`,
      engine_id: engine.id,
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