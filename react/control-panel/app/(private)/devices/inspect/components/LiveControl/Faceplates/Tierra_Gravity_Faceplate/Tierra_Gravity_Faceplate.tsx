import React, { useEffect } from 'react'
import { DeviceReportsType } from '@/types'
import { Container, ContinuousKnob, SteppedKnob, ChannelToggleButton, HorizontalMeter, ChannelPillButtonSwitch } from '@moonlight-neon-ui'
import { InstanceParameters, ParameterId } from '@audiocloud/api'
import { convertToNumber, convertToString, convertToBoolean, ConversionRules } from '../valueConverters'

type Props = {
  channelIds: string[],
  parameters: InstanceParameters,
  wet: number,
  reports: DeviceReportsType,
  webSocketDefaultParametersSetter: (interfaceOnlyParams: ParameterId[], conversionRules: ConversionRules) => void,
  channelParameterHandler: (value: string | number | boolean, channel: number, parameter: string, conversionRules: ConversionRules) => void,
  instanceParameterHandler: (value: string | number | boolean, parameter: string, conversionRules: ConversionRules) => void,
  interfaceParameterHandler: (value: string | number | boolean, parameter: string) => void
}

const Tierra_Gravity_Faceplate: React.FC<Props> = ({ channelIds, parameters, wet, reports, webSocketDefaultParametersSetter, channelParameterHandler, instanceParameterHandler, interfaceParameterHandler }) => {

  const interfaceOnlyParams: string[] = []

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

      <div className='text-slate-200 pb-4 w-full text-center'>Tierra Gravity VCA Compressor</div>

      <div className='flex flex-wrap justify-center items-center gap-10'>

        <ContinuousKnob
          value={convertToNumber(parameters['threshold'][0], 'threshold', conversionRules)}
          range={[-15, 15]}
          parameter='threshold'
          parameterHandler={channelParameterHandlerWrapper}
          size='lg'
          label='THRESHOLD'
          unit='dB'
        />

        <ChannelPillButtonSwitch
          options={[2, 4, 10].map(item => item.toString())}
          value={convertToString(parameters['ratio'][0], 'ratio', conversionRules)}
          channel={0}
          parameter='ratio'
          parameterHandler={channelParameterHandlerWrapper}
          label={'RATIO'}
        />

        <SteppedKnob
          value={convertToNumber(parameters['attack'][0], 'attack', conversionRules)}
          range={[1, 6]}
          step={1}
          parameter='attack'
          parameterHandler={channelParameterHandlerWrapper}
          size='lg'
          label='ATTACK'
        />

        <SteppedKnob
          value={convertToNumber(parameters['release'][0], 'release', conversionRules)}
          range={[0, 4]}
          step={1}
          parameter='release'
          parameterHandler={channelParameterHandlerWrapper}
          size='lg'
          label='RELEASE'
        />

        <ContinuousKnob
          value={convertToNumber(parameters['makeUp'][0], 'makeUp', conversionRules)}
          range={[0, 15]}
          parameter='makeUp'
          parameterHandler={channelParameterHandlerWrapper}
          size='lg'
          label='MAKE UP'
          unit='dB'
        />

        <div className='flex flex-col justify-center items-center gap-2'>
          <ChannelToggleButton
            channel={0}
            parameter='bypass'
            parameterHandler={channelParameterHandlerWrapper}
            toggled={convertToBoolean(parameters['bypass'][0], 'bypass', conversionRules)}
            className='w-24'
          >BYPASS</ChannelToggleButton>

          <ChannelToggleButton
            channel={0}
            parameter='highpass'
            parameterHandler={channelParameterHandlerWrapper}
            toggled={convertToBoolean(parameters['highpass'][0], 'highpass', conversionRules)}
            className='w-24'
          >HPF SC</ChannelToggleButton>
        </div>

        <HorizontalMeter
          scale={['0', '4', '8', '12', '16', '20']}
          value_stereo={reports['gainReduction'] as [number, number]}
          label='GR'
        />

      </div>

    </Container>
  )
}

export default Tierra_Gravity_Faceplate