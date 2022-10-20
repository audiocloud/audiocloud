# Media Services

Media objects are files that can be used as inputs on track nodes. They are created either by downloading from some
arbitrary remote App storage to our local storage or as render outputs from the Engine.

Renders from the engine must be "imported" before they are usable as inputs in subsequent plays and renders.

## Restarts

When the domain server restarts, uploads/downloads must restart and try to complete normally. The full state of the
database must persistent over service restarts and machine reboots.

## Metadata

Metadata for renders is provided by Engine when it says it is done with a render. Metadata for App files is provided
when source URLs are provided.

## Upload / Download info

When a media object is referenced by a task, nothing happens by default. Status packets sent by the task will include
the media object IDs that are not resolved (and thus not playing). Only when the App POSTs the information about the
file, will a download job be scheduled and executed.

## Giving up

After a configurable number of attempts, the jobs will be cancelled. The app can POST new info again to retry.

## Updating

If a job is processing, the next retry of the job (or start if job does not exist or has been cancelled) will use the
new info.

## Querying

An app can query if a media object is already present. No batch queries or filters.

# Design

We'll be using a key-value store to store the state of the jobs and media.

## Task created

When a task is created, its spec may reference media files that don't exist. No action is taken by the system, but
rendering is blocked and playback may lack the full information / resolution since media is missing.

## Task updated

When a task is updated the its spec may reference media files that don't exist. This is fine and will not interrupt any
rendering or playback.

## Task render completed

When a task render is completed, the engine will send a message to the domain server and the media will be moved from
the engine working directory and into the domain server media directory. The domain server will then update the media
database with the new media object metadata and location.

## Providing Download info

When a task is created or updated, the App can provide a URL to download the media file from. The domain server will
then schedule a download job and execute it. The download job will download the file and update the media database

## Deleting files

Files can be manually deleted by the app when they are deleted at the source.

## Cache busting

If a media is not linked with any task for a prolonged time, it will be evicted. The eviction will be done in a
