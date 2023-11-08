import React, { useEffect, useState } from 'react'
import { DeviceReportsType } from '@/types'
import { Container, ContinuousKnob, HorizontalMeter, InstancePillButtonSwitch, HorizontalDivider, ToggleButton } from '@moonlight-neon-ui'
import { InstanceParameters, ParameterId } from '@audiocloud/api'
import { convertToNumber, convertToString, convertToBoolean, ConversionRules } from '../valueConverters'
import Channel from './Channel'

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

const Gyraf_G24_Faceplate: React.FC<Props> = ({ channelIds, parameters, wet, reports, webSocketDefaultParametersSetter, channelParameterHandler, instanceParameterHandler, interfaceParameterHandler }) => {
  
  const [link, setLink] = useState<boolean>(parameters['link'][0] as boolean)

  const interfaceOnlyParams: string[] = ['link']

  const conversionRules: ConversionRules = {
    'stringToString': {},
    'stringToNumber': {
      'outputType': {
        'PASSIVE': 0,
        'OUT': 1,
        'ACTIVE': 2
      }
    },
    'stringToBoolean': {},
    'numberToString': {
      'outputType': {
        '0': 'PASSIVE',
        '1': 'OUT',
        '2': 'ACTIVE'
      }
    },
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

      <div className='text-slate-200 pb-4 w-full text-center'>Gyraf G24 Compressor</div>

      <div className='flex flex-col justify-between items-center gap-5'>
        
        <ToggleButton
          toggled={link}
          setToggled={setLink}
          className='w-24'
        >LINK</ToggleButton>
        
        <HorizontalDivider />

        <div className='flex flex-wrap justify-center items-center gap-2'>

          <div className='flex flex-col justify-center items-center gap-5'>
            <Channel
              channelIndex={0}
              link={link}
              channelParameterHandler={channelParameterHandlerWrapper}
              parameters={parameters}
              conversionRules={conversionRules}
              reports={reports}
            />

            { !link && (
              <>
                <HorizontalDivider />
                <Channel
                  channelIndex={1}
                  link={link}
                  channelParameterHandler={channelParameterHandlerWrapper}
                  parameters={parameters}
                  conversionRules={conversionRules}
                  reports={reports}
                />
              </>
            )}
          </div>
          
          <div className='flex justify-center items-center'>
            <ContinuousKnob
              value={convertToNumber(parameters['control'][0], 'control', conversionRules)}
              range={[0, 100]}
              parameter='control'
              parameterHandler={instanceParameterHandlerWrapper}
              size='lg'
              label='CONTROL'
              unit='%'
            />

            <div className='flex flex-col justify-center items-center gap-5'>

              <ContinuousKnob
                value={convertToNumber(parameters['emphasis'][0], 'emphasis', conversionRules)}
                range={[0, 100]}
                parameter='emphasis'
                parameterHandler={instanceParameterHandlerWrapper}
                size='lg'
                label='EMPHASIS'
              />

              <ContinuousKnob
                value={convertToNumber(parameters['output'][0], 'output', conversionRules)}
                range={[0, 100]}
                parameter='output'
                parameterHandler={instanceParameterHandlerWrapper}
                size='lg'
                label='OUTPUT'
                unit='dB'
              />

            </div>
          </div>

          <div className='flex flex-col justify-center items-center gap-2 my-4'>

            <HorizontalMeter
              scale={['-10', '-8', '-6', '-4', '-3', '-2', '-1', '0.5', '0']}
              value_stereo={reports['gainReduction'] as [number, number]}
              label='GR'
            />

            <InstancePillButtonSwitch
              options={['PASSIVE', 'OUT', 'ACTIVE']}
              value={convertToString(parameters['outputType'][0], 'outputType', conversionRules)}
              parameter='outputType'
              parameterHandler={instanceParameterHandlerWrapper}
              label={'OUTPUT TYPE'}
            />
              
          </div>
        </div>

      </div>

    </Container>
  )
}

export default Gyraf_G24_Faceplate