import React from 'react'
import { InstanceParameters } from '@audiocloud/api'
import { ChannelPillButtonSwitch, ContinuousKnob, Header2, ListKnob } from '@moonlight-neon-ui'
import { convertToNumber, convertToString, ConversionRules } from '../valueConverters'
import { DeviceReportsType } from '@/types'

type Props = {
  channelIndex: number
  link: boolean,
  parameters: InstanceParameters,
  channelParameterHandler: (value: string | number | boolean, channel: number, parameter: string) => void,
  conversionRules: ConversionRules,
  reports: DeviceReportsType
}

const Channel: React.FC<Props> = ({ channelIndex, link, parameters, channelParameterHandler, conversionRules, reports }) => {

  const lowFreqOptions = [20, 30, 60, 100]
  const midFreqOptions = [200, 400, 600, 800, 1000, 2000]
  const highFreqOptions = [3000, 4000, 5000, 8000, 10000, 12000, 16000]
  const attenuationSelectionOptions = [5, 10, 20]

  return (
    <div className='flex flex-wrap justify-center items-center gap-10'>

      { !link && <Header2>CH {channelIndex + 1}</Header2> }

      <ContinuousKnob
        channel={channelIndex}
        value={convertToNumber(parameters['bandwidth'][channelIndex], 'bandwidth', conversionRules)}
        range={[0, 10]}
        parameter='bandwidth'
        parameterHandler={channelParameterHandler}
        size='lg'
        label='BANDWIDTH'
      />

      <div className='flex flex-col justify-between items-center gap-5'>
        
        <div className='flex justify-center items-center gap-5'>
          <ContinuousKnob
            channel={channelIndex}
            value={convertToNumber(parameters['lowBoost'][channelIndex], 'lowBoost', conversionRules)}
            range={[0, 10]}
            parameter='lowBoost'
            parameterHandler={channelParameterHandler}
            size='lg'
            label='LOW BOOST'
            unit='dB'
          />
          <ContinuousKnob
            channel={channelIndex}
            value={convertToNumber(parameters['lowAttenuation'][channelIndex], 'lowAttenuation', conversionRules)}
            range={[0, 10]}
            parameter='lowAttenuation'
            parameterHandler={channelParameterHandler}
            size='lg'
            label='LOW ATTEN'
            unit='dB'
          />
        </div>
        <ListKnob
          channel={channelIndex}
          value={convertToString(parameters['lowFrequency'][channelIndex], 'lowFrequency', conversionRules)}
          list={lowFreqOptions.map(item => item.toString())}
          parameter='lowFrequency'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='LOW FREQ'
          unit='Hz'
        />
      </div>

      <div className='flex flex-col justify-between items-center gap-5'>
        <div className='flex gap-5'>
          <ContinuousKnob
            channel={channelIndex}
            value={convertToNumber(parameters['midBoost'][channelIndex], 'midBoost', conversionRules)}
            range={[0, 10]}
            parameter='midBoost'
            parameterHandler={channelParameterHandler}
            size='lg'
            label='MID BOOST'
            unit='dB'
          />
          <ContinuousKnob
            channel={channelIndex}
            value={convertToNumber(parameters['midAttenuation'][channelIndex], 'midAttenutation', conversionRules)}
            range={[0, 10]}
            parameter='midAttenuation'
            parameterHandler={channelParameterHandler}
            size='lg'
            label='MID ATTEN'
            unit='dB'
          />
        </div>
        <ListKnob
          channel={channelIndex}
          value={convertToString(parameters['midFrequency'][channelIndex], 'midFrequency', conversionRules)}
          list={midFreqOptions.map(item => item.toString())}
          parameter='midFrequency'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='MID FREQ'
          unit='Hz'
        />
      </div>

      <div className='flex flex-col justify-between items-center gap-5'>
        <div className='flex gap-5'>
          <ContinuousKnob
            channel={channelIndex}
            value={convertToNumber(parameters['highBoost'][channelIndex], 'highBoost', conversionRules)}
            range={[0, 10]}
            parameter='highBoost'
            parameterHandler={channelParameterHandler}
            size='lg'
            label='HIGH BOOST'
            unit='dB'
          />
          <ContinuousKnob
            channel={channelIndex}
            value={convertToNumber(parameters['highAttenuation'][channelIndex], 'highAttenuation', conversionRules)}
            range={[0, 10]}
            parameter='highAttenuation'
            parameterHandler={channelParameterHandler}
            size='lg'
            label='HIGH ATTEN'
            unit='dB'
          />
        </div>

        <ListKnob
          channel={channelIndex}
          value={convertToString(parameters['highFrequency'][channelIndex], 'highFrequency', conversionRules)}
          list={highFreqOptions.map(item => item.toString())}
          parameter='highFrequency'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='HIGH FREQ'
          unit='Hz'
        />
      </div>

      <ChannelPillButtonSwitch
        options={attenuationSelectionOptions.map(item => item.toString())}
        value={convertToString(parameters['attenuationSelection'][channelIndex], 'attenuationSelection', conversionRules)}
        channel={channelIndex}
        parameter='attenuationSelection'
        parameterHandler={channelParameterHandler}
        label={'ATTEN SEL'}
      />

    </div>
  )
}

export default Channel