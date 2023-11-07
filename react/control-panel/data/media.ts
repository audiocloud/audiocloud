import { IMedia } from '@/types'

export const media: Record<string, IMedia> = {
  '0': {
    id: 'media-0',
    app_id: 'tape_cloud',
    metadata: {
      length: 60,
      channels: 2,
      sample_rate: 48000,
      bit_depth: 24,
      size: 22000000,
      format: 'wav',
      codec: 'pcm_s24le'
    },
    upload: {
      url: 'some_upload_url_that_is_really_really_long_longer_the_longest_wow_this_is_actually_long_now_huh',
      notify_url: 'some_notify_url_that_is_really_really_long_longer_the_longest_wow_this_is_actually_long_now_huh',
      context: 'some_context',
      attempts: 1,
      progress: 100
    }
  },
  '1': {
    id: 'media-1',
    app_id: 'tape_cloud',
    metadata: {
      length: 60,
      channels: 2,
      sample_rate: 48000,
      bit_depth: 24,
      size: 22000000,
      format: 'wav',
      codec: 'pcm_s24le'
    },
    upload: {
      url: 'some_upload_url_that_is_really_really_long_longer_the_longest_wow_this_is_actually_long_now_huh',
      notify_url: 'some_notify_url_that_is_really_really_long_longer_the_longest_wow_this_is_actually_long_now_huh',
      context: 'some_context',
      attempts: 5,
      error: 'some_error',
      progress: 24
    }
  },
  '2': {
    id: 'media-2',
    app_id: 'tape_cloud',
    metadata: {
      length: 60,
      channels: 2,
      sample_rate: 48000,
      bit_depth: 24,
      size: 22000000,
      format: 'wav',
      codec: 'pcm_s24le'
    },
    download: {
      url: 'some_download_url_that_is_really_really_long_longer_the_longest_wow_this_is_actually_long_now_huh',
      notify_url: 'some_notify_url_that_is_really_really_long_longer_the_longest_wow_this_is_actually_long_now_huh',
      context: 'some_context',
      attempts: 5,
      error: 'some_error',
      progress: 24
    }
  }
}