import React from 'react'
import { IAudioEngine } from '@/types'
import CustomCard from '@/components/general/Card/CustomCard'
import CardLine from '@/components/general/Card/CardLine'

type Props = {
  engine: IAudioEngine
}

const Configuration: React.FC<Props> = ({ engine }) => {

  return (
    <CustomCard label='Configuration' className='w-[300px]'>
      <div className='w-full flex flex-col justify-start items-center gap-1'>
        <CardLine label='Buffer Size' item={`${engine.buffer_size} samples`} />
        <CardLine label='Sample Rate' item={`${engine.sample_rate && (engine.sample_rate / 1000).toFixed(1)} kHz`} />
        <CardLine label='Bit Depth Rate' item={`${engine.bit_depth}-bit ${engine.bit_depth === 32 ? 'float' : ''}`} />
        <CardLine label='Inputs Size' item={`${engine.inputs.length}`} />
        <CardLine label='Outputs Size' item={`${engine.outputs.length}`} />
      </div>
    </CustomCard>
  )
}

export default Configuration