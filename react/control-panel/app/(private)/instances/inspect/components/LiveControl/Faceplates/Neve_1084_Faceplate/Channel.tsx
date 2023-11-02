import React from 'react'
import { InstanceParameters } from '@audiocloud/api'
import { ChannelToggleButton, Header2, ListKnob, SteppedKnob } from '@moonlight-neon-ui'
import { convertToNumber, convertToString, convertToBoolean, ConversionRules } from '../valueConverters'

type Props = {
  channelIndex: number
  link: boolean,
  parameters: InstanceParameters,
  channelParameterHandler: (value: string | number | boolean, channel: number, parameter: string) => void,
  conversionRules: ConversionRules
}

const Channel: React.FC<Props> = ({ channelIndex, link, parameters, channelParameterHandler, conversionRules }) => {

  const inputGainList = [ 'OFF', -10, -5, 0, 5, 10, 15, 20 ]
  const highPassFilterList = [ 'OFF', 22, 45, 70, 160, 360 ]

  return (
    <div className='flex flex-wrap justify-center items-center gap-10'>

    { !link && <Header2>CH {channelIndex + 1}</Header2> }

    <ChannelToggleButton
      channel={channelIndex}
      parameter='eql'
      parameterHandler={channelParameterHandler}
      toggled={convertToBoolean(parameters['eql'][channelIndex], 'eql', conversionRules)}
      className='w-24'
    >EQL</ChannelToggleButton>

    <ListKnob
      channel={channelIndex}
      value={convertToString(parameters['inputGain'][channelIndex], 'inputGain', conversionRules)}
      list={inputGainList.map(item => item.toString())}
      parameter='inputGain'
      parameterHandler={channelParameterHandler}
      size='lg'
      label='GAIN'
      unit='dB'
    />

    <div className='flex flex-col self-start justify-center items-center gap-2'>
      <SteppedKnob
        channel={channelIndex}
        value={convertToNumber(parameters['highShelfGain'][channelIndex], 'highShelfGain', conversionRules)}
        range={[-8, 8]}
        step={1}
        parameter='highShelfGain'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='HIGH SHELF GAIN'
        unit='dB'
      />
      <SteppedKnob
        channel={channelIndex}
        value={convertToNumber(parameters['highShelfFreq'][channelIndex], 'highShelfFreq', conversionRules)}
        range={[-8, 8]}
        step={1}
        parameter='highShelfFreq'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='HIGH SHELF FREQ'
        unit='Hz'
      />
    </div>

    <div className='flex flex-col justify-center items-center gap-2'>
      <SteppedKnob
        channel={channelIndex}
        value={convertToNumber(parameters['highBellGain'][channelIndex], 'highBellGain', conversionRules)}
        range={[-8, 8]}
        step={1}
        parameter='highBellGain'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='HIGH BELL GAIN'
        unit='dB'
      />
      <SteppedKnob
        channel={channelIndex}
        value={convertToNumber(parameters['highBellFreq'][channelIndex], 'highBellFreq', conversionRules)}
        range={[-8, 8]}
        step={1}
        parameter='highBellFreq'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='HIGH BELL FREQ'
        unit='Hz'
      />
      <ChannelToggleButton
        channel={channelIndex}
        parameter='highBellHIQ'
        parameterHandler={channelParameterHandler}
        toggled={convertToBoolean(parameters['highBellHIQ'][channelIndex], 'highBellHIQ', conversionRules)}
        className='w-24'
      >HI Q</ChannelToggleButton>
    </div>

    <div className='flex flex-col justify-center items-center gap-2'>
      <SteppedKnob
        channel={channelIndex}
        value={convertToNumber(parameters['lowBellGain'][channelIndex], 'lowBellGain', conversionRules)}
        range={[-8, 8]}
        step={1}
        parameter='lowBellGain'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='LOW BELL GAIN'
        unit='dB'
      />
      <SteppedKnob
        channel={channelIndex}
        value={convertToNumber(parameters['lowBellFreq'][channelIndex], 'lowBellFreq', conversionRules)}
        range={[-8, 8]}
        step={1}
        parameter='lowBellFreq'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='LOG BELL FREQ'
        unit='Hz'
      />
      <ChannelToggleButton
        channel={channelIndex}
        parameter='lowBellHIQ'
        parameterHandler={channelParameterHandler}
        toggled={convertToBoolean(parameters['lowBellHIQ'][channelIndex], 'lowBellHIQ', conversionRules)}
        className='w-24'
      >HI Q</ChannelToggleButton>
    </div>

    <div className='flex flex-col justify-center items-center gap-2'>
      <SteppedKnob
        channel={channelIndex}
        value={convertToNumber(parameters['lowShelfGain'][channelIndex], 'lowShelfGain', conversionRules)}
        range={[-8, 8]}
        step={1}
        parameter='lowShelfGain'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='LOW SHELF GAIN'
        unit='dB'
      />
      <SteppedKnob
        channel={channelIndex}
        value={convertToNumber(parameters['lowShelfFreq'][channelIndex], 'lowShelfFreq', conversionRules)}
        range={[-8, 8]}
        step={1}
        parameter='lowShelfFreq'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='LOW SHELF FREQ'
        unit='Hz'
      />
      <ChannelToggleButton
        channel={channelIndex}
        parameter='loweShelfLOZ'
        parameterHandler={channelParameterHandler}
        toggled={convertToBoolean(parameters['loweShelfLOZ'][channelIndex], 'loweShelfLOZ', conversionRules)}
        className='w-24'
      >LO Z</ChannelToggleButton>
    </div>

    <ListKnob
      channel={channelIndex}
      value={convertToString(parameters['hpf'][channelIndex], 'hpf', conversionRules)}
      list={highPassFilterList.map(item => item.toString())}
      parameter='hpf'
      parameterHandler={channelParameterHandler}
      size='lg'
      label='HPF'
      unit='Hz'
    />

  </div>
  )
}

export default Channel