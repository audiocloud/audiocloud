import React from 'react'
import { IAudioEngine } from '@/types'
import CustomCard from '@/components/general/Card/CustomCard'
import CardLine from '@/components/general/Card/CardLine'

type Props = {
  engine: IAudioEngine
}

const Resources: React.FC<Props> = ({ engine }) => {

  return (
    <CustomCard label='Resources' className='w-[300px]'>
      <CardLine label='CPU' item={`${engine.resources.cpu} MHz`} />
      <CardLine label='Memory' item={`${engine.resources.memory} MB`} />
      <CardLine label='Disk' item={`${engine.resources.disk} MB`} />
      { engine.resources.antelope_dsp && <CardLine label='Antelope DSP' item={`${engine.resources.antelope_dsp}%`} /> }
      { engine.resources.cuda_dsp && <CardLine label='Cuda DSP' item={`${engine.resources.cuda_dsp} cores`} /> }
      { engine.resources.uad_dsp && <CardLine label='UAD DSP' item={`${engine.resources.uad_dsp} cores`} /> }
    </CustomCard>
  )
}

export default Resources