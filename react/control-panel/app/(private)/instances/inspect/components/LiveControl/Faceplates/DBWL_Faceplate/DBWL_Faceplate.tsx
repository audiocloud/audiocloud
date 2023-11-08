import React, { useEffect, useState } from 'react'
import { InstanceReportsType } from '@/types'
import { Container, ContinuousKnob, HorizontalMeter, InstanceToggleButton, InstancePillButtonSwitch, ToggleButton } from '@moonlight-neon-ui'
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

const DBWL_Faceplate: React.FC<Props> = ({ channelIds, parameters, wet, reports, webSocketDefaultParametersSetter, channelParameterHandler, instanceParameterHandler, interfaceParameterHandler }) => {

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

      <div className='text-slate-200 pb-4 w-full text-center'>Distopik Brickwall Limiter</div>

      <div className='flex flex-wrap justify-center items-center gap-5'>

        <div className='flex xl:flex-col gap-5'>

          <ContinuousKnob
            channel={0}
            value={convertToNumber(parameters['input'][0], 'input', conversionRules)}
            range={[0, 10]}
            parameter='input'
            parameterHandler={channelParameterHandlerWrapper}
            size='lg'
            label='INPUT L'
            unit='dB'
          />

          <ContinuousKnob
            channel={1}
            value={convertToNumber(parameters['input'][1], 'input', conversionRules)}
            range={[0, 10]}
            parameter='input'
            parameterHandler={channelParameterHandlerWrapper}
            size='lg'
            label='INPUT R'
            unit='dB'
          />

        </div>

        <div className='flex flex-col justify-center items-center gap-5'>

          <HorizontalMeter
            scale={['-30', '-28', '-26', '-24', '-22', '-20', '-18', '-16', '-14', '-12', '-10', '-8', '-6', '-4', '-2', '0']}
            value_stereo={reports['peakLevel'] as [number, number]}
            label='Peak'
          />

          <div className='flex flex-wrap justify-center items-center gap-10'>
          
            <div className='w-40'>
              <InstancePillButtonSwitch
                options={['JFET', 'MOSFET']}
                value={convertToString(parameters['fet'][0], 'fet', conversionRules)}
                parameter='fet'
                parameterHandler={instanceParameterHandlerWrapper}
              />
            </div>

            <ContinuousKnob
              value={convertToNumber(parameters['ceiling'][0], 'ceiling', conversionRules)}
              range={[22, 10]}
              parameter='ceiling'
              parameterHandler={instanceParameterHandlerWrapper}
              size='lg'
              label='CEILING'
              unit='dB'
            />

            <div className='w-40 flex flex-col justify-center items-center gap-2'>
              <ToggleButton
                toggled={link}
                setToggled={setLink}
                className='w-32'
              >LINK</ToggleButton>

              <InstanceToggleButton
                parameter='bypass'
                parameterHandler={instanceParameterHandlerWrapper}
                toggled={convertToBoolean(parameters['bypass'][0], 'bypass', conversionRules)}
                className='w-32'
              >BYPASS</InstanceToggleButton>
            </div>

          </div>

        </div>

        <div className='flex xl:flex-col gap-5'>
          
          <ContinuousKnob
            channel={0}
            value={convertToNumber(parameters['output'][0], 'output', conversionRules)}
            range={[0, 10]}
            parameter='output'
            parameterHandler={channelParameterHandlerWrapper}
            size='lg'
            label='OUTPUT L'
            unit='dB'
          />

          <ContinuousKnob
            channel={1}
            value={convertToNumber(parameters['output'][1], 'output', conversionRules)}
            range={[0, 10]}
            parameter='output'
            parameterHandler={channelParameterHandlerWrapper}
            size='lg'
            label='OUTPUT R'
            unit='dB'
          />

        </div>

      </div>

    </Container>

  )
}

export default DBWL_Faceplate