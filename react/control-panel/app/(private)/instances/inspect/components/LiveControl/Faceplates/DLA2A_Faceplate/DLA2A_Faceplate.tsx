import React, { useEffect, useState } from 'react'
import { InstanceReports } from '@/types'
import { Container, InstanceToggleButton, ToggleButton, HorizontalDivider, PillButtonSwitch } from '@moonlight-neon-ui'
import { InstanceParameters, ParameterId } from '@audiocloud/api'
import { ConversionRules, convertToBoolean } from '../valueConverters'
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

const DLA2A_Faceplate: React.FC<Props> = ({ channelIds, parameters, wet, reports, webSocketDefaultParametersSetter, channelParameterHandler, instanceParameterHandler, interfaceParameterHandler }) => {

  const [link, setLink] = useState<boolean>(parameters['link'][0] as boolean)
  const [metering, setMetering] = useState<string>(parameters['metering'][0] as string)

  const interfaceOnlyParams: string[] = ['link', 'metering']

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

  useEffect(() => {
    interfaceParameterHandler(metering, 'metering')
  }, [metering])

  return (
    <Container className='w-full p-4 relative rounded-lg'>

      <div className='text-slate-200 pb-4 w-full text-center'>Distopik LA-2A</div>

      <div className='flex flex-col justify-center items-center gap-5'>

        <div className='flex flex-wrap justify-center items-center gap-5'>

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
          >BYPASS</InstanceToggleButton>

          <PillButtonSwitch
            options={['GR', 'VU +4']}
            currentValue={metering}
            setValue={setMetering}
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

export default DLA2A_Faceplate