import React from 'react'
import { InstanceParameters } from '@audiocloud/api'
import { ContinuousKnob, Header2, HorizontalMeter, SteppedKnob } from '@moonlight-neon-ui'
import { convertToNumber, convertToString, ConversionRules } from '../valueConverters'
import { InstanceReportsType } from '@/types'

type Props = {
  channelIndex: number
  link: boolean,
  parameters: InstanceParameters,
  channelParameterHandler: (value: string | number | boolean, channel: number, parameter: string) => void
  conversionRules: ConversionRules,
  reports: InstanceReportsType
}

const Channel: React.FC<Props> = ({ channelIndex, link, parameters, channelParameterHandler, conversionRules, reports }) => {

  return (
    <div className='flex flex-wrap justify-center items-center gap-10'>

      { !link && <Header2>CH {channelIndex + 1}</Header2> }

      { convertToString(parameters['metering'][0], 'metering', conversionRules) === 'IN' && (
        <HorizontalMeter
          scale={['-20', '-10', '-7', '-5', '-3', '-1', '0', '1', '2', '3']}
          value_stereo={reports['inputLevel'] as [number, number]}
          label='VU (input)'
        />
      )}

      { convertToString(parameters['metering'][0], 'metering', conversionRules) === 'GR' && (  
        <HorizontalMeter
          scale={['-20', '-10', '-7', '-5', '-3', '-1', '0', '1', '2', '3']}
          value_stereo={reports['gainReduction'] as [number, number]}
          label='VU (gr)'
        />
      )}

      { convertToString(parameters['metering'][0], 'metering', conversionRules) === 'OUT' && (
        <HorizontalMeter
          scale={['-20', '-10', '-7', '-5', '-3', '-1', '0', '1', '2', '3']}
          value_stereo={reports['outputLevel'] as [number, number]}
          label='VU (output)'
        />
      )}
      
      <ContinuousKnob
        channel={channelIndex}
        value={convertToNumber(parameters['input'][channelIndex], 'input', conversionRules)}
        range={[-20, 0]}
        parameter='input'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='LEFT INPUT'
        unit='dB'
      />

      <ContinuousKnob
        channel={channelIndex}
        value={convertToNumber(parameters['threshold'][channelIndex], 'threshold', conversionRules)}
        range={[0, 10]}
        parameter='threshold'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='THRESHOLD'
        unit='dB'
      />

      <ContinuousKnob
        channel={channelIndex}
        value={convertToNumber(parameters['ratio'][channelIndex], 'ratio', conversionRules)}
        range={[0, 10]}
        parameter='ratio'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='RATIO'
        unit='dB'
      />

      <SteppedKnob
        channel={channelIndex}
        value={convertToNumber(parameters['timeConstant'][channelIndex], 'timeConstant', conversionRules)}
        range={[1, 6]}
        step={1}
        parameter='timeConstant'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='TIME CONSTANT'
      />

    </div>
  )
}

export default Channel