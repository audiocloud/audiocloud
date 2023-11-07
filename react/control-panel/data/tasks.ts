import { ITask } from '@/types'

export const tasks: Record<string, ITask> = {
  '0': {
    id: 'task-0',
    app_id: 'tape_cloud',
    start: '2011-01-24T19:45:00.000Z',
    end: '2011-01-24T19:48:00.000Z',
    status: 'queued',
    nodes: [
      { id: 'node-id-0' }
    ],
    mixers: [
      { id: 'mixer-id-0' }
    ],
    tracks: [
      { id: 'track-id-0' }
    ],
  },
  '1': {
    id: 'task-1',
    app_id: 'tape_cloud',
    start: '2011-01-24T19:49:00.000Z',
    end: '2011-01-24T19:51:00.000Z',
    status: 'running',
    nodes: [
      { id: 'node-id-1' }
    ],
    mixers: [
      { id: 'mixer-id-1' }
    ],
    tracks: [
      { id: 'track-id-1' }
    ],
  },
  '2': {
    id: 'task-2',
    app_id: 'tape_cloud',
    start: '2011-01-24T19:52:00.000Z',
    end: '2011-01-24T19:53:00.000Z',
    status: 'error',
    nodes: [
      { id: 'node-id-2' }
    ],
    mixers: [
      { id: 'mixer-id-2' }
    ],
    tracks: [
      { id: 'track-id-2' }
    ],
  }
}