import React, { useEffect, useState } from 'react'
import { InstanceReports } from '@/types'
import { Container, HorizontalDivider, InstancePillButtonSwitch, ToggleButton } from '@moonlight-neon-ui'
import { InstanceParameters, ParameterId } from '@audiocloud/api'
import { convertToString, ConversionRules } from '../valueConverters'
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

const D1176_Faceplate: React.FC<Props> = ({ channelIds, parameters, wet, reports, webSocketDefaultParametersSetter, channelParameterHandler, instanceParameterHandler, interfaceParameterHandler }) => {
  
  const [link, setLink] = useState<boolean>(parameters['link'][0] as boolean)
  
  const interfaceOnlyParams: string[] = ['link']

  const conversionRules: ConversionRules = {
    'stringToString': {},
    'stringToNumber': {
      'amplifierMode': {
        'a': 1,
        'd': 2,
        'g': 3
      }
    },
    'stringToBoolean': {},
    'numberToString': {
      'amplifierMode': {
        '1': 'a',
        '2': 'd',
        '3': 'g'
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

      <div className='text-slate-200 pb-4 w-full text-center'>Distopik Stereo 1176 A/D/G</div>

      <div className='flex flex-col justify-between items-center gap-5'>
        
        <div className='flex justify-center items-center gap-5'>
          <ToggleButton
            toggled={link}
            setToggled={setLink}
            className='w-24'
          >LINK</ToggleButton>

          <InstancePillButtonSwitch
            options={['a', 'd', 'g']}
            value={convertToString(parameters['amplifierMode'][0], 'amplifierMode', conversionRules)}
            parameter='amplifierMode'
            parameterHandler={instanceParameterHandlerWrapper}
            label={'MODE'}
          />
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

export default D1176_Faceplate