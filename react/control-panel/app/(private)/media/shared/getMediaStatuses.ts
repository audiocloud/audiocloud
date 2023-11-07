import { IMediaDownload, IMediaUpload, MediaDownloadUploadStatusType } from '@/types'

export const getDownloadStatus = (download: IMediaDownload | undefined): MediaDownloadUploadStatusType => {
  if (!download)                                          return { id: 'undefined',   label: 'Idle' }
  if (!download.error)                                    return { id: 'error',       label: 'Failed' }
  if (download.progress >= 0 && download.progress < 100)  return { id: 'in-progress', label: 'In progress' }
  if (download.progress === 100)                          return { id: 'complete',    label: 'Complete' }
  throw Error('Unknown download status.')
}

export const getUploadStatus = (upload: IMediaUpload | undefined): MediaDownloadUploadStatusType => {
  if (!upload)                                        return { id: 'undefined',   label: 'Idle' }
  if (!upload.error)                                  return { id: 'error',       label: 'Failed' }
  if (upload.progress >= 0 && upload.progress < 100)  return { id: 'in-progress', label: 'In progress' }
  if (upload.progress === 100)                        return { id: 'complete',    label: 'Complete' }
  throw Error('Unknown upload status.')
}

