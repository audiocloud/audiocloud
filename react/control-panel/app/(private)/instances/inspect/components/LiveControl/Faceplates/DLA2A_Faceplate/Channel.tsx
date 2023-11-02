import React from 'react'
import { InstanceParameters } from '@audiocloud/api'
import { ChannelPillButtonSwitch, ContinuousKnob, Header2, HorizontalMeter } from '@moonlight-neon-ui'
import { convertToNumber, convertToString, ConversionRules } from '../valueConverters'
import { InstanceReports } from '@/types'

type Props = {
  channelIndex: number
  link: boolean,
  parameters: InstanceParameters,
  channelParameterHandler: (value: string | number | boolean, channel: number, parameter: string) => void
  conversionRules: ConversionRules,
  reports: InstanceReports
}

const Channel: React.FC<Props> = ({ channelIndex, link, parameters, channelParameterHandler, conversionRules, reports }) => {

  return (
    <div className='flex flex-wrap justify-center items-center gap-10'>

      { !link && <Header2>CH {channelIndex + 1}</Header2> }

      <div className='flex justify-center items-center gap-5'>

        <ContinuousKnob
          channel={channelIndex}
          value={convertToNumber(parameters['feedback'][channelIndex], 'feedback', conversionRules)}
          range={[0, 100]}
          parameter='feedback'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='FEEDBACK'
        />
        
        <ContinuousKnob
          channel={channelIndex}
          value={convertToNumber(parameters['gain'][channelIndex], 'gain', conversionRules)}
          range={[0, 100]}
          parameter='gain'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='GAIN'
          unit='dB'
        />

      </div>

      <div className='flex flex-col justify-center items-center gap-5'>

        { parameters['metering'][0] === 'GR' && (
          <HorizontalMeter
            scale={['-20', '-10', '-7', '-5', '-3', '-2', '-1', '0', '1', '2', '3']}
            value_stereo={reports['gainReduction'] as [number, number]}
            label='Gain Reduction'
          />
        )}

        { parameters['metering'][0] === 'VU +4' && (
          <HorizontalMeter
            scale={['-20', '-10', '-7', '-5', '-3', '-2', '-1', '0', '1', '2', '3']}
            value_stereo={reports['vuLevel'] as [number, number]}
            label='VU +4'
          />
        )}
                
      </div>

      <div className='flex justify-center items-center gap-5'>

        <ContinuousKnob
          channel={channelIndex}
          value={convertToNumber(parameters['peakReduction'][channelIndex], 'peakReduction', conversionRules)}
          range={[0, 100]}
          parameter='peakReduction'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='PEAK REDUCTION'
        />
        
        <ContinuousKnob
          channel={channelIndex}
          value={convertToNumber(parameters['emphasis'][channelIndex], 'emphasis', conversionRules)}
          range={[0, 100]}
          parameter='emphasis'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='EMPHASIS'
        />

      </div>

      <div className='flex flex-col justify-center items-center gap-5'>

        <ChannelPillButtonSwitch
          options={['FAST', 'SLOW']}
          value={convertToString(parameters['speed'][channelIndex], 'speed', conversionRules)}
          channel={channelIndex}
          parameter='speed'
          parameterHandler={channelParameterHandler}
        />

        <ChannelPillButtonSwitch
          options={['LIMIT', 'COMPRESS']}
          value={convertToString(parameters['mode'][channelIndex], 'mode', conversionRules)}
          channel={channelIndex}
          parameter='mode'
          parameterHandler={channelParameterHandler}
        />

    </div>

    </div>
  )
}

export default Channel