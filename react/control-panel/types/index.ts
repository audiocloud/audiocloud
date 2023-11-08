import { InstanceParameters } from '@audiocloud/api'

export interface Domain {
  id: string,
  audio_engines: IAudioEngine[],
  media_service: IMediaService,
  tasks_service: ITasksService,
  devices: IDevice[],
  drivers: string[],
}

export type AudioEngineStatusType = 'online' | 'offline'

export interface IAudioEngine {
  id: string,
  maintenance: IMaintenanceInfo[],
  status: AudioEngineStatusType,
  last_seen: string,
  engine_tasks: IEngineTask[],
  machine: string,
  buffer_size: number,
  sample_rate: number,
  bit_depth: number,
  inputs: number[],
  outputs: number[],
  models: Model[], // which models can be instantiated on this engine
  resources: {
    cpu: number,
    memory: number,
    disk: number,
    antelope_dsp?: number,
    uad_dsp?: number,
    cuda_dsp?: number,
  }
}

export type Model = string

export interface IEngineTask {
  nodes: IEngineTaskNode[]
}

export interface IEngineTaskNode {
  id: string,
  model_id: string,
  resources: {
    cpu: number, // MHz
    memory: number, // megabytes
    disk: number, // megabytes
    antelope_dsp?: number, // antelope percentages
    uad_dsp?: number, // uad cores
    cuda_dsp?: number, // cuda cores
  }
}

export interface IMediaService {
  id: string,
  media: IMedia[],
}

export interface IMedia {
  id: string,
  app_id: string,
  metadata?: IMediaMetadata,
  download?: IMediaDownload,
  upload?: IMediaUpload,
}

export interface IMediaMetadata {
  length: number,
  channels: number,
  sample_rate: number,
  bit_depth: number,
  size: number,
  format: 'flac' | 'wav' | 'mp3',
  codec: 'flac' | 'pcm_s16le' | 'pcm_s16be' | 'pcm_s24le' | 'pcm_s32le' | 'pcm_f32le' | 'mp3',
}

export interface IMediaDownload {
  url: string,
  notify_url?: string,
  context?: unknown,
  attempts: number,
  error?: string,
  progress: number,
}

export interface IMediaUpload {
  url: string,
  notify_url?: string,
  context?: unknown,
  attempts: number,
  error?: string,
  progress: number,
}

export type MediaDownloadUploadStatusType = {
  id: 'undefined',
  label: 'Idle'
} | {
  id: 'error',
  label: 'Failed'
} | {
  id: 'in-progress',
  label: 'In progress'
} | {
  id: 'complete',
  label: 'Complete'
}

export interface ITasksService {
  id: string,
  tasks: ITask[],
}

export interface ITask {
  start: string,
  end: string,
  app_id: string,
  id: string,
  user_id?: string,
  status: TaskStatusType,
  nodes: ITaskNode[],
  mixers: ITaskMixer[],
  tracks: ITaskTrack[],
}

export type TaskStatusType = 'queued' | 'running' | 'complete' | 'error'

export interface ITaskNode {
  id: string
}

export interface ITaskMixer {
  id: string
}

export interface ITaskTrack {
  id: string
}

export type DeviceStatusType = 'online' | 'offline'

export interface IDevice {
  id: string,
  model_id: string,
  engine_id: string,
  driver_id: string,
  status: DeviceStatusType,
  last_seen: string,
  engine_input_at: number,
  engine_output_at: number,
  driver_attachment_url: string,
  media_config?: string,
  power_config?: IPowerConfig,
  maintenance: IMaintenanceInfo[],
}

export interface IDeviceParametersConfig {
  channel_ids: string[],
  parameters: InstanceParameters,
  wet: number
}

export type DeviceReportsType = Record<string, (string | number | boolean)[]>

export interface IPowerConfig {
  warm_up_delay_ms: number, // how long to wait after powering on before considered fully warmed up
  cool_down_delay_ms: number, // how long to wait after shutting down before considered fully powered off (do not power on before cooldown is complete!)
  idle_shutdown_timeout_ms: number, // after what time of no tasks using this device will the device automatically power off
}

export interface IMaintenanceInfo {
  start: string,
  end?: string,
  header: string, // plain text
  body: string, // markdown
  updated_at: string,
}

export interface IAudioEngineMaintenance {
  key: string,
  engine_id: string,
  data: IMaintenanceInfo
}

export interface IDeviceMaintenance {
  key: string,
  device_id: string,
  data: IMaintenanceInfo
}


export type MediaAlertType = { key: string, media_id: string, data: IMediaUpload | IMediaDownload }