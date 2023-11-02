import React from 'react'
import { InstanceParameters } from '@audiocloud/api'
import { ContinuousKnob, ListKnob, ChannelToggleButton, HorizontalMeter, Header2, ChannelPillButtonSwitch } from '@moonlight-neon-ui'
import { convertToNumber, convertToString, convertToBoolean, ConversionRules } from '../valueConverters'
import { InstanceReports } from '@/types'

type Props = {
  channelIndex: number
  link: boolean,
  parameters: InstanceParameters,
  channelParameterHandler: (value: string | number | boolean, channel: number, parameter: string) => void,
  conversionRules: ConversionRules,
  reports: InstanceReports
}

const Channel: React.FC<Props> = ({ channelIndex, link, parameters, channelParameterHandler, conversionRules, reports }) => {

  const distortionTypeOptions = ['TR', 'PT', 'PK1', 'PK2', 'PK3', 'PK4']

  return (
    <div className='flex flex-wrap justify-center items-center gap-10'>

      { !link && <Header2>CH {channelIndex + 1}</Header2> }

      <div className='flex flex-col justify-center items-center gap-5'>

        <div className='flex flex-wrap justify-center items-center gap-5'>

          <ContinuousKnob
            channel={channelIndex}
            value={convertToNumber(parameters['drive'][channelIndex], 'drive', conversionRules)}
            range={[0, 11]}
            parameter='drive'
            parameterHandler={channelParameterHandler}
            size='lg'
            label='DRIVE'
          />

          <ListKnob
            channel={channelIndex}
            value={convertToString(parameters['distortionType'][channelIndex], 'distortionType', conversionRules)}
            list={distortionTypeOptions.map(item => item.toString())}
            parameter='distortionType'
            parameterHandler={channelParameterHandler}
            size='lg'
            label='DISTORTION TYPE'
          />

          <ContinuousKnob
            channel={channelIndex}
            value={convertToNumber(parameters['outputLevel'][channelIndex], 'outputLevel', conversionRules)}
            range={[0, 11]}
            parameter='outputLevel'
            parameterHandler={channelParameterHandler}
            size='lg'
            label='OUTPUT LEVEL'
            unit='dB'
          />

          <ContinuousKnob
            channel={channelIndex}
            value={convertToNumber(parameters['bias'][channelIndex], 'bias', conversionRules)}
            range={[0, 11]}
            parameter='bias'
            parameterHandler={channelParameterHandler}
            size='lg'
            label='BIAS'
            unit='dB'
          />

        </div>

        <div className='flex justify-center items-center gap-5'>

          <ChannelPillButtonSwitch
            options={['OFF', 'I', 'II']}
            value={convertToString(parameters['overDrive'][channelIndex], 'overDrive', conversionRules)}
            channel={channelIndex}
            parameter='overDrive'
            parameterHandler={channelParameterHandler}
            label={'Overdrive'}
          />

          <ChannelToggleButton
            channel={channelIndex}
            parameter='lpf'
            parameterHandler={channelParameterHandler}
            toggled={convertToBoolean(parameters['lpf'][channelIndex], 'lpf', conversionRules)}
            className='w-32'
          >LPF</ChannelToggleButton>

        </div>

      </div>

      <HorizontalMeter
        scale={['-30', '-28', '-26', '-24', '-22', '-20', '-18', '-16', '-14', '-12', '-10', '-8', '-6', '-4', '-2', '0']}
        value_stereo={reports['peakLevel'] as [number, number]}
        label='Peak'
      />

    </div>
  )
}

export default Channel