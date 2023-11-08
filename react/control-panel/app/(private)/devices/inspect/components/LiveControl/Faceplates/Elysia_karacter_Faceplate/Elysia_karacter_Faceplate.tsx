import React, { useEffect, useState } from 'react'
import { DeviceReportsType } from '@/types'
import { Container, ChannelToggleButton, ListKnob, InstanceToggleButton } from '@moonlight-neon-ui'
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

const Elysia_karacter_Faceplate: React.FC<Props> = ({ channelIds, parameters, wet, reports, webSocketDefaultParametersSetter, channelParameterHandler, instanceParameterHandler, interfaceParameterHandler }) => {

  const [link, setLink] = useState<boolean>(parameters['link'][0] as boolean)

  const interfaceOnlyParams: string[] = ['link']

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

  useEffect(() => {
    interfaceParameterHandler(link, 'link')
  }, [link])

  return (
    <Container className='w-full p-4 relative rounded-lg'>

      <div className='text-slate-200 pb-4 w-full text-center'>Elysia karacter</div>

      <div className='flex flex-wrap justify-center items-center gap-2'>

        {/* LEFT SECTION */}
        <ListKnob
          channel={0}
          value={convertToString(parameters['drive'][0], 'drive', conversionRules)}
          list={[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11].map(item => item.toString())}
          parameter='drive'
          parameterHandler={channelParameterHandlerWrapper}
          size='lg'
          label='Drive'
          unit='Warp'
        />
        <ListKnob
          channel={0}
          value={convertToString(parameters['color'][0], 'color', conversionRules)}
          list={[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11].map(item => item.toString())}
          parameter='color'
          parameterHandler={channelParameterHandlerWrapper}
          size='lg'
          label='Color'
        />
        <ListKnob
          channel={0}
          value={convertToString(parameters['gain'][0], 'gain', conversionRules)}
          list={[-11, -9, -7, -5, -3, -1, 1, 3, 5, 7, 9, 11].map(item => item.toString())}
          parameter='gain'
          parameterHandler={channelParameterHandlerWrapper}
          size='lg'
          label='Gain'
          unit='dB'
        />
        <ListKnob
          channel={0}
          value={convertToString(parameters['mix'][0], 'mix', conversionRules)}
          list={[0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110].map(item => item.toString())}
          parameter='mix'
          parameterHandler={channelParameterHandlerWrapper}
          size='lg'
          label='Mix'
          unit='%'
        />

        {/* CENTER SECTION */}
        <div className='flex justify-center items-center gap-2'>
          <div className='flex flex-col justify-center items-center gap-2'>
            <ChannelToggleButton
              channel={0}
              parameter='enabled'
              parameterHandler={channelParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['enabled'][0], 'enabled', conversionRules)}
              className='w-32'
            >Left On</ChannelToggleButton>
            <ChannelToggleButton
              channel={0}
              parameter='fetShred'
              parameterHandler={channelParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['fetShred'][0], 'fetShred', conversionRules)}
              className='w-32'
            >FET Shred</ChannelToggleButton>
            <ChannelToggleButton
              channel={0}
              parameter='turboBoost'
              parameterHandler={channelParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['turboBoost'][0], 'turboBoost', conversionRules)}
              className='w-32'
            >Turbo Boost</ChannelToggleButton>
            <InstanceToggleButton
              parameter='link'
              parameterHandler={instanceParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['link'][0], 'link', conversionRules)}
              className='w-32'
            >Stereo Link</InstanceToggleButton>
          </div>
          <div className='flex flex-col justify-center items-center gap-2'>
            <ChannelToggleButton
              channel={1}
              parameter='enabled'
              parameterHandler={channelParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['enabled'][1], 'enabled', conversionRules)}
              className='w-32'
            >Left On</ChannelToggleButton>
            <ChannelToggleButton
              channel={1}
              parameter='fetShred'
              parameterHandler={channelParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['fetShred'][1], 'fetShred', conversionRules)}
              className='w-32'
            >FET Shred</ChannelToggleButton>
            <ChannelToggleButton
              channel={1}
              parameter='turboBoost'
              parameterHandler={channelParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['turboBoost'][1], 'turboBoost', conversionRules)}
              className='w-32'
            >Turbo Boost</ChannelToggleButton>
            <InstanceToggleButton
              parameter='midSideEnabled'
              parameterHandler={instanceParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['midSideEnabled'][0], 'midSideEnabled', conversionRules)}
              className='w-32'
            >MS Mode</InstanceToggleButton>
          </div>
        </div>

        {/* RIGHT SECTION */}
        <ListKnob
          channel={1}
          value={convertToString(parameters['drive'][1], 'drive', conversionRules)}
          list={[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11].map(item => item.toString())}
          parameter='drive'
          parameterHandler={channelParameterHandlerWrapper}
          size='lg'
          label='Drive'
          unit='Warp'
        />
        <ListKnob
          channel={1}
          value={convertToString(parameters['color'][1], 'color', conversionRules)}
          list={[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11].map(item => item.toString())}
          parameter='color'
          parameterHandler={channelParameterHandlerWrapper}
          size='lg'
          label='Color'
        />
        <ListKnob
          channel={1}
          value={convertToString(parameters['gain'][1], 'gain', conversionRules)}
          list={[-11, -9, -7, -5, -3, -1, 1, 3, 5, 7, 9, 11].map(item => item.toString())}
          parameter='gain'
          parameterHandler={channelParameterHandlerWrapper}
          size='lg'
          label='Gain'
          unit='dB'
        />
        <ListKnob
          channel={1}
          value={convertToString(parameters['mix'][1], 'mix', conversionRules)}
          list={[0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110].map(item => item.toString())}
          parameter='mix'
          parameterHandler={channelParameterHandlerWrapper}
          size='lg'
          label='Mix'
          unit='%'
        />

      </div>

    </Container>
  )
}

export default Elysia_karacter_Faceplate