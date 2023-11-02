import React from 'react'
import { InstanceParameters } from '@audiocloud/api'
import { ChannelToggleButton, Header2, ListKnob } from '@moonlight-neon-ui'
import { convertToString, ConversionRules, convertToBoolean } from '../valueConverters'
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

      <div className='flex flex-col justify-center items-center gap-5'>

        { !link && <Header2>CH {channelIndex + 1}</Header2> }

        <ChannelToggleButton
          channel={channelIndex}
          parameter='enabled'
          parameterHandler={channelParameterHandler}
          toggled={convertToBoolean(parameters['enabled'][channelIndex], 'enabled', conversionRules)}
          className='w-32'
        >Engage</ChannelToggleButton>

        <ChannelToggleButton
          channel={channelIndex}
          parameter='warmMode'
          parameterHandler={channelParameterHandler}
          toggled={convertToBoolean(parameters['warmMode'][channelIndex], 'warmMode', conversionRules)}
          className='w-32'
        >Warm</ChannelToggleButton>

      </div>

      {/* LOW */}
      <div className='flex flex-col justify-center items-center gap-2'>
        <ListKnob
          channel={channelIndex}
          value={convertToString(parameters['lowGain'][channelIndex], 'lowGain', conversionRules)}
          list={[-1, 0, 1, 2.5, 4, 5, 6, 7, 8, 9, 11, 13, 15].map(item => item.toString())}
          parameter='lowGain'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='Low'
          unit='dB'
        />
        
        <ChannelToggleButton
          channel={channelIndex}
          parameter='lowCutCurve'
          parameterHandler={channelParameterHandler}
          toggled={convertToBoolean(parameters['lowCutCurve'][channelIndex], 'lowCutCurve', conversionRules)}
          className='w-32'
        >Cut</ChannelToggleButton>

        <ChannelToggleButton
          channel={channelIndex}
          parameter='lowCutGain'
          parameterHandler={channelParameterHandler}
          toggled={convertToBoolean(parameters['lowCutGain'][channelIndex], 'lowCutGain', conversionRules)}
          className='w-32'
        >Negative Gain</ChannelToggleButton>

        <ListKnob
          value={convertToString(parameters['lowFreq'][channelIndex], 'lowFreq', conversionRules)}
          list={[8, 9, 10, 12, 18, 30, 55, 65, 75, 95, 120, 160, 200].map(item => item.toString())}
          parameter='lowFreq'
          parameterHandler={channelParameterHandler}
          size='lg'
          label=''
          unit='Hz'
        />
      </div>

      {/* BOTTOM */}
      <div className='flex flex-col justify-center items-center gap-2'>
        <ListKnob
          channel={channelIndex}
          value={convertToString(parameters['bottomGain'][channelIndex], 'bottomGain', conversionRules)}
          list={[-1, 0, 1, 2.5, 4, 5, 6, 7, 8, 9, 11, 13, 15].map(item => item.toString())}
          parameter='bottomGain'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='Bottom'
          unit='dB'
        />
        
        <ChannelToggleButton
          channel={channelIndex}
          parameter='bottomHighQ'
          parameterHandler={channelParameterHandler}
          toggled={convertToBoolean(parameters['bottomHighQ'][channelIndex], 'bottomHighQ', conversionRules)}
          className='w-32'
        >Cut</ChannelToggleButton>

        <ChannelToggleButton
          channel={channelIndex}
          parameter='bottomCutGain'
          parameterHandler={channelParameterHandler}
          toggled={convertToBoolean(parameters['bottomCutGain'][channelIndex], 'bottomCutGain', conversionRules)}
          className='w-32'
        >Negative Gain</ChannelToggleButton>

        <ListKnob
          value={convertToString(parameters['bottomFreq'][channelIndex], 'bottomFreq', conversionRules)}
          list={[8, 9, 10, 12, 18, 30, 55, 65, 75, 95, 120, 160, 200].map(item => item.toString())}
          parameter='bottomFreq'
          parameterHandler={channelParameterHandler}
          size='lg'
          label=''
          unit='Hz'
        />
      </div>

      {/* MIDDLE */}
      <div className='flex flex-col justify-center items-center gap-2'>
        <ListKnob
          channel={channelIndex}
          value={convertToString(parameters['middleGain'][channelIndex], 'middleGain', conversionRules)}
          list={[-1, 0, 1, 2.5, 4, 5, 6, 7, 8, 9, 11, 13, 15].map(item => item.toString())}
          parameter='middleGain'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='Middle'
          unit='dB'
        />
        
        <ChannelToggleButton
          channel={channelIndex}
          parameter='middleHighQ'
          parameterHandler={channelParameterHandler}
          toggled={convertToBoolean(parameters['middleHighQ'][channelIndex], 'middleHighQ', conversionRules)}
          className='w-32'
        >Cut</ChannelToggleButton>

        <ChannelToggleButton
          channel={channelIndex}
          parameter='middleCutGain'
          parameterHandler={channelParameterHandler}
          toggled={convertToBoolean(parameters['middleCutGain'][channelIndex], 'middleCutGain', conversionRules)}
          className='w-32'
        >Negative Gain</ChannelToggleButton>

        <ListKnob
          channel={channelIndex}
          value={convertToString(parameters['middleFreq'][channelIndex], 'middleFreq', conversionRules)}
          list={[8, 9, 10, 12, 18, 30, 55, 65, 75, 95, 120, 160, 200].map(item => item.toString())}
          parameter='middleFreq'
          parameterHandler={channelParameterHandler}
          size='lg'
          label=''
          unit='Hz'
        />
      </div>

      {/* TOP */}
      <div className='flex flex-col justify-center items-center gap-2'>
        <ListKnob
          channel={channelIndex}
          value={convertToString(parameters['topGain'][channelIndex], 'topGain', conversionRules)}
          list={[-1, 0, 1, 2.5, 4, 5, 6, 7, 8, 9, 11, 13, 15].map(item => item.toString())}
          parameter='topGain'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='Top'
          unit='dB'
        />
        
        <ChannelToggleButton
          channel={channelIndex}
          parameter='topHighQ'
          parameterHandler={channelParameterHandler}
          toggled={convertToBoolean(parameters['topHighQ'][channelIndex], 'topHighQ', conversionRules)}
          className='w-32'
        >Cut</ChannelToggleButton>

        <ChannelToggleButton
          channel={channelIndex}
          parameter='topCutGain'
          parameterHandler={channelParameterHandler}
          toggled={convertToBoolean(parameters['topCutGain'][channelIndex], 'topCutGain', conversionRules)}
          className='w-32'
        >Negative Gain</ChannelToggleButton>

        <ListKnob
          channel={channelIndex}
          value={convertToString(parameters['topFreq'][channelIndex], 'topFreq', conversionRules)}
          list={[8, 9, 10, 12, 18, 30, 55, 65, 75, 95, 120, 160, 200].map(item => item.toString())}
          parameter='topFreq'
          parameterHandler={channelParameterHandler}
          size='lg'
          label=''
          unit='Hz'
        />
      </div>

      {/* HIGH */}
      <div className='flex flex-col justify-center items-center gap-2'>
        <ListKnob
          channel={channelIndex}
          value={convertToString(parameters['highGain'][channelIndex], 'highGain', conversionRules)}
          list={[-1, 0, 1, 2.5, 4, 5, 6, 7, 8, 9, 11, 13, 15].map(item => item.toString())}
          parameter='highGain'
          parameterHandler={channelParameterHandler}
          size='lg'
          label='High'
          unit='dB'
        />
        
        <ChannelToggleButton
          channel={channelIndex}
          parameter='highCutCurve'
          parameterHandler={channelParameterHandler}
          toggled={convertToBoolean(parameters['highCutCurve'][channelIndex], 'highCutCurve', conversionRules)}
          className='w-32'
        >Cut</ChannelToggleButton>

        <ChannelToggleButton
          channel={channelIndex}
          parameter='highCutGain'
          parameterHandler={channelParameterHandler}
          toggled={convertToBoolean(parameters['highCutGain'][channelIndex], 'highCutGain', conversionRules)}
          className='w-32'
        >Negative Gain</ChannelToggleButton>

        <ListKnob
          channel={channelIndex}
          value={convertToString(parameters['highFreq'][channelIndex], 'highFreq', conversionRules)}
          list={[8, 9, 10, 12, 18, 30, 55, 65, 75, 95, 120, 160, 200].map(item => item.toString())}
          parameter='highFreq'
          parameterHandler={channelParameterHandler}
          size='lg'
          label=''
          unit='Hz'
        />
      </div>

    </div>
  )
}

export default Channel