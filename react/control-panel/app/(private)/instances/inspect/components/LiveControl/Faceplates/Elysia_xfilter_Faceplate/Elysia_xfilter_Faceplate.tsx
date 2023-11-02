import React, { useEffect, useState } from 'react'
import { InstanceReports } from '@/types'
import { Container, ContinuousKnob, ChannelToggleButton, ListKnob } from '@moonlight-neon-ui'
import { InstanceParameters, ParameterId } from '@audiocloud/api'
import { convertToNumber, convertToString, convertToBoolean, ConversionRules } from '../valueConverters'

type Props = {
  channelIds: string[],
  parameters: InstanceParameters,
  wet: number,
  reports: InstanceReports,
  webSocketDefaultParametersSetter: (interfaceOnlyParams: ParameterId[], conversionRules: ConversionRules) => void,
  channelParameterHandler: (value: string | number | boolean, channel: number, parameter: string, conversionRules: ConversionRules) => void,
  instanceParameterHandler: (value: string | number | boolean, parameter: string, conversionRules: ConversionRules) => void,
  interfaceParameterHandler: (value: string | number | boolean, parameter: string) => void
}

const Elysia_xfilter_Faceplate: React.FC<Props> = ({ channelIds, parameters, wet, reports, webSocketDefaultParametersSetter, channelParameterHandler, instanceParameterHandler, interfaceParameterHandler }) => {

  const interfaceOnlyParams: string[] = ['']

  const conversionRules: ConversionRules = {
    'stringToString': {},
    'stringToNumber': {},
    'stringToBoolean': {},
    'numberToString': {},
    'numberToNumber': {},
    'numberToBoolean': {},
    'booleanToString': {},
    'booleanToNumber': {},
    'booleanToBoolean': {}
  }

  const channelParameterHandlerWrapper = (value: string | number | boolean, channel: number, parameter: string) => channelParameterHandler(value, channel, parameter, conversionRules)
  const instanceParameterHandlerWrapper = (value: string | number | boolean, parameter: string) => instanceParameterHandler(value, parameter, conversionRules)

  useEffect(() => {
    webSocketDefaultParametersSetter(interfaceOnlyParams, conversionRules)
  }, [])

  return (
    <Container className='w-full p-4 relative rounded-lg'>

      <div className='text-slate-200 pb-4 w-full text-center'>Elysia xfilter</div>

      <div className='flex flex-wrap flex-col xl:flex-row justify-center items-center gap-10 2xl:gap-3'>

        {/* LEFT SECTION */}
        <div className='flex flex-wrap justify-center items-center gap-2'>
          <ContinuousKnob
            value={convertToNumber(parameters['lowGain'][0], 'lowGain', conversionRules)}
            range={[-16, 16]}
            parameter='lowGain'
            parameterHandler={channelParameterHandlerWrapper}
            size='lg'
            label='LOW GAIN'
            unit='dB'
          />

          <ListKnob
            value={convertToString(parameters['lowFreq'][0], 'lowFreq', conversionRules)}
            list={[20, 25, 30, 40, 60, 80, 100, 150, 200, 400, 600, 900].map(item => item.toString())}
            parameter='lowFreq'
            parameterHandler={channelParameterHandlerWrapper}
            size='lg'
            label='LOW FREQ'
            unit='Hz'
          />

          <ContinuousKnob
            value={convertToNumber(parameters['lowMidGain'][0], 'lowMidGain', conversionRules)}
            range={[-13, 13]}
            parameter='lowMidGain'
            parameterHandler={channelParameterHandlerWrapper}
            size='lg'
            label='LOW MID GAIN'
            unit='dB'
          />

          <ListKnob
            value={convertToString(parameters['lowMidFreq'][0], 'lowMidFreq', conversionRules)}
            list={[45, 50, 70, 100, 150, 200, 300, 400, 600, 1000, 1500, 2200].map(item => item.toString())}
            parameter='lowMidFreq'
            parameterHandler={channelParameterHandlerWrapper}
            size='lg'
            label='LOW MID FREQ'
            unit='Hz'
          />
        </div>

        {/* CENTER SECTION */}
        <div className='flex justify-center items-center gap-2'>
          <div className='flex flex-col justify-center items-center gap-2'>
            <ChannelToggleButton
              channel={0}
              parameter='hitIt'
              parameterHandler={channelParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['hitIt'][0], 'hitIt', conversionRules)}
              className='w-40'
            >Hit It!</ChannelToggleButton>

            <ChannelToggleButton
              channel={0}
              parameter='lowCut'
              parameterHandler={channelParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['lowCut'][0], 'lowCut', conversionRules)}
              className='w-40'
            >Low Cut</ChannelToggleButton>

            <ChannelToggleButton
              channel={0}
              parameter='lowMidNarrowQ'
              parameterHandler={channelParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['lowMidNarrowQ'][0], 'lowMidNarrowQ', conversionRules)}
              className='w-40'
            >L-mid Narrow Q</ChannelToggleButton>
          </div>
          <div className='flex flex-col justify-center items-center gap-2'>
            <ChannelToggleButton
              channel={0}
              parameter='highMidNarrowQ'
              parameterHandler={channelParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['highMidNarrowQ'][0], 'highMidNarrowQ', conversionRules)}
              className='w-40'
            >H-mid Narrow Q</ChannelToggleButton>

            <ChannelToggleButton
              channel={0}
              parameter='highCut'
              parameterHandler={channelParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['highCut'][0], 'highCut', conversionRules)}
              className='w-40'
            >High Cut</ChannelToggleButton>

            <ChannelToggleButton
              channel={0}
              parameter='passiveMassage'
              parameterHandler={channelParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['passiveMassage'][0], 'passiveMassage', conversionRules)}
              className='w-40'
            >Passive Massage</ChannelToggleButton>
          </div>
        </div>

        {/* RIGHT SECTION */}
        <div className='flex flex-wrap justify-center items-center gap-2'>
          <ContinuousKnob
            value={convertToNumber(parameters['highMidGain'][0], 'highMidGain', conversionRules)}
            range={[-13, 13]}
            parameter='highMidGain'
            parameterHandler={channelParameterHandlerWrapper}
            size='lg'
            label='HIGH MID GAIN'
            unit='dB'
          />

          <ListKnob
            value={convertToString(parameters['highMidFreq'][0], 'highMidFreq', conversionRules)}
            list={[300, 350, 500, 1000, 1500, 2000, 2500, 3000, 4500, 8000, 12000, 16000].map(item => item.toString())}
            parameter='highMidFreq'
            parameterHandler={channelParameterHandlerWrapper}
            size='lg'
            label='HIGH MID FREQ'
            unit='Hz'
          />

          <ContinuousKnob
            value={convertToNumber(parameters['highGain'][0], 'highGain', conversionRules)}
            range={[-16, 16]}
            parameter='highGain'
            parameterHandler={channelParameterHandlerWrapper}
            size='lg'
            label='HIGH GAIN'
            unit='dB'
          />

          <ListKnob
            value={convertToString(parameters['highFreq'][0], 'highFreq', conversionRules)}
            list={[700, 800, 1000, 2000, 3000, 4000, 5000, 7500, 10000, 16000, 22000, 24000].map(item => item.toString())}
            parameter='highFreq'
            parameterHandler={channelParameterHandlerWrapper}
            size='lg'
            label='HIGH FREQ'
            unit='Hz'
          />
        </div>
        
      </div>

    </Container>
  )
}

export default Elysia_xfilter_Faceplate