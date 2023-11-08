import React, { useEffect, useState } from 'react'
import { DeviceReportsType } from '@/types'
import { Container, HorizontalDivider, InstanceToggleButton, PillButtonSwitch, ToggleButton } from '@moonlight-neon-ui'
import { InstanceParameters, ParameterId } from '@audiocloud/api'
import { convertToBoolean, ConversionRules } from '../valueConverters'
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

const DFC670_Faceplate: React.FC<Props> = ({ channelIds, parameters, wet, reports, webSocketDefaultParametersSetter, channelParameterHandler, instanceParameterHandler, interfaceParameterHandler }) => {

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

      <div className='text-slate-200 pb-4 w-full text-center'>Distopik FC670</div>

      <div className='flex flex-col justify-between items-center gap-5'>
        
        <div className='flex flex-wrap justify-center items-center gap-5'>

          <PillButtonSwitch
            options={['IN', 'GR', 'OUT']}
            currentValue={metering}
            setValue={setMetering}
            label={'METERING'}
          />

          <InstanceToggleButton
            parameter='linkSideChain'
            parameterHandler={instanceParameterHandlerWrapper}
            toggled={convertToBoolean(parameters['linkSideChain'][0], 'linkSideChain', conversionRules)}
            className='w-40'
          >Link S. Chain</InstanceToggleButton>

          <ToggleButton
            toggled={link}
            setToggled={setLink}
            className='w-28'
          >Link</ToggleButton>

          <InstanceToggleButton
            parameter='midSideEnabled'
            parameterHandler={instanceParameterHandlerWrapper}
            toggled={convertToBoolean(parameters['midSideEnabled'][0], 'midSideEnabled', conversionRules)}
            className='w-28'
          >M/S</InstanceToggleButton>

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

export default DFC670_Faceplate