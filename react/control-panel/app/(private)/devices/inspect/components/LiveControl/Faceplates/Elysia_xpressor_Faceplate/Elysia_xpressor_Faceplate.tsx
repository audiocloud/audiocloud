import React, { useEffect, useState } from 'react'
import { DeviceReportsType } from '@/types'
import { Container, SteppedKnob, ChannelToggleButton, HorizontalMeter, ListKnob } from '@moonlight-neon-ui'
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

const Elysia_xpressor_Faceplate: React.FC<Props> = ({ channelIds, parameters, wet, reports, webSocketDefaultParametersSetter, channelParameterHandler, instanceParameterHandler, interfaceParameterHandler }) => {

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

      <div className='text-slate-200 pb-4 w-full text-center'>Elysia xpressor</div>

      <div className='flex flex-col justify-center items-center gap-5'>

        <HorizontalMeter
          scale={['14', '13', '12', '11', '10', '9', '8', '7', '6', '5', '4', '3', '2', '1']}
          value_stereo={reports['gainReduction'] as [number, number]}
          label='GR'
        />

        <div className='flex flex-wrap justify-center items-center gap-5'>

          {/* LEFT SECTION */}

          <SteppedKnob
            value={convertToNumber(parameters['threshold'][0], 'threshold', conversionRules)}
            range={[-42, 22]}
            step={1}
            parameter='threshold'
            parameterHandler={channelParameterHandlerWrapper}
            size='lg'
            label='THRESHOLD'
            unit='dB'
          />
          
          <ListKnob
            value={convertToString(parameters['attack'][0], 'attack', conversionRules)}
            list={[0, 0.01, 0.5, 3.5, 7, 10, 15, 25, 40, 60, 80, 100, 120].map(item => item.toString())}
            parameter='attack'
            parameterHandler={channelParameterHandlerWrapper}
            size='lg'
            label='ATTACK'
            unit='ms'
          />

          <ListKnob
            value={convertToString(parameters['release'][0], 'release', conversionRules)}
            list={[0, 5, 7, 12, 25, 50, 85, 140, 250, 400, 700, 1000, 1300].map(item => item.toString())}
            parameter='release'
            parameterHandler={channelParameterHandlerWrapper}
            size='lg'
            label='RELEASE'
            unit='ms'
          />

          <ListKnob
            value={convertToString(parameters['ratio'][0], 'ratio', conversionRules)}
            list={[0, 1.1, 1.3, 1.6, 2, 3.5, 5, 15, 1000, -0.4, -0.8, -1.2, -1.6].map(item => item.toString())}
            parameter='ratio'
            parameterHandler={channelParameterHandlerWrapper}
            size='lg'
            label='RATIO'
            unit='1:X'
          />

          {/* CENTER SECTION */}
          <div className='flex flex-col justify-center items-center gap-2'>
            <ChannelToggleButton
              channel={0}
              parameter='hitIt'
              parameterHandler={channelParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['hitIt'][0], 'hitIt', conversionRules)}
              className='w-32'
            >Hit It!</ChannelToggleButton>

            <ChannelToggleButton
              channel={0}
              parameter='warmMode'
              parameterHandler={channelParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['warmMode'][0], 'warmMode', conversionRules)}
              className='w-32'
            >Warm Mode</ChannelToggleButton>

            <ChannelToggleButton
              channel={0}
              parameter='logRel'
              parameterHandler={channelParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['logRel'][0], 'logRel', conversionRules)}
              className='w-32'
            >Log Rel</ChannelToggleButton>

            <ChannelToggleButton
              channel={0}
              parameter='autoFast'
              parameterHandler={channelParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['autoFast'][0], 'autoFast', conversionRules)}
              className='w-32'
            >Auto Fast</ChannelToggleButton>
          </div>

          {/* RIGHT SECTION */}

          <ListKnob
            value={convertToString(parameters['scf'][0], 'scf', conversionRules)}
            list={[31, 35, 40, 80, 120, 160, 200, 300, 400, 600, 800, 1000].map(item => item.toString())}
            parameter='scf'
            parameterHandler={channelParameterHandlerWrapper}
            size='lg'
            label='SCF'
            unit='Hz'
          />

          <ListKnob
            value={convertToString(parameters['grl'][0], 'grl', conversionRules)}
            list={[30, 26, 22, 18, 14, 11, 9, 7.5, 6, 3.5, 2, 1.5].map(item => item.toString())}
            parameter='grl'
            parameterHandler={channelParameterHandlerWrapper}
            size='lg'
            label='GRL'
            unit='dB'
          />

          <ListKnob
            value={convertToString(parameters['gain'][0], 'gain', conversionRules)}
            list={[0, 0.2, 1.5, 3, 4, 5.5, 7, 9, 11, 15, 19, 21].map(item => item.toString())}
            parameter='gain'
            parameterHandler={channelParameterHandlerWrapper}
            size='lg'
            label='GAIN'
            unit='dB'
          />

          <SteppedKnob
            value={convertToNumber(parameters['mix'][0], 'mix', conversionRules)}
            range={[0, 100]}
            step={5}
            parameter='mix'
            parameterHandler={channelParameterHandlerWrapper}
            size='lg'
            label='MIX'
            unit='%'
          />

        </div>
        
      </div>

    </Container>
  )
}

export default Elysia_xpressor_Faceplate