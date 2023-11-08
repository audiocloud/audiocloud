import React, { useEffect, useState } from 'react'
import { DeviceReportsType } from '@/types'
import { Container, HorizontalDivider, HorizontalMeter, InstanceToggleButton, ToggleButton } from '@moonlight-neon-ui'
import { InstanceParameters, ParameterId } from '@audiocloud/api'
import { ConversionRules, convertToBoolean } from '../valueConverters'
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

const DVCA_Faceplate: React.FC<Props> = ({ channelIds, parameters, wet, reports, webSocketDefaultParametersSetter, channelParameterHandler, instanceParameterHandler, interfaceParameterHandler }) => {

  const [link, setLink] = useState<boolean>(parameters['link'][0] as boolean)

  const interfaceOnlyParams: string[] = ['link']

  const conversionRules: ConversionRules = {
    'stringToString': {},
    'stringToNumber': {
      'release': {
        'auto': 1
      },
      'scf': {
        'TL': 1,
        'TM': 2,
        'OFF': 0
      }
    },
    'stringToBoolean': {},
    'numberToString': {
      'release': {
        '1': 'auto'
      },
      'scf': {
        '1': 'TL',
        '2': 'TM',
        '0': 'OFF'
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

      <div className='text-slate-200 pb-4 w-full text-center'>Distopik VCA Compressor</div>
      
      <div className='flex flex-col justify-center items-center gap-5'>

        <div className='flex flex-wrap justify-center items-center gap-5'>

          <HorizontalMeter
            scale={['-30', '-28', '-26', '-24', '-22', '-20', '-18', '-16', '-14', '-12', '-10', '-8', '-6', '-4', '-2', '0']}
            value_stereo={reports['gainReduction'] as [number, number]}
            label='dB COMPRESSION'
          />

          <div className='flex flex-col justify-center items-center gap-2'>

            <ToggleButton
              toggled={link}
              setToggled={setLink}
              className='w-28'
            >Link</ToggleButton>
            
            <InstanceToggleButton
              parameter='bypass'
              parameterHandler={instanceParameterHandlerWrapper}
              toggled={convertToBoolean(parameters['bypass'][0], 'bypass', conversionRules)}
              className='w-28'
            >Bypass</InstanceToggleButton>

          </div>

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

export default DVCA_Faceplate