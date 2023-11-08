import React, { useEffect, useState } from 'react'
import { DeviceReportsType } from '@/types'
import { Container, ToggleButton, HorizontalDivider } from '@moonlight-neon-ui'
import { ConversionRules } from '../valueConverters'
import { InstanceParameters, ParameterId } from '@audiocloud/api'
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

const Elysia_museq_Faceplate: React.FC<Props> = ({ channelIds, parameters, wet, reports, webSocketDefaultParametersSetter, channelParameterHandler, instanceParameterHandler, interfaceParameterHandler }) => {

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

      <div className='text-slate-200 pb-4 w-full text-center'>Elysia museq</div>

      <div className='flex flex-col justify-between items-center gap-5'>

        <ToggleButton
          toggled={link}
          setToggled={setLink}
          className='w-32'
        >Link</ToggleButton>

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

export default Elysia_museq_Faceplate