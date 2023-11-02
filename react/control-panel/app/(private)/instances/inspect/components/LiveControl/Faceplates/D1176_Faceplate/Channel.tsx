import React from 'react'
import { InstanceParameters } from '@audiocloud/api'
import { ChannelPillButtonSwitch, ContinuousKnob, Header2, HorizontalMeter } from '@moonlight-neon-ui'
import { convertToNumber, convertToString, ConversionRules } from '../valueConverters'
import { InstanceReports } from '@/types'

type Props = {
  channelIndex: number
  link: boolean,
  parameters: InstanceParameters,
  channelParameterHandler: (value: string | number | boolean, channel: number, parameter: string) => void,
  conversionRules: ConversionRules,
  reports: InstanceReports
}

const Channel: React.FC<Props> = ({ channelIndex, link, parameters, channelParameterHandler, conversionRules, reports }) => {

  return (
    <div className='flex flex-wrap justify-center items-center gap-10'>

      { !link && <Header2>CH {channelIndex + 1}</Header2> }

      <ContinuousKnob
        channel={channelIndex}
        value={convertToNumber(parameters['inputLevel'][channelIndex], 'inputLevel', conversionRules)}
        range={[-60, 0]}
        parameter='inputLevel'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='INPUT'
        unit='dB'
      />
      
      <ContinuousKnob
        channel={channelIndex}
        value={convertToNumber(parameters['outputLevel'][channelIndex], 'outputLevel', conversionRules)}
        range={[-60, 0]}
        parameter='outputLevel'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='OUTPUT'
        unit='dB'
      />

      <div className='flex flex-col justify-center items-center gap-5'>
        <ContinuousKnob
          channel={channelIndex}
          value={convertToNumber(parameters['attack'][channelIndex], 'attack', conversionRules)}
          range={[1, 7]}
          parameter='attack'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='ATTACK'
        />
        <ContinuousKnob
          channel={channelIndex}
          value={convertToNumber(parameters['release'][channelIndex], 'release', conversionRules)}
          range={[1, 7]}
          parameter='release'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='RELEASE'
        />
      </div>

      <ChannelPillButtonSwitch
        options={['4', '8', '12', '20']}
        value={convertToString(parameters['ratio'][channelIndex], 'ratio', conversionRules)}
        channel={channelIndex}
        parameter='ratio'
        parameterHandler={channelParameterHandler}
        label={'RATIO'}
      />

      <div className='flex flex-col justify-center items-center gap-5'>
        <HorizontalMeter
          scale={['-20', '-10', '-7', '-5', '-3', '-1', '0', '1', '2', '3']}
          value_stereo={reports['gainReduction'] as [number, number]}
          label='GR'
        />
        <HorizontalMeter
          scale={['-20', '-10', '-7', '-5', '-3', '-1', '0', '1', '2', '3']}
          value_stereo={reports['volume'] as [number, number]}
          label='VU'
        />
      </div>

    </div>
  )
}

export default Channel