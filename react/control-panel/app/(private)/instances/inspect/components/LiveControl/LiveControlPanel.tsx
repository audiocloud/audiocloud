import React, { useState } from 'react'
import debounce from 'lodash.debounce'
import { defaultStates } from './Faceplates/data/defaultStates'
import { InstancePlayState, SetInstanceParameter } from '@/services/domainClient/types'
import { InstanceReportsType } from '@/types'
import { InstanceParameters, InstanceReportsType, ParameterId } from '@audiocloud/api'
import { ConversionRules, convertToNumber } from './Faceplates/valueConverters'
import clonedeep from 'lodash.clonedeep'
import Btrmkr_ML_Faceplate from './Faceplates/Btrmkr_ML_Faceplate/Btrmkr_ML_Faceplate'
import D1176_ADG_Faceplate from './Faceplates/D1176_Faceplate/D1176_Faceplate'
import DADU_Faceplate from './Faceplates/DADU_Faceplate/DADU_Faceplate'
import DBWL_Faceplate from './Faceplates/DBWL_Faceplate/DBWL_Faceplate'
import DEQP1_Faceplate from './Faceplates/DEQP1_Faceplate/DEQP1_Faceplate'
import DFC670_Faceplate from './Faceplates/DFC670_Faceplate/DFC670_Faceplate'
import DLA2A_Faceplate from './Faceplates/DLA2A_Faceplate/DLA2A_Faceplate'
import DMEQ_Faceplate from './Faceplates/DMEQ_Faceplate/DMEQ_Faceplate'
import DVCA_Faceplate from './Faceplates/DVCA_Faceplate/DVCA_Faceplate'
import Elysia_karacter_Faceplate from './Faceplates/Elysia_karacter_Faceplate/Elysia_karacter_Faceplate'
import Elysia_museq_Faceplate from './Faceplates/Elysia_museq_Faceplate/Elysia_museq_Faceplate'
import Elysia_nvelope_Faceplate from './Faceplates/Elysia_nvelope_Faceplate/Elysia_nvelope_Faceplate'
import Elysia_xfilter_Faceplate from './Faceplates/Elysia_xfilter_Faceplate/Elysia_xfilter_Faceplate'
import Elysia_xpressor_Faceplate from './Faceplates/Elysia_xpressor_Faceplate/Elysia_xpressor_Faceplate'
import Gyraf_G24_Faceplate from './Faceplates/Gyraf_G24_Faceplate/Gyraf_G24_Faceplate'
import Neve_1084_Faceplate from './Faceplates/Neve_1084_Faceplate/Neve_1084_Faceplate'
import Tierra_Gravity_Faceplate from './Faceplates/Tierra_Gravity_Faceplate/Tierra_Gravity_Faceplate'

type Props = {
  instance_id: string,
  liveControlEnabled: boolean,
  playState: InstancePlayState | 'unknown',
  reports: InstanceReportsType
}

const LiveControlPanel: React.FC<Props> = ({ instance_id, liveControlEnabled, playState, reports }) => {

  if (!instance_id) return <div>No instance found.</div>

  const [channelIds, setChannelIds] = useState<string[]>(clonedeep(defaultStates[instance_id].channel_ids))
  const [parameters, setParameters] = useState<InstanceParameters>(clonedeep(defaultStates[instance_id].parameters))
  const [wet, setWet] = useState(defaultStates[instance_id].wet)

  const facePlateDataReady = () => {
    if (channelIds.length > 0 && Object.keys(parameters).length > 0) return true
    return false
  }

  const instancesWithoutExtractedChannelComponents = ['elysia_nvelope_1']

  // CHANNEL HANDLER

  const channelParameterHandler = debounce((value: string | number | boolean, channel: number, parameter: string, conversionRules: ConversionRules) => {

    const updatedParameters: Record<string, (string | number | boolean | null)[]> = {...parameters}
    
    updatedParameters[parameter][channel] = convertToNumber(value, parameter, conversionRules)
    const changes = [{ value: convertToNumber(value, parameter, conversionRules), channel, parameter }]

    if (parameters['link'] && parameters['link'][0] && !instancesWithoutExtractedChannelComponents.find(item => item === instance_id)) {
      updatedParameters[parameter][channel + 1] = convertToNumber(value, parameter, conversionRules)
      changes.push({ value: convertToNumber(value, parameter, conversionRules), channel: channel + 1, parameter })
    }
    
    setParameters({...updatedParameters})
    // webSocketParametersHandler(changes)
  }, 100)

  // INSTANCE WIDE HANDLER

  const instanceParameterHandler = debounce((value: string | number | boolean, parameter: string, conversionRules: ConversionRules) => {

    const updatedParameters: Record<string, (string | number | boolean | null)[]> = {...parameters}
    updatedParameters[parameter][0] = convertToNumber(value, parameter, conversionRules)
    setParameters({...updatedParameters})

    // webSocketParametersHandler([{ value: convertToNumber(value, parameter, conversionRules), channel: 0, parameter }])
  }, 100)

  // UI PARAM HANDLER

  const interfaceParameterHandler = (value: string | number | boolean, parameter: string) => {

    const updatedParameters: Record<string, (string | number | boolean | null)[]> = {...parameters}
    updatedParameters[parameter][0] = value
    setParameters({...updatedParameters})
  }

  // WEBSOCKET DEFAULT PARAMS SETTER

  const webSocketDefaultParametersSetter = (interfaceOnlyParams: ParameterId[], conversionRules: ConversionRules) => {
 
    const changes: { value: number, channel: number, parameter: ParameterId }[] = []
    const paramsToChange = Object.keys(parameters).filter(item => item == interfaceOnlyParams.find(i => i === item))

    paramsToChange.forEach((parameter) => {
      parameters[parameter].forEach((channelValue, index) => {
        changes.push({
          value: convertToNumber(channelValue, parameter, conversionRules),
          channel: index,
          parameter: parameter
        })
      })
    })

    // webSocketParametersHandler([...changes])
  }

  // PURE WEBSOCKET HANDLER

  // const webSocketParametersHandler = async (changes: { value: number, channel: number, parameter: string }[]) => {
  //   if (connectionStatus) {
  //     console.log('WS parameter handler:', changes)
  //     setInstanceParameters(instance_id, changes)
  //   } else {
  //     console.log('WS parameter handler: connection', connectionStatus)
  //   }
  // }

  if (!facePlateDataReady()) return null

  return (
    <div className='m-4 border'>
      
      {/* TITLE */}
      {/* <div className='p-4 text-slate-600 text-sm text-center'>Live Control Panel (Status: {connectionStatus ? 'connected' : 'disconnected'})</div> */}

      {/* FACEPLATE */}
      <div className='px-2 pb-4'>
        {/* { instance_id === 'btrmkr_ml_1' &&          <Btrmkr_ML_Faceplate        parameters={parameters} channelIds={channelIds} wet={wet} reports={reports} webSocketDefaultParametersSetter={webSocketDefaultParametersSetter} channelParameterHandler={channelParameterHandler} instanceParameterHandler={instanceParameterHandler} interfaceParameterHandler={interfaceParameterHandler} /> }
        { instance_id === 'distopik_1176_1' &&      <D1176_ADG_Faceplate        parameters={parameters} channelIds={channelIds} wet={wet} reports={reports} webSocketDefaultParametersSetter={webSocketDefaultParametersSetter} channelParameterHandler={channelParameterHandler} instanceParameterHandler={instanceParameterHandler} interfaceParameterHandler={interfaceParameterHandler} /> }
        { instance_id === 'distopik_adu_1' &&       <DADU_Faceplate             parameters={parameters} channelIds={channelIds} wet={wet} reports={reports} webSocketDefaultParametersSetter={webSocketDefaultParametersSetter} channelParameterHandler={channelParameterHandler} instanceParameterHandler={instanceParameterHandler} interfaceParameterHandler={interfaceParameterHandler} /> }
        { instance_id === 'distopik_bwl_1' &&       <DBWL_Faceplate             parameters={parameters} channelIds={channelIds} wet={wet} reports={reports} webSocketDefaultParametersSetter={webSocketDefaultParametersSetter} channelParameterHandler={channelParameterHandler} instanceParameterHandler={instanceParameterHandler} interfaceParameterHandler={interfaceParameterHandler} /> }
        { instance_id === 'distopik_eqp1_1' &&      <DEQP1_Faceplate            parameters={parameters} channelIds={channelIds} wet={wet} reports={reports} webSocketDefaultParametersSetter={webSocketDefaultParametersSetter} channelParameterHandler={channelParameterHandler} instanceParameterHandler={instanceParameterHandler} interfaceParameterHandler={interfaceParameterHandler} /> }
        { instance_id === 'distopik_fc670_1' &&     <DFC670_Faceplate           parameters={parameters} channelIds={channelIds} wet={wet} reports={reports} webSocketDefaultParametersSetter={webSocketDefaultParametersSetter} channelParameterHandler={channelParameterHandler} instanceParameterHandler={instanceParameterHandler} interfaceParameterHandler={interfaceParameterHandler} /> }
        { instance_id === 'distopik_la2a_1' &&      <DLA2A_Faceplate            parameters={parameters} channelIds={channelIds} wet={wet} reports={reports} webSocketDefaultParametersSetter={webSocketDefaultParametersSetter} channelParameterHandler={channelParameterHandler} instanceParameterHandler={instanceParameterHandler} interfaceParameterHandler={interfaceParameterHandler} /> }
        { instance_id === 'distopik_meq_1' &&       <DMEQ_Faceplate             parameters={parameters} channelIds={channelIds} wet={wet} reports={reports} webSocketDefaultParametersSetter={webSocketDefaultParametersSetter} channelParameterHandler={channelParameterHandler} instanceParameterHandler={instanceParameterHandler} interfaceParameterHandler={interfaceParameterHandler} /> }
        { instance_id === 'distopik_vca_1' &&       <DVCA_Faceplate             parameters={parameters} channelIds={channelIds} wet={wet} reports={reports} webSocketDefaultParametersSetter={webSocketDefaultParametersSetter} channelParameterHandler={channelParameterHandler} instanceParameterHandler={instanceParameterHandler} interfaceParameterHandler={interfaceParameterHandler} /> }
        { instance_id === 'elysia_karacter_1' &&    <Elysia_karacter_Faceplate  parameters={parameters} channelIds={channelIds} wet={wet} reports={reports} webSocketDefaultParametersSetter={webSocketDefaultParametersSetter} channelParameterHandler={channelParameterHandler} instanceParameterHandler={instanceParameterHandler} interfaceParameterHandler={interfaceParameterHandler} /> }
        { instance_id === 'elysia_museq_1' &&       <Elysia_museq_Faceplate     parameters={parameters} channelIds={channelIds} wet={wet} reports={reports} webSocketDefaultParametersSetter={webSocketDefaultParametersSetter} channelParameterHandler={channelParameterHandler} instanceParameterHandler={instanceParameterHandler} interfaceParameterHandler={interfaceParameterHandler} /> }
        { instance_id === 'elysia_nvelope_1' &&     <Elysia_nvelope_Faceplate   parameters={parameters} channelIds={channelIds} wet={wet} reports={reports} webSocketDefaultParametersSetter={webSocketDefaultParametersSetter} channelParameterHandler={channelParameterHandler} instanceParameterHandler={instanceParameterHandler} interfaceParameterHandler={interfaceParameterHandler} /> }
        { instance_id === 'elysia_xfilter_1' &&     <Elysia_xfilter_Faceplate   parameters={parameters} channelIds={channelIds} wet={wet} reports={reports} webSocketDefaultParametersSetter={webSocketDefaultParametersSetter} channelParameterHandler={channelParameterHandler} instanceParameterHandler={instanceParameterHandler} interfaceParameterHandler={interfaceParameterHandler} /> }
        { instance_id === 'elysia_xpressor_1' &&    <Elysia_xpressor_Faceplate  parameters={parameters} channelIds={channelIds} wet={wet} reports={reports} webSocketDefaultParametersSetter={webSocketDefaultParametersSetter} channelParameterHandler={channelParameterHandler} instanceParameterHandler={instanceParameterHandler} interfaceParameterHandler={interfaceParameterHandler} /> }
        { instance_id === 'gyraf_g24_1' &&          <Gyraf_G24_Faceplate        parameters={parameters} channelIds={channelIds} wet={wet} reports={reports} webSocketDefaultParametersSetter={webSocketDefaultParametersSetter} channelParameterHandler={channelParameterHandler} instanceParameterHandler={instanceParameterHandler} interfaceParameterHandler={interfaceParameterHandler} /> }
        { instance_id === 'tierra_gravity_1' &&     <Tierra_Gravity_Faceplate   parameters={parameters} channelIds={channelIds} wet={wet} reports={reports} webSocketDefaultParametersSetter={webSocketDefaultParametersSetter} channelParameterHandler={channelParameterHandler} instanceParameterHandler={instanceParameterHandler} interfaceParameterHandler={interfaceParameterHandler} /> }
        { instance_id === 'neve_1084_1' &&          <Neve_1084_Faceplate        parameters={parameters} channelIds={channelIds} wet={wet} reports={reports} webSocketDefaultParametersSetter={webSocketDefaultParametersSetter} channelParameterHandler={channelParameterHandler} instanceParameterHandler={instanceParameterHandler} interfaceParameterHandler={interfaceParameterHandler} /> } */}
      </div>

      {/*
        * - [ ] btrmkr_ml_1
        * - [ ] tierra_gravity_1
        * - [ ] elysia_karacter_1
        * - [ ] elysia_museq_1
        * - [ ] elysia_nvelope_1
        * - [ ] elysia_xfilter_1
        * - [ ] elysia_xpressor_1
        * - [ ] gyraf_g24_1
        * - [ ] distopik_1176_1
        * 
        * - [ ] distopik_adu_1
        * - [ ] distopik_bwl_1
        * - [ ] distopik_eqp1_1
        * - [ ] distopik_fc670_1
        * - [ ] distopik_la2a_1
        * - [ ] distopik_meq_1
        * - [ ] distopik_vca_1
        * 
        * - [ ] neve_1084_1
        */}
        
    </div>
  )
}

export default LiveControlPanel