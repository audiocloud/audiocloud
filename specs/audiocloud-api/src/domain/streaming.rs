//! API definitions for communicating with the apps
use std::collections::HashMap;

use chrono::Utc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::common::change::TaskPlayState;
use crate::common::media::{PlayId, RenderId};
use crate::common::time::Timestamp;
use crate::domain::tasks::TaskUpdated;
use crate::domain::DomainError;
use crate::{
    AppTaskId, ClientSocketId, ModifyTaskSpec, RequestId, SecureKey, SerializableResult, SocketId,
    TaskEvent, TaskPermissions,
};

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct StreamStats {
    pub id: AppTaskId,
    pub play_id: PlayId,
    pub state: TaskPlayState,
    pub low: Option<u64>,
    pub high: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SessionPacketError {
    Playing(PlayId, String),
    Rendering(RenderId, String),
    General(String),
}

/// Difference stamped in milliseconds since a common epoch, in order to pack most efficiently
/// The epoch in InstancePacket is the created_at field of SessionPacket
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct DiffStamped<T>(usize, T);

impl<T> DiffStamped<T> {
    pub fn new(timestamp: Timestamp, value: T) -> Self {
        (timestamp, value).into()
    }

    pub fn value(&self) -> &T {
        &self.1
    }

    pub fn value_mut(&mut self) -> &mut T {
        &mut self.1
    }
}

impl<T> From<(Timestamp, T)> for DiffStamped<T> {
    fn from(value: (Timestamp, T)) -> Self {
        let (timestamp, value) = value;
        let diff = Utc::now() - timestamp;
        Self(diff.num_milliseconds() as usize, value)
    }
}

/// A mesasge received over a real-time communication channel from a streaming domain connection
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DomainServerMessage {
    /// Task generated event
    TaskEvent {
        /// Id of the task generating the event
        task_id: AppTaskId,
        /// Event details
        event: TaskEvent,
    },
    /// Response to a request to change a task play state
    SetDesiredPlayStateResponse {
        /// Request id this message is responding to
        request_id: RequestId,
        /// Result
        result: SerializableResult<TaskUpdated, DomainError>,
    },
    /// Response to a request to change task specification
    ModifyTaskSpecResponse {
        /// Request id this message is responding to
        request_id: RequestId,
        /// Result of the operation
        result: SerializableResult<TaskUpdated, DomainError>,
    },
    /// Response to initiating a new peer connection
    PeerConnectionResponse {
        /// Request id this message is responding to
        request_id: RequestId,
        /// Result of the operation - the assigned socket ID
        result: SerializableResult<PeerConnectionCreated, DomainError>,
    },
    AnswerPeerConnectionResponse {
        /// Request id this message is responding to
        request_id: RequestId,
        /// Result of the operation or error
        result: SerializableResult<(), DomainError>,
    },
    /// Response to submitting a peer connection candidate
    PeerConnectionCandidateResponse {
        /// Request id this message is responding to
        request_id: RequestId,
        /// Result of the operation
        result: SerializableResult<(), DomainError>,
    },
    /// Response to a request to attach the socket to a task
    AttachToTaskResponse {
        /// Request id this message is responding to
        request_id: RequestId,
        /// Result of the operation
        result: SerializableResult<(), DomainError>,
    },
    /// Response to detach the socket from a task
    DetachFromTaskResponse {
        /// Request id this message is responding to
        request_id: RequestId,
        /// Result of the operation - will be success even if task does not exist
        result: SerializableResult<(), DomainError>,
    },
    /// Submit a new WebRTC peer connection ICE candidate
    SubmitPeerConnectionCandidate {
        /// Socket id of the peer connection
        socket_id: SocketId,
        /// ICE Candidate
        candidate: Option<String>,
    },
    /// Ping message
    Ping {
        /// Challenge string
        ///
        /// In a future release, this field will contain a challenge that must be processed and returned
        /// to validate that the client is running a valid version of the client code
        challenge: String,
    },
    /// Notify the task permissions on this socket
    NotifyTaskPermissions {
        /// Mapping from each available task to permission information to that task
        permissions: HashMap<AppTaskId, TaskPermissions>,
    },
}

/// Confirmation that the socket has been created normally from the domain client offer
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PeerConnectionCreated {
    /// Connection created normally
    Created {
        /// Created socket id
        socket_id: ClientSocketId,

        /// The domain server's WebRTC offer
        remote_description: String,
    },
}

/// A message sent over a real-time communication channel to a streaming domain connection
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DomainClientMessage {
    /// Request to modify task specification
    RequestModifyTaskSpec {
        /// Request id (to reference the response to)
        request_id: RequestId,
        /// Id of the task to modify
        task_id: AppTaskId,
        /// List of modifications to apply
        modify_spec: Vec<ModifyTaskSpec>,
        /// If true, the modifications are optional (no error if task already diverged)
        optional: bool,
        /// Task version
        revision: u64,
    },
    /// Request a new WebRTC peer connection to the domain
    RequestPeerConnection {
        /// Request id (to reference the response to)
        request_id: RequestId,
    },
    AnswerPeerConnection {
        /// The socket for which we are generating an anwser
        socket_id: SocketId,
        /// Request id (to reference the response to)
        request_id: RequestId,
        /// The domain server's WebRTC offer response (answer)
        answer: String,
    },
    /// Submit a new WebRTC peer connection ICE candidate
    SubmitPeerConnectionCandidate {
        /// Request id (to reference the response to)
        request_id: RequestId,
        /// Socket id of the peer connection
        socket_id: SocketId,
        /// ICE Candidate
        candidate: Option<String>,
    },
    /// Request attaching to a task
    RequestAttachToTask {
        /// Request id (to reference the response to)
        request_id: RequestId,
        /// Id of the task to attach to
        task_id: AppTaskId,
        /// Secure key to use for attachment
        secure_key: SecureKey,
    },
    RequestDetachFromTask {
        /// Request id (to reference the response to)
        request_id: RequestId,
        /// Id of the task to attach to
        task_id: AppTaskId,
    },
    Pong {
        challenge: String,
        response: String,
    },
}

/// Load packet data
///
/// For each PlayId, on a task, a stream is kept in memory with a history of packets, by ascending
/// serial number. For a sane amount of time, the packets may be requested by the clients. If a
/// packet is not yet models (but it is expected they will be, in the future) the request will
/// block (wait) for `Timeout` milliseconds before giving up and returning 408.
#[utoipa::path(
  get,
  path = "/v1/streams/{app_id}/{task_id}/{play_id}/packet/{serial}",
  responses(
    (status = 200, description = "Success", body = StreamingPacket),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "App, task or stream not found", body = DomainError),
    (status = 408, description = "Timed out waiting for packet", body = DomainError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id"),
    ("play_id" = PlayId, Path, description = "Play id"),
    ("serial" = u64, Path, description = "Packet serial number"),
    ("Timeout" = u64, Header, description = "Milliseconds to wait for the packet to be ready")
  ))]
pub(crate) fn stream_packets() {}

/// Get stream statistics
///
/// Get statistics about cached packets available in the stream.
#[utoipa::path(
  get,
  path = "/v1/streams/{app_id}/{task_id}/{play_id}",
  responses(
    (status = 200, description = "Success", body = StreamStats),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "Not found", body = DomainError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id"),
    ("play_id" = PlayId, Path, description = "Play id")
  ))]
pub(crate) fn stream_stats() {}
