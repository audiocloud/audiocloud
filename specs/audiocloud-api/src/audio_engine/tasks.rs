/// Create or update task spec
///
/// Create or update a task by providing its spec. Changing the spec even trivially could result
/// in a rendering or playback interruption.
#[utoipa::path(
 put,
 path = "/v1/tasks/{app_id}/{task_id}",
 request_body = TaskSpec,
 responses(
  (status = 200, description = "Success", body = TaskReplaced),
  (status = 401, description = "Not authorized", body = EngineError),
 ),
 params(
  ("app_id" = AppId, Path, description = "App id"),
  ("task_id" = TaskId, Path, description = "Task id")
 ))]
pub(crate) fn set_spec() {}

/// Modify task spec
///
/// Apply a modification to an existing spec. Changing the spec even trivially could result in a
/// rendering or playback interruption. The task must exist in order to be modified.
#[utoipa::path(
 patch,
 path = "/v1/tasks/{app_id}/{task_id}",
 request_body = ModifyTaskSpec,
 responses(
  (status = 200, description = "Success", body = TaskModified),
  (status = 400, description = "Modification failed", body = EngineError),
  (status = 404, description = "Not found", body = EngineError),
 ),
 params(
  ("app_id" = AppId, Path, description = "App id"),
  ("task_id" = TaskId, Path, description = "Task id")
 ))]
pub(crate) fn modify_spec() {}

/// Delete a task
///
/// Delete an existing task spec. This will interrupt any playback or rendering and will free
/// resources associated with the task (such as instances or locks on media files).
#[utoipa::path(
 delete,
 path = "/v1/tasks/{app_id}/{task_id}",
 responses(
  (status = 200, description = "Success", body = TaskDeleted),
  (status = 404, description = "Not found", body = EngineError),
 ),
 params(
  ("app_id" = AppId, Path, description = "App id"),
  ("task_id" = TaskId, Path, description = "Task id")
 ))]
pub(crate) fn delete() {}

/// List tasks
///
/// Return a list of all current tasks and their play status.
#[utoipa::path(
 get,
 path = "/v1/tasks",
 responses(
  (status = 200, description = "Success", body = TaskWithStatusList),
  (status = 404, description = "Not found", body = EngineError),
 ))]
pub(crate) fn list() {}

/// Start playing a task
///
/// Start playing a task that is stopped. The request will return when the task has started to play
/// or with an error.
#[utoipa::path(
  post,
  path = "/v1/tasks/{app_id}/{task_id}/transport/play",
  request_body = RequestPlay,
  responses(
    (status = 200, description = "Success", body = TaskPlaying),
    (status = 404, description = "Not found", body = EngineError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id")
  ))]
pub(crate) fn play() {}

/// Seek while task is playing
///
/// If the task is playing, change the playing position.
#[utoipa::path(
  post,
  path = "/v1/tasks/{app_id}/{task_id}/transport/seek",
  request_body = RequestSeek,
  responses(
    (status = 200, description = "Success", body = TaskSought),
    (status = 404, description = "Task Not found", body = EngineError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id")
  ))]
pub(crate) fn seek() {}

/// Change the selected mixer
///
/// If the task is playing, change the mixer that is used to derive monitoring.
#[utoipa::path(
  post,
  path = "/v1/tasks/{app_id}/{task_id}/transport/mixer",
  request_body = RequestChangeMixer,
  responses(
    (status = 200, description = "Success", body = TaskMixerChanged),
    (status = 404, description = "Task or mixer Not found", body = EngineError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id")
  ))]
pub(crate) fn change_mixer() {}

/// Stop playing a task
///
/// Request to stop a track if the task is playing.
#[utoipa::path(
  post,
  path = "/v1/tasks/{app_id}/{task_id}/transport/stop",
  request_body = RequestStopPlay,
  responses(
    (status = 200, description = "Success", body = TaskPlayStopped),
    (status = 404, description = "Task or mixer Not found", body = EngineError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id")
  ))]
pub(crate) fn stop_playing() {}

/// Cancel rendering a task
///
/// Request to stop (cancel) rendering if the task is rendering.
#[utoipa::path(
  post,
  path = "/v1/tasks/{app_id}/{task_id}/transport/cancel",
  request_body = RequestCancelRender,
  responses(
    (status = 200, description = "Success", body = TaskRenderCancelled),
    (status = 404, description = "Task or mixer Not found", body = EngineError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id")
  ))]
pub(crate) fn cancel_render() {}

/// Render a task to a new file
///
/// Start rendering a task. Note that unlike the orchestration or domain API, the audio engine
/// does not care if the media files are present and will happily execute a render even when no
/// files (or instances) are ready. The caller to this API should make sure that any such
/// preconditions are met.
#[utoipa::path(
  post,
  path = "/v1/tasks/{app_id}/{task_id}/transport/render",
  request_body = RequestRender,
  responses(
    (status = 200, description = "Success", body = TaskRendering),
    (status = 404, description = "Task or mixer Not found", body = EngineError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id")
  ))]
pub(crate) fn render() {}
