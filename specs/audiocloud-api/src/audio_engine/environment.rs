/// Set media presence
///
/// The Audio Engine needs to map AppMediaObjectId on track items to
#[utoipa::path(
 put,
 path = "/v1/media",
 request_body = SetMedia,
 responses(
  (status = 200, description = "Success", body = MediaUpdated),
  (status = 404, description = "Not found", body = EngineError),
 ))]
pub(crate) fn set_media() {}

/// Set instance I/O mapping
///
/// The Audio Engine needs to map FixedInstanceNode to I/O on the audio interface it is bound
/// to. For example, an instance may be bound to channels 0 and 1 or to channels 5 and 6 and
/// the Audio Engine needs to know to route the audio correctly.
#[utoipa::path(
 put,
 path = "/v1/instances",
 request_body = SetInstances,
 responses(
  (status = 200, description = "Success", body = InstancesUpdated),
  (status = 404, description = "Not found", body = EngineError),
 ))]
pub(crate) fn set_instances() {}
