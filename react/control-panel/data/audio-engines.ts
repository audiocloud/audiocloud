import { IAudioEngine } from '@/types'

export const audio_engines: Record<string, IAudioEngine> = {
  '0': {
    id: 'audio-engine-0',
    maintenance: [{
      start: '2023-01-04T10:21:00.000Z',
      end: '2023-01-04T11:21:00.000Z',
      header: 'Ticket checkup',
      body: `
      What is Lorem Ipsum?
Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.

Why do we use it?
It is a long established fact that a reader will be distracted by the readable content of a page when looking at its layout. The point of using Lorem Ipsum is that it has a more-or-less normal distribution of letters, as opposed to using 'Content here, content here', making it look like readable English. Many desktop publishing packages and web page editors now use Lorem Ipsum as their default model text, and a search for 'lorem ipsum' will uncover many web sites still in their infancy. Various versions have evolved over the years, sometimes by accident, sometimes on purpose (injected humour and the like).
      `,
      updated_at: '2023-01-24T12:51:00.000Z'
    }],
    status: 'online',
    last_seen: '2023-01-24T14:48:00.000Z',
    engine_tasks: [
      {
        nodes: [
          {
            id: 'engine_task_node_id_0',
            model_id: 'model_id_0',
            resources: {
              cpu: 200,
              memory: 1000,
              disk: 2000,
              antelope_dsp: 20,
              uad_dsp: 2,
              cuda_dsp: 6
            }
          },
          {
            id: 'engine_task_node_id_1',
            model_id: 'model_id_1',
            resources: {
              cpu: 200,
              memory: 1000,
              disk: 2000,
              antelope_dsp: 20,
              uad_dsp: 2,
              cuda_dsp: 6
            }
          }
        ]
      },
      {
        nodes: [
          {
            id: 'engine_task_node_id_0',
            model_id: 'model_id_0',
            resources: {
              cpu: 200,
              memory: 1000,
              disk: 2000,
              antelope_dsp: 20,
              uad_dsp: 2,
              cuda_dsp: 6
            }
          }
        ]
      }
    ],
    machine: 'some_machine',
    buffer_size: 512,
    sample_rate: 192000,
    bit_depth: 32,
    inputs: [0, 1],
    outputs: [0, 1],
    models: [
      'model_id_0',
      'model_id_1',
      'model_id_2'
    ],
    resources: {
      cpu: 600,
      memory: 32000,
      disk: 1000,
      antelope_dsp: 100,
      uad_dsp: 100,
      cuda_dsp: 100,
    }
  },
  '1': {
    id: 'audio-engine-1',
    maintenance: [
      {
        start: '2023-01-04T10:21:00.000Z',
        end: '2023-01-04T11:21:00.000Z',
        header: 'Weekly checkup',
        body: `
        What is Lorem Ipsum?
Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.

Why do we use it?
It is a long established fact that a reader will be distracted by the readable content of a page when looking at its layout. The point of using Lorem Ipsum is that it has a more-or-less normal distribution of letters, as opposed to using 'Content here, content here', making it look like readable English. Many desktop publishing packages and web page editors now use Lorem Ipsum as their default model text, and a search for 'lorem ipsum' will uncover many web sites still in their infancy. Various versions have evolved over the years, sometimes by accident, sometimes on purpose (injected humour and the like).
        `,
        updated_at: '2023-01-24T12:51:00.000Z'
      },
      {
        start: '2023-01-05T10:21:00.000Z',
        header: 'Software update',
        body: 'Description of maintenance. This can be markdown text.',
        updated_at: '2023-01-24T12:51:00.000Z'
      }
    ],
    status: 'online',
    last_seen: '2023-01-24T14:48:00.000Z',
    engine_tasks: [
      {
        nodes: [
          {
            id: 'engine_task_node_id_1',
            model_id: 'model_id_1',
            resources: {
              cpu: 200,
              memory: 1000,
              disk: 2000,
              antelope_dsp: 20,
              uad_dsp: 2,
              cuda_dsp: 6
            }
          }
        ]
      }
    ],
    machine: 'some_machine',
    buffer_size: 512,
    sample_rate: 192000,
    bit_depth: 32,
    inputs: [2, 3],
    outputs: [2, 3],
    models: [],
    resources: {
      cpu: 200,
      memory: 32000,
      disk: 1000,
      antelope_dsp: 100,
      uad_dsp: 100,
      cuda_dsp: 100,
    }
  },
  '2': {
    id: 'audio-engine-2',
    maintenance: [{
      start: '2023-01-03T10:21:00.000Z',
      end: '2023-01-03T11:21:00.000Z',
      header: 'Weekly checkup',
      body: 'Description of maintenance. This can be markdown text.',
      updated_at: '2023-01-24T12:51:00.000Z'
    },
    {
      start: '2023-01-03T10:21:00.000Z',
      header: 'Urgent service',
      body: 'Description of maintenance. This can be markdown text.',
      updated_at: '2023-01-24T12:51:00.000Z'
    }],
    status: 'offline',
    last_seen: '2023-01-03T10:21:00.000Z',
    engine_tasks: [
      {
        nodes: [
          {
            id: 'engine_task_node_id_2',
            model_id: 'model_id_2',
            resources: {
              cpu: 200,
              memory: 1000,
              disk: 2000,
              antelope_dsp: 20,
              uad_dsp: 2,
              cuda_dsp: 6
            }
          }
        ]
      }
    ],
    machine: 'some_machine',
    buffer_size: 512,
    sample_rate: 192000,
    bit_depth: 32,
    inputs: [4, 5],
    outputs: [4, 5],
    models: [],
    resources: {
      cpu: 200,
      memory: 32000,
      disk: 1000,
      antelope_dsp: 100,
      uad_dsp: 100,
      cuda_dsp: 100,
    }
  },
}