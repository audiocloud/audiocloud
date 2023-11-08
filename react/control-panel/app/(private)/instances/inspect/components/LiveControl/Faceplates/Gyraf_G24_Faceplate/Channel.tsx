import React from 'react'
import { InstanceParameters } from '@audiocloud/api'
import { ContinuousKnob, Header2 } from '@moonlight-neon-ui'
import { convertToNumber, ConversionRules } from '../valueConverters'
import { InstanceReportsType } from '@/types'

type Props = {
  channelIndex: number
  link: boolean,
  parameters: InstanceParameters,
  channelParameterHandler: (value: string | number | boolean, channel: number, parameter: string) => void,
  conversionRules: ConversionRules,
  reports: InstanceReportsType
}

const Channel: React.FC<Props> = ({ channelIndex, link, parameters, channelParameterHandler, conversionRules, reports }) => {

  return (
    <div className='flex flex-wrap justify-center items-center gap-2'>

      { !link && <Header2>{ channelIndex ? 'B' : 'A'}</Header2> }

      <ContinuousKnob
        channel={channelIndex}
        value={convertToNumber(parameters['threshold'][channelIndex], 'threshold', conversionRules)}
        range={[0, 100]}
        parameter='threshold'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='THRESHOLD'
        unit='dB'
      />

      <ContinuousKnob
        channel={channelIndex}
        value={convertToNumber(parameters['ratio'][channelIndex], 'ratio', conversionRules)}
        range={[0, 100]}
        parameter='ratio'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='RATIO'
      />

      <ContinuousKnob
        channel={channelIndex}
        value={convertToNumber(parameters['attack'][channelIndex], 'attack', conversionRules)}
        range={[0, 100]}
        parameter='attack'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='ATTACK'
        unit='ms'
      />

      <ContinuousKnob
        channel={channelIndex}
        value={convertToNumber(parameters['release'][channelIndex], 'release', conversionRules)}
        range={[0, 100]}
        parameter='release'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='RELEASE'
        unit='ms'
      />

      <ContinuousKnob
        channel={channelIndex}
        value={convertToNumber(parameters['feed'][channelIndex], 'feed', conversionRules)}
        range={[0, 100]}
        parameter='feed'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='FEED'
      />

      <ContinuousKnob
        channel={channelIndex}
        value={convertToNumber(parameters['elliptic'][channelIndex], 'elliptic', conversionRules)}
        range={[0, 100]}
        parameter='elliptic'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='ELLIPTIC'
      />

    </div>
  )
}

export default Channel