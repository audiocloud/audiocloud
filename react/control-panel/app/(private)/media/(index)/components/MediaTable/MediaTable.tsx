import React from 'react'
import { media } from '@/data/media'
import { Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import MediaButtonLink from '@/components/general/ButtonLinks/MediaButtonLink'
import MediaDownloadStatus from '../../../shared/MediaDownloadStatus'
import MediaUploadStatus from '../../../shared/MediaUploadStatus'
import MediaActions from './MediaActions'
import { getDownloadStatus, getUploadStatus } from '../../../shared/getMediaStatuses'

const MediaTable: React.FC = () => {

  const mediaList = Object.values(media)

  return (
    <Table>
        
      { !mediaList.length && <TableCaption>No media objects found.</TableCaption> }

      <TableHeader>
        <TableRow>
          <TableHead className='whitespace-nowrap'>Download (domain)</TableHead>
          <TableHead className='whitespace-nowrap'>Upload (S3)</TableHead>
          <TableHead className='whitespace-nowrap'>Media ID</TableHead>
          <TableHead className='whitespace-nowrap'>App ID</TableHead>
          <TableHead className='whitespace-nowrap'>Channels</TableHead>
          <TableHead className='whitespace-nowrap'>Sample Rate</TableHead>
          <TableHead className='whitespace-nowrap'>Bit Depth</TableHead>
          <TableHead>Format</TableHead>
          <TableHead>Codec</TableHead>
          <TableHead>Length</TableHead>
          <TableHead>Size</TableHead>
          <TableHead className='text-right'>Actions</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody className='text-slate-400'>
        { mediaList.map((media) => (
          <TableRow className='group/row' key={media.id}>
            <TableCell><MediaDownloadStatus status={getDownloadStatus(media.download)} /></TableCell>
            <TableCell><MediaUploadStatus status={getUploadStatus(media.upload)} /></TableCell>
            <TableCell><MediaButtonLink media_id={media.id}/></TableCell>
            <TableCell>{ media.app_id }</TableCell>
            <TableCell>{ media.metadata?.channels ?? '' }</TableCell>
            <TableCell>{ media.metadata?.sample_rate ? `${(media.metadata.sample_rate / 1000).toFixed(1)} khz` : '' }</TableCell>
            <TableCell>{ media.metadata?.bit_depth ? `${media.metadata.bit_depth}${media.metadata.bit_depth === 32 ? '-bit float' : '-bit'}`: '' }</TableCell>
            <TableCell>{ media.metadata?.format ? `.${media.metadata.format}` : ''}</TableCell>
            <TableCell>{ media.metadata?.codec ? `.${media.metadata.codec}` : ''}</TableCell>
            <TableCell>{ media.metadata?.length ? `${media.metadata.length / 60} min ${media.metadata.length % 60} sec` : ''}</TableCell>
            <TableCell>{ media.metadata?.size ? `${(media.metadata.size / 1000000).toFixed(1)} MB` : ''}</TableCell>
            <TableCell className='text-right'><MediaActions media={media}/></TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  )
}

export default MediaTable