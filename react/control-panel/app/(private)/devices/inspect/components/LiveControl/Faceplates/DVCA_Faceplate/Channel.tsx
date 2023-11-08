import React from 'react'
import { InstanceParameters } from '@audiocloud/api'
import { ContinuousKnob, Header2, ListKnob } from '@moonlight-neon-ui'
import { convertToNumber, convertToString, ConversionRules } from '../valueConverters'
import { DeviceReportsType } from '@/types'

type Props = {
  channelIndex: number
  link: boolean,
  parameters: InstanceParameters,
  channelParameterHandler: (value: string | number | boolean, channel: number, parameter: string) => void
  conversionRules: ConversionRules,
  reports: DeviceReportsType
}

const Channel: React.FC<Props> = ({ channelIndex, link, parameters, channelParameterHandler, conversionRules, reports }) => {

  const attackOptions = [0.1, 0.3, 1, 3, 5, 10, 15, 20, 30]
  const ratioOptions = [1.5, 2, 4, 6, 8, 10]
  const releaseOptions = [0.01, 0.02, 0.35, 0.45, 0.6, 0.8, 'auto']
  const scfOptions = ['TL', 'TM', 'OFF', 40, 80, 120]
  
  return (
    <div className='flex flex-wrap justify-center items-center gap-10'>

      { !link && <Header2>CH {channelIndex + 1}</Header2> }

      <ContinuousKnob
        channel={channelIndex}
        value={convertToNumber(parameters['threshold'][channelIndex], 'threshold', conversionRules)}
        range={[-15, 15]}
        parameter='threshold'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='THRESHOLD'
        unit='dB'
      />

      <ListKnob
        channel={channelIndex}
        value={convertToString(parameters['attack'][channelIndex], 'attack', conversionRules)}
        list={attackOptions.map(item => item.toString())}
        parameter='attack'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='ATTACK'
        unit='ms'
      />

      <ListKnob
        channel={channelIndex}
        value={convertToString(parameters['ratio'][channelIndex], 'ratio', conversionRules)}
        list={ratioOptions.map(item => item.toString())}
        parameter='ratio'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='RATIO'
      />

      <ListKnob
        channel={channelIndex}
        value={convertToString(parameters['release'][channelIndex], 'release', conversionRules)}
        list={releaseOptions.map(item => item.toString())}
        parameter='release'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='RELEASE'
        unit='ms'
      />

      <ContinuousKnob
        channel={channelIndex}
        value={convertToNumber(parameters['makeUp'][channelIndex], 'makeUp', conversionRules)}
        range={[0, 20]}
        parameter='makeUp'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='MAKE-UP'
        unit='dB'
      />

      <ListKnob
        channel={channelIndex}
        value={convertToString(parameters['scf'][channelIndex], 'scf', conversionRules)}
        list={scfOptions.map(item => item.toString())}
        parameter='scf'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='SC. FILTER'
        unit='Hz'
      />

      <ContinuousKnob
        channel={channelIndex}
        value={convertToNumber(parameters['scLink'][channelIndex], 'scLink', conversionRules)}
        range={[0, -20]}
        parameter='scLink'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='SC. LINK'
        unit='dB'
      />

    </div>
  )
}

export default Channel