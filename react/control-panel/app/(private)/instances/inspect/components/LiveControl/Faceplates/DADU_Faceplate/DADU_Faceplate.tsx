import React, { useEffect, useState } from 'react'
import { InstanceReports } from '@/types'
import { Container, HorizontalDivider, InstanceToggleButton, ToggleButton } from '@moonlight-neon-ui'
import { InstanceParameters, ParameterId } from '@audiocloud/api'
import { convertToBoolean, ConversionRules } from '../valueConverters'
import Channel from './Channel'

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

const DADU_Faceplate: React.FC<Props> = ({ channelIds, parameters, wet, reports, webSocketDefaultParametersSetter, channelParameterHandler, instanceParameterHandler, interfaceParameterHandler }) => {

  const [link, setLink] = useState<boolean>(parameters['link'][0] as boolean)

  const interfaceOnlyParams: string[] = ['link']

  const conversionRules: ConversionRules = {
    'stringToString': {},
    'stringToNumber': {
      'distortionType': {
        'TR': 1,
        'PT': 2,
        'PK1': 3,
        'PK2': 4,
        'PK3': 5,
        'PK4': 6
      },
      'overDrive': {
        'OFF': 0,
        'I': 1,
        'II': 2
      }
    },
    'stringToBoolean': {},
    'numberToString': {
      'distortionType': {
        '1': 'TR',
        '2': 'PT',
        '3': 'PK1',
        '4': 'PK2',
        '5': 'PK3',
        '6': 'PK4'
      },
      'overDrive': {
        '0': 'OFF',
        '1': 'I',
        '2': 'II'
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

      <div className='text-slate-200 pb-4 w-full text-center'>Distopik ADU Tube Saturator</div>

      <div className='flex flex-col justify-center items-center gap-5'>

        <div className='flex justify-center items-center gap-5'>

          <InstanceToggleButton
            parameter='bypass'
            parameterHandler={instanceParameterHandlerWrapper}
            toggled={convertToBoolean(parameters['bypass'][0], 'bypass', conversionRules)}
            className='w-24'
          >BYPASS</InstanceToggleButton>

          <ToggleButton
            toggled={link}
            setToggled={setLink}
            className='w-24'
          >LINK</ToggleButton>

        </div>

        <HorizontalDivider />

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

    </Container>
  )
}

export default DADU_Faceplate