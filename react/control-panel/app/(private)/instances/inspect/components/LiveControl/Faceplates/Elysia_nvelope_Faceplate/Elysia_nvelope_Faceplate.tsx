import React, { useEffect, useState } from 'react'
import { InstanceReportsType } from '@/types'
import { Container, ChannelToggleButton, ListKnob, InstanceToggleButton, ToggleButton, ContinuousKnob } from '@moonlight-neon-ui'
import { InstanceParameters, ParameterId } from '@audiocloud/api'
import { convertToNumber, convertToString, convertToBoolean, ConversionRules } from '../valueConverters'

type Props = {
  channelIds: string[],
  parameters: InstanceParameters,
  wet: number,
  reports: InstanceReportsType,
  webSocketDefaultParametersSetter: (interfaceOnlyParams: ParameterId[], conversionRules: ConversionRules) => void,
  channelParameterHandler: (value: string | number | boolean, channel: number, parameter: string, conversionRules: ConversionRules) => void,
  instanceParameterHandler: (value: string | number | boolean, parameter: string, conversionRules: ConversionRules) => void,
  interfaceParameterHandler: (value: string | number | boolean, parameter: string) => void
}

const Elysia_nvelope_Faceplate: React.FC<Props> = ({ channelIds, parameters, wet, reports, webSocketDefaultParametersSetter, channelParameterHandler, instanceParameterHandler, interfaceParameterHandler }) => {

  const freqAOptions = [ 20, 21, 22, 23, 25, 26, 27, 29, 30, 35, 42, 50, 60, 70, 80, 90, 105, 120, 140, 150, 165, 183, 200, 230, 260, 290, 330, 420, 510, 590, 790, 990, 1200, 1900, 2600, 3300, 4000, 5300, 6600, 8000 ]
  const freqSOptions = [ 50, 52, 55, 56, 57, 58, 60, 75, 88, 100, 130, 150, 180, 210, 240, 270, 300, 325, 350, 370, 395, 420, 455, 490, 525, 560, 600, 830, 1100, 1300, 1700, 2100, 2500, 3600, 4800, 5900, 7000, 9600, 12200, 15000 ]

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

      <div className='text-slate-200 pb-4 w-full text-center'>Elysia nvelope</div>

      <div className='flex flex-wrap justify-center items-center gap-5'>

        {/* LEFT SECTION */}
        <ContinuousKnob
          channel={0}
          value={convertToNumber(parameters['attack'][0], 'attack', conversionRules)}
          range={[-15, 15]}
          parameter='attack'
          parameterHandler={channelParameterHandlerWrapper}
          size='lg'
          label='ATTACK'
          unit='dB'
        />

        <ListKnob
          channel={0}
          value={convertToString(parameters['freqA'][0], 'freqA', conversionRules)}
          list={freqAOptions.map(item => item.toString())}
          parameter='freqA'
          parameterHandler={channelParameterHandlerWrapper}
          size='lg'
          label='Freq A'
          unit='Hz'
        />

        <ContinuousKnob
          channel={0}
          value={convertToNumber(parameters['sustain'][0], 'sustain', conversionRules)}
          range={[-15, 15]}
          parameter='sustain'
          parameterHandler={channelParameterHandlerWrapper}
          size='lg'
          label='Sustain'
          unit='dB'
        />

        <ListKnob
          channel={0}
          value={convertToString(parameters['freqS'][0], 'freqS', conversionRules)}
          list={freqSOptions.map(item => item.toString())}
          parameter='freqS'
          parameterHandler={channelParameterHandlerWrapper}
          size='lg'
          label='Freq S'
          unit='Hz'
        />

        {/* CENTER SECTION */}
        <div className='flex justify-center items-center gap-2'>
          <div className='flex flex-col justify-center items-center gap-2'>
            <ChannelToggleButton
              channel={0}
              parameter='lrOn'
              parameterHandler={channelParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['lrOn'][0], 'lrOn', conversionRules)}
              className='w-32'
            >Left On</ChannelToggleButton>
            <ChannelToggleButton
              channel={0}
              parameter='eqMode'
              parameterHandler={channelParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['eqMode'][0], 'eqMode', conversionRules)}
              className='w-32'
            >EQ Mode</ChannelToggleButton>
            <ChannelToggleButton
              channel={0}
              parameter='fullRange'
              parameterHandler={channelParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['fullRange'][0], 'fullRange', conversionRules)}
              className='w-32'
            >Full Range</ChannelToggleButton>
            <ToggleButton
              toggled={link}
              setToggled={setLink}
              className='w-32'
            >LINK</ToggleButton>
          </div>
          <div className='flex flex-col justify-center items-center gap-2'>
            <ChannelToggleButton
              channel={1}
              parameter='lrOn'
              parameterHandler={channelParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['lrOn'][1], 'lrOn', conversionRules)}
              className='w-32'
            >Right On</ChannelToggleButton>
            <ChannelToggleButton
              channel={1}
              parameter='eqMode'
              parameterHandler={channelParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['eqMode'][1], 'eqMode', conversionRules)}
              className='w-32'
            >EQ Mode</ChannelToggleButton>
            <ChannelToggleButton
              channel={1}
              parameter='fullRange'
              parameterHandler={channelParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['fullRange'][1], 'fullRange', conversionRules)}
              className='w-32'
            >Full Range</ChannelToggleButton>
            <InstanceToggleButton
              parameter='autoGain'
              parameterHandler={instanceParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['autoGain'][0], 'autoGain', conversionRules)}
              className='w-32'
            >Auto Gain</InstanceToggleButton>
          </div>
        </div>

        {/* RIGHT SECTION */}
        <ContinuousKnob
          channel={1}
          value={convertToNumber(parameters['attack'][1], 'attack', conversionRules)}
          range={[-15, 15]}
          parameter='attack'
          parameterHandler={channelParameterHandlerWrapper}
          size='lg'
          label='ATTACK'
          unit='dB'
        />

        <ListKnob
          channel={1}
          value={convertToString(parameters['freqA'][1], 'freqA', conversionRules)}
          list={freqAOptions.map(item => item.toString())}
          parameter='freqA'
          parameterHandler={channelParameterHandlerWrapper}
          size='lg'
          label='Freq A'
          unit='Hz'
        />

        <ContinuousKnob
          channel={1}
          value={convertToNumber(parameters['sustain'][1], 'sustain', conversionRules)}
          range={[-15, 15]}
          parameter='sustain'
          parameterHandler={channelParameterHandlerWrapper}
          size='lg'
          label='Sustain'
          unit='dB'
        />

        <ListKnob
          channel={1}
          value={convertToString(parameters['freqS'][1], 'freqS', conversionRules)}
          list={freqSOptions.map(item => item.toString())}
          parameter='freqS'
          parameterHandler={channelParameterHandlerWrapper}
          size='lg'
          label='Freq S'
          unit='Hz'
        />

      </div>

    </Container>
  )
}

export default Elysia_nvelope_Faceplate