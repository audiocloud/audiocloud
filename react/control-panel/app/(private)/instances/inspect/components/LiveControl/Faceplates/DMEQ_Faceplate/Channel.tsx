import React from 'react'
import { InstanceParameters } from '@audiocloud/api'
import { ChannelPillButtonSwitch, ContinuousKnob, Header2 } from '@moonlight-neon-ui'
import { convertToNumber, convertToString, ConversionRules } from '../valueConverters'
import { InstanceReportsType } from '@/types'

type Props = {
  channelIndex: number
  link: boolean,
  parameters: InstanceParameters,
  channelParameterHandler: (value: string | number | boolean, channel: number, parameter: string) => void
  conversionRules: ConversionRules,
  reports: InstanceReportsType
}

const Channel: React.FC<Props> = ({ channelIndex, link, parameters, channelParameterHandler, conversionRules, reports }) => {

  const doubleGainOptions = [5, 10]
  const enabledOptions = ['IN', 'OUT']

  return (
    <div className='flex flex-wrap justify-center items-center gap-10'>

      { !link && <Header2>CH {channelIndex + 1}</Header2> }

      {/* LOW */}
      <div className='flex flex-col justify-center items-center gap-5'>

        <Header2>LF</Header2>

        <ContinuousKnob
          channel={channelIndex}
          value={convertToNumber(parameters['lowQ'][channelIndex], 'lowQ', conversionRules)}
          range={[-1, 4]}
          parameter='lowQ'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='SHAPE'
        />

        <ContinuousKnob
          channel={channelIndex}
          value={convertToNumber(parameters['lowFrequency'][channelIndex], 'lowFrequency', conversionRules)}
          range={[22, 800]}
          parameter='lowFrequency'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='FREQ'
          unit='dB'
        />

        <div className='flex flex-col gap-1'>

          <ChannelPillButtonSwitch
            options={doubleGainOptions.map(item => item.toString())}
            value={convertToString(parameters['lowDoubleGain'][channelIndex], 'lowDoubleGain', conversionRules)}
            channel={channelIndex}
            parameter='lowDoubleGain'
            parameterHandler={channelParameterHandler}
          />

          <ChannelPillButtonSwitch
            options={enabledOptions}
            value={convertToString(parameters['lowEnabled'][channelIndex], 'lowEnabled', conversionRules)}
            channel={channelIndex}
            parameter='lowEnabled'
            parameterHandler={channelParameterHandler}
          />

        </div>

        <ContinuousKnob
          channel={channelIndex}
          value={convertToNumber(parameters['lowGain'][channelIndex], 'lowGain', conversionRules)}
          range={[-5, 5]}
          parameter='lowGain'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='GAIN'
          unit='dB'
        />

      </div>

      {/* LOW MID */}
      <div className='flex flex-col justify-center items-center gap-5'>

        <Header2>LMF</Header2>

        <ContinuousKnob
          channel={channelIndex}
          value={convertToNumber(parameters['lowMidQ'][channelIndex], 'lowMidQ', conversionRules)}
          range={[-1, 4]}
          parameter='lowMidQ'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='SHAPE'
        />

        <ContinuousKnob
          channel={channelIndex}
          value={convertToNumber(parameters['lowMidFrequency'][channelIndex], 'lowMidFrequency', conversionRules)}
          range={[130, 4700]}
          parameter='lowMidFrequency'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='FREQ'
          unit='dB'
        />

        <div className='flex flex-col gap-1'>
          <ChannelPillButtonSwitch
            options={doubleGainOptions.map(item => item.toString())}
            value={convertToString(parameters['lowMidDoubleGain'][channelIndex], 'lowMidDoubleGain', conversionRules)}
            channel={channelIndex}
            parameter='lowMidDoubleGain'
            parameterHandler={channelParameterHandler}
          />

          <ChannelPillButtonSwitch
            options={enabledOptions}
            value={convertToString(parameters['lowMidEnabled'][channelIndex], 'lowMidEnabled', conversionRules)}
            channel={channelIndex}
            parameter='lowMidEnabled'
            parameterHandler={channelParameterHandler}
          />
        </div>

        <ContinuousKnob
          channel={channelIndex}
          value={convertToNumber(parameters['lowMidGain'][channelIndex], 'lowMidGain', conversionRules)}
          range={[-5, 5]}
          parameter='lowMidGain'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='GAIN'
          unit='dB'
        />

      </div>

      {/* MID */}
      <div className='flex flex-col justify-center items-center gap-5'>

        <Header2>MF</Header2>

        <ContinuousKnob
          channel={channelIndex}
          value={convertToNumber(parameters['midQ'][channelIndex], 'midQ', conversionRules)}
          range={[-1, 4]}
          parameter='midQ'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='SHAPE'
        />

        <ContinuousKnob
          channel={channelIndex}
          value={convertToNumber(parameters['midFrequency'][channelIndex], 'midFrequency', conversionRules)}
          range={[220, 8000]}
          parameter='midFrequency'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='FREQ'
          unit='dB'
        />

        <div className='flex flex-col gap-1'>
          <ChannelPillButtonSwitch
            options={doubleGainOptions.map(item => item.toString())}
            value={convertToString(parameters['midDoubleGain'][channelIndex], 'midDoubleGain', conversionRules)}
            channel={channelIndex}
            parameter='midDoubleGain'
            parameterHandler={channelParameterHandler}
          />

          <ChannelPillButtonSwitch
            options={enabledOptions}
            value={convertToString(parameters['midEnabled'][channelIndex], 'midEnabled', conversionRules)}
            channel={channelIndex}
            parameter='midEnabled'
            parameterHandler={channelParameterHandler}
          />
        </div>

        <ContinuousKnob
          channel={channelIndex}
          value={convertToNumber(parameters['midGain'][channelIndex], 'midGain', conversionRules)}
          range={[-5, 5]}
          parameter='midGain'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='GAIN'
          unit='dB'
        />

      </div>

      {/* HIGH MID */}
      <div className='flex flex-col justify-center items-center gap-5'>

        <Header2>HMF</Header2>

        <ContinuousKnob
          channel={channelIndex}
          value={convertToNumber(parameters['highMidQ'][channelIndex], 'highMidQ', conversionRules)}
          range={[-1, 4]}
          parameter='highMidQ'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='SHAPE'
        />

        <ContinuousKnob
          channel={channelIndex}
          value={convertToNumber(parameters['highMidFrequency'][channelIndex], 'highMidFrequency', conversionRules)}
          range={[480, 18000]}
          parameter='highMidFrequency'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='FREQ'
          unit='dB'
        />

        <div className='flex flex-col gap-1'>
          <ChannelPillButtonSwitch
            options={doubleGainOptions.map(item => item.toString())}
            value={convertToString(parameters['highMidDoubleGain'][channelIndex], 'highMidDoubleGain', conversionRules)}
            channel={channelIndex}
            parameter='highMidDoubleGain'
            parameterHandler={channelParameterHandler}
          />

          <ChannelPillButtonSwitch
            options={enabledOptions}
            value={convertToString(parameters['highMidEnabled'][channelIndex], 'highMidEnabled', conversionRules)}
            channel={channelIndex}
            parameter='highMidEnabled'
            parameterHandler={channelParameterHandler}
          />
        </div>

        <ContinuousKnob
          channel={channelIndex}
          value={convertToNumber(parameters['highMidGain'][channelIndex], 'highMidGain', conversionRules)}
          range={[-5, 5]}
          parameter='highMidGain'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='GAIN'
          unit='dB'
        />

      </div>

      {/* HIGH */}
      <div className='flex flex-col justify-between items-center gap-5 self-end'>

        <Header2>HF</Header2>

        <div className='flex flex-col justify-center items-center gap-5'>

          <ChannelPillButtonSwitch
            options={['6', '12']}
            value={convertToString(parameters['highSlope'][channelIndex], 'highSlope', conversionRules)}
            channel={channelIndex}
            parameter='highSlope'
            parameterHandler={channelParameterHandler}
            label='SLOPE'
          />

          <ContinuousKnob
            channel={channelIndex}
            value={convertToNumber(parameters['highFrequency'][channelIndex], 'highFrequency', conversionRules)}
            range={[900, 20000]}
            parameter='highFrequency'
            parameterHandler={channelParameterHandler}
            size='lg'
            label='FREQ'
            unit='dB'
          />

          <div className='flex flex-col gap-1'>
            <ChannelPillButtonSwitch
              options={doubleGainOptions.map(item => item.toString())}
              value={convertToString(parameters['highDoubleGain'][channelIndex], 'highDoubleGain', conversionRules)}
              channel={channelIndex}
              parameter='highDoubleGain'
              parameterHandler={channelParameterHandler}
            />

            <ChannelPillButtonSwitch
              options={enabledOptions}
              value={convertToString(parameters['highEnabled'][channelIndex], 'highEnabled', conversionRules)}
              channel={channelIndex}
              parameter='highEnabled'
              parameterHandler={channelParameterHandler}
            />
          </div>

          <ContinuousKnob
            channel={channelIndex}
            value={convertToNumber(parameters['highGain'][channelIndex], 'highGain', conversionRules)}
            range={[-5, 5]}
            parameter='highGain'
            parameterHandler={channelParameterHandler}
            size='lg'
            label='GAIN'
            unit='dB'
          />

        </div>

      </div>

    </div>
  )
}

export default Channel