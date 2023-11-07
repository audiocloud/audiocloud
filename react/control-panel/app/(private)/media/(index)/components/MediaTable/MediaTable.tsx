import React from 'react'
import { media } from '@/data/media'
import { Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import MediaButtonLink from '@/components/general/MediaButtonLink'
import MediaDownloadStatus from './MediaDownloadStatus'
import MediaUploadStatus from './MediaUploadStatus '
import MediaActions from './MediaActions'
import { getDownloadStatus, getUploadStatus } from './getMediaStatuses'

const MediaTable: React.FC = () => {

  const mediaList = Object.values(media)

  return (
    <Table>
        
      { !mediaList.length && <TableCaption>No audio engines found.</TableCaption> }

      <TableHeader>
        <TableRow>
          <TableHead>Download (domain)</TableHead>
          <TableHead>Upload (S3)</TableHead>
          <TableHead>Media ID</TableHead>
          <TableHead>App ID</TableHead>
          <TableHead>Channels</TableHead>
          <TableHead>Sample Rate</TableHead>
          <TableHead>Bit Depth</TableHead>
          <TableHead>Format</TableHead>
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
            <TableCell>{ media.metadata?.length ? `${media.metadata.length / 60} min ${media.metadata.length % 60} sec` : ''}</TableCell>
            <TableCell>{ media.metadata?.size ? `${media.metadata.size / 1000000} MB` : ''}</TableCell>
            <TableCell className='text-right'><MediaActions media={media}/></TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  )
}

export default MediaTable