import React, { useEffect } from 'react'
import { InstanceReportsType } from '@/types'
import { Container, ContinuousKnob, SteppedKnob, ListKnob, ChannelToggleButton, HorizontalMeter } from '@moonlight-neon-ui'
import { InstanceParameters, ParameterId } from '@audiocloud/api'
import { convertToNumber, convertToString, convertToBoolean, ConversionRules } from '../valueConverters'

type Props = {
  channelIds: string[],
  parameters: InstanceParameters,
  wet: number,
  reports: InstanceReportsType,
  webSocketDefaultParametersSetter: (interfaceOnlyParams: ParameterId[], conversionRules: ConversionRules) => void,
  channelParameterHandler: (value: string | number | boolean, channel: number, parameter: string, conversionRules: ConversionRules) => void,
  instanceParameterHandler: (value: string | number | boolean, parameter: string, conversionRules: ConversionRules) => void,
  interfaceParameterHandler: (value: string | number | boolean, parameter: string) => void
}
const Btrmkr_ML_Faceplate: React.FC<Props> = ({ channelIds, parameters, wet, reports, webSocketDefaultParametersSetter, channelParameterHandler, instanceParameterHandler, interfaceParameterHandler }) => {

  const interfaceOnlyParams: string[] = ['measureOutput']

  const conversionRules: ConversionRules = {
    'stringToString': {},
    'stringToNumber': {},
    'stringToBoolean': {},
    'numberToString': {},
    'numberToNumber': {
      'evenColorAmount': {
        '0': 1,
        '1': 0
      },
      'evenColorDrive': {
        '0': 1,
        '1': 0
      },
      'oddColorAmount': {
        '0': 1,
        '1': 0
      },
      'oddColorDrive': {
        '0': 1,
        '1': 0
      }
    },
    'numberToBoolean': {},
    'booleanToString': {},
    'booleanToNumber': {},
    'booleanToBoolean': {}
  }

  const color_freq_scale = [35, 45, 56, 67, 83, 100, 130, 160, 200, 300, 440, 560, 700, 900, 1050, 1250, 1500, 2000, 3000, 4000, 5000, 6000, 7000, 8500, 10000, 11500, 13500, 15000, 17000, 19000, 21000, 23000]

  const channelParameterHandlerWrapper = (value: string | number | boolean, channel: number, parameter: string) => channelParameterHandler(value, channel, parameter, conversionRules)
  const instanceParameterHandlerWrapper = (value: string | number | boolean, parameter: string) => instanceParameterHandler(value, parameter, conversionRules)

  useEffect(() => {
    webSocketDefaultParametersSetter(interfaceOnlyParams, conversionRules)
  }, [])

  return (
    <Container className='w-full p-4 relative rounded-lg'>

      <div className='text-slate-200 pb-4 w-full text-center'>Bettermaker Mastering Limiter</div>

      <div className='flex flex-col justify-between items-center gap-5'>

        <div className='flex flex-wrap justify-center items-center gap-5'>

          <HorizontalMeter
            scale={['-30', '-28', '-26', '-24', '-22', '-20', '-18', '-16', '-14', '-12', '-10', '-8', '-6', '-4', '-2', '0']}
            value_stereo={reports['peakLevel'] as [number, number]}
            label='Peak'
          />

          <HorizontalMeter
            scale={['-36', '-33', '-30', '-27', '-24', '-21', '-18', '-15', '-12', '-9', '-6', '-3', '0', '3']}
            value_stereo={reports['rmsLevel'] as [number, number]}
            label='RMS'
          />

          <HorizontalMeter
            scale={['8', '7', '6', '5', '4', '3', '2', '1', '0']}
            value_stereo={reports['gainReduction'] as [number, number]}
            label='GR'
          />
          
          <HorizontalMeter
            scale={['0', '1', '2', '3', '4', '5', '6']}
            value_stereo={reports['clip'] as [number, number]}
            label='Clip'
          />

        </div>

        <div className='flex flex-wrap justify-center 3xl:justify-between items-center gap-10'>

          {/* LEFT SECTION */}
          <div className='flex items-center gap-5'>

            <div className='flex flex-col gap-2'>
              
              <ChannelToggleButton
                channel={0}
                parameter='processingEnabled'
                parameterHandler={channelParameterHandlerWrapper}
                toggled={convertToBoolean(parameters['processingEnabled'][0], 'processingEnabled', conversionRules)}
                className='w-24'
              >Engage</ChannelToggleButton>

              <ChannelToggleButton
                channel={0}
                parameter='clippiterEnabled'
                parameterHandler={channelParameterHandlerWrapper}
                toggled={convertToBoolean(parameters['clippiterEnabled'][0], 'clippiterEnabled', conversionRules)}
                className='w-24'
              >Clipper</ChannelToggleButton>

              <ChannelToggleButton
                channel={0}
                parameter='midSideEnabled'
                parameterHandler={channelParameterHandlerWrapper}
                toggled={convertToBoolean(parameters['midSideEnabled'][0], 'midSideEnabled', conversionRules)}
                className='w-24'
              >M/S</ChannelToggleButton>

            </div>

            <ContinuousKnob
              channel={0}
              value={convertToNumber(parameters['inputLevel'][0], 'inputLevel', conversionRules)}
              range={[0, 20]}
              parameter='inputLevel'
              parameterHandler={channelParameterHandlerWrapper}
              size='lg'
              label='INPUT'
              unit='dB'
            />

          </div>

          {/* MIDDLE SECTION */}
          <div className='flex flex-col justify-center items-center gap-5'>

            {/* CLIPPITER, M/S, ATK, REL */}
            <div className='flex flex-wrap justify-center items-center gap-5'>

              <ContinuousKnob
                channel={0}
                value={convertToNumber(parameters['clippiterAmount'][0], 'clippiterAmount', conversionRules)}
                range={[0, 100]}
                parameter='clippiterAmount'
                parameterHandler={channelParameterHandlerWrapper}
                size='lg'
                label='CLIPPITER'
                unit='%'
              />
              
              <ContinuousKnob
                channel={0}
                value={convertToNumber(parameters['midSideWidth'][0], 'midSideWidth', conversionRules)}
                range={[-8, 8]}
                parameter='midSideWidth'
                parameterHandler={channelParameterHandlerWrapper}
                size='lg'
                label='M/S Width'
                unit='dB'
              />

              <ContinuousKnob
                channel={0}
                value={convertToNumber(parameters['attack'][0], 'attack', conversionRules)}
                range={[0, 2500]}
                parameter='attack'
                parameterHandler={channelParameterHandlerWrapper}
                size='lg'
                label='ATTACK'
                unit='ms'
              />

              <ContinuousKnob
                channel={0}
                value={convertToNumber(parameters['release'][0], 'release', conversionRules)}
                range={[10, 1300]}
                parameter='release'
                parameterHandler={channelParameterHandlerWrapper}
                size='lg'
                label='RELEASE'
                unit='ms'
              />

            </div>

            {/* COLORS */}
            <div className='flex flex-wrap justify-center items-center gap-5'>

              <SteppedKnob
                channel={0}
                value={convertToNumber(parameters['oddColorDrive'][0], 'oddColorDrive', conversionRules)}
                range={[0, 100]}
                step={5}
                parameter='oddColorDrive'
                parameterHandler={channelParameterHandlerWrapper}
                size='lg'
                label='DRIVE (ODD)'
                unit='%'
              />

              <SteppedKnob
                channel={0}
                value={convertToNumber(parameters['evenColorDrive'][0], 'evenColorDrive', conversionRules)}
                range={[0, 100]}
                step={5}
                parameter='evenColorDrive'
                parameterHandler={channelParameterHandlerWrapper}
                size='lg'
                label='DRIVE (EVEN)'
                unit='%'
              />

              <ListKnob
                channel={0}
                value={convertToString(parameters['oddColorFrequency'][0], 'oddColorFrequency', conversionRules)}
                list={color_freq_scale.map(item => item.toString())}
                parameter='oddColorFrequency'
                parameterHandler={channelParameterHandlerWrapper}
                size='lg'
                label='FREQ (ODD)'
                unit='Hz'
              />

              <ListKnob
                channel={0}
                value={convertToString(parameters['evenColorFrequency'][0], 'evenColorFrequency', conversionRules)}
                list={color_freq_scale.map(item => item.toString())}
                parameter='evenColorFrequency'
                parameterHandler={channelParameterHandlerWrapper}
                size='lg'
                label='FREQ (EVEN)'
                unit='Hz'
              />
              
              <SteppedKnob
                channel={0}
                value={convertToNumber(parameters['oddColorAmount'][0], 'oddColorAmount', conversionRules)}
                range={[0, 100]}
                step={5}
                parameter='oddColorAmount'
                parameterHandler={channelParameterHandlerWrapper}
                size='lg'
                label='AMOUNT (ODD)'
                unit='%'
              />

              <SteppedKnob
                channel={0}
                value={convertToNumber(parameters['evenColorAmount'][0], 'evenColorAmount', conversionRules)}
                range={[0, 100]}
                step={5}
                parameter='evenColorAmount'
                parameterHandler={channelParameterHandlerWrapper}
                size='lg'
                label='AMOUNT (EVEN)'
                unit='%'
              />

            </div>
          </div>

          {/* RIGHT SECTION */}
          <div className='flex items-center gap-5'>

            <ContinuousKnob
              channel={0}
              value={convertToNumber(parameters['outputLevel'][0], 'outputLevel', conversionRules)}
              range={[-15, 5]}
              parameter='outputLevel'
              parameterHandler={channelParameterHandlerWrapper}
              size='lg'
              label='OUTPUT'
              unit='dB'
            />

            <div className='flex flex-col gap-2'>

              <ChannelToggleButton
                channel={0}
                parameter='intelligentReleaseEnabled'
                parameterHandler={channelParameterHandlerWrapper}
                toggled={convertToBoolean(parameters['intelligentReleaseEnabled'][0], 'intelligentReleaseEnabled', conversionRules)}
                className='w-24'
              >IREL</ChannelToggleButton>

              <ChannelToggleButton
                channel={0}
                parameter='oddColorEnabled'
                parameterHandler={channelParameterHandlerWrapper}
                toggled={convertToBoolean(parameters['oddColorEnabled'][0], 'oddColorEnabled', conversionRules)}
                className='w-24'
              >ODD</ChannelToggleButton>

              <ChannelToggleButton
                channel={0}
                parameter='evenColorEnabled'
                parameterHandler={channelParameterHandlerWrapper}
                toggled={convertToBoolean(parameters['evenColorEnabled'][0], 'evenColorEnabled', conversionRules)}
                className='w-24'
              >EVEN</ChannelToggleButton>

            </div>
          </div>

        </div>

      </div>

    </Container>
  )
}

export default Btrmkr_ML_Faceplate