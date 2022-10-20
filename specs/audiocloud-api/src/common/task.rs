use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::ops::Range;

use derive_more::{From, IsVariant, Unwrap};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::audio_engine::CompressedAudio;
use crate::cloud::tasks::CreateTask;
use crate::cloud::CloudError;
use crate::cloud::CloudError::*;
use crate::domain::streaming::DiffStamped;
use crate::{
    now, AppMediaObjectId, DesiredTaskPlayState, DomainId, DynamicInstanceNodeId, FixedInstanceId, FixedInstanceNodeId, MediaObjectId,
    MixerNodeId, Model, ModelId, NodeConnectionId, PlayId, SecureKey, TaskPlayState, TimeRange, Timestamp, Timestamped, TrackMediaId,
    TrackNodeId,
};

/// Task specification
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, JsonSchema)]
pub struct TaskSpec {
    /// Track nodes of the task
    #[serde(default)]
    pub tracks:      HashMap<TrackNodeId, TrackNode>,
    /// Mixer nodes of the task
    #[serde(default)]
    pub mixers:      HashMap<MixerNodeId, MixerNode>,
    /// Dynamic instance nodes of the task
    #[serde(default)]
    pub dynamic:     HashMap<DynamicInstanceNodeId, DynamicInstanceNode>,
    /// Fixed instance nodes of the task
    #[serde(default)]
    pub fixed:       HashMap<FixedInstanceNodeId, FixedInstanceNode>,
    /// Connections between nodes
    #[serde(default)]
    pub connections: HashMap<NodeConnectionId, NodeConnection>,
    /// The revision number of the specification (starts at zero, increments for every change)
    #[serde(default)]
    pub revision:    u64,
}

/// Create task spec
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, JsonSchema)]
pub struct CreateTaskSpec {
    /// Track nodes of the task
    #[serde(default)]
    pub tracks:      HashMap<TrackNodeId, TrackNode>,
    /// Mixer nodes of the task
    #[serde(default)]
    pub mixers:      HashMap<MixerNodeId, MixerNode>,
    /// Dynamic instance nodes of the task
    #[serde(default)]
    pub dynamic:     HashMap<DynamicInstanceNodeId, DynamicInstanceNode>,
    /// Fixed instance nodes of the task
    #[serde(default)]
    pub fixed:       HashMap<FixedInstanceNodeId, FixedInstanceNode>,
    /// Connections between nodes
    #[serde(default)]
    pub connections: HashMap<NodeConnectionId, NodeConnection>,
}

impl Into<TaskSpec> for CreateTaskSpec {
    fn into(self) -> TaskSpec {
        let Self { tracks,
                   mixers,
                   dynamic,
                   fixed,
                   connections, } = self;
        TaskSpec { tracks,
                   mixers,
                   dynamic,
                   fixed,
                   connections,
                   revision: 0 }
    }
}

impl TaskSpec {
    pub fn validate(&self, models: &HashMap<ModelId, Model>) -> Result<(), CloudError> {
        if self.fixed.is_empty() && self.dynamic.is_empty() && self.mixers.is_empty() && self.tracks.is_empty() {
            return Err(InternalInconsistency { message:
                                                   format!("No tracks, mixers, dynamic instances, or fixed instances declared in task spec"), });
        }

        for (connection_id, connection) in self.connections.iter() {
            self.validate_connection(connection_id, connection, models)?;
        }

        Ok(())
    }

    pub fn fixed_instance_to_fixed_id(&self, instance_id: &FixedInstanceId) -> Option<&FixedInstanceNodeId> {
        for (fixed_id, fixed) in &self.fixed {
            if &fixed.instance_id == instance_id {
                return Some(fixed_id);
            }
        }
        None
    }

    fn validate_connection(&self,
                           id: &NodeConnectionId,
                           connection: &NodeConnection,
                           models: &HashMap<ModelId, Model>)
                           -> Result<(), CloudError> {
        self.check_source_channel_exists(id, &connection.from, connection.from_channels, models)?;
        self.check_destination_channel_exists(id, &connection.to, connection.to_channels, models)?;

        Ok(())
    }

    fn check_source_channel_exists(&self,
                                   connection_id: &NodeConnectionId,
                                   pad_id: &OutputPadId,
                                   channels: ChannelMask,
                                   models: &HashMap<ModelId, Model>)
                                   -> Result<(), CloudError> {
        let complete_error = |error| ConnectionError { connection_id: connection_id.clone(),
                                                       error:         Box::new(error), };

        match pad_id {
            OutputPadId::MixerOutput(id) => self.mixers
                                                .get(id)
                                                .ok_or_else(|| MixerNodeNotFound { mixer_node_id: id.clone() })
                                                .and_then(|node| node.validate_source_channels(channels))
                                                .map_err(complete_error),
            OutputPadId::FixedInstanceOutput(id) => {
                let fixed = self.fixed
                                .get(id)
                                .ok_or_else(|| FixedInstanceNodeNotFound { fixed_node_id: id.clone() })
                                .map_err(complete_error)?;

                let model = models.get(&fixed.instance_id.model_id())
                                  .ok_or_else(|| ModelNotFound { model_id: fixed.instance_id.model_id(), })
                                  .map_err(complete_error)?;

                fixed.validate_source_channels(channels, model).map_err(complete_error)
            }
            OutputPadId::DynamicInstanceOutput(id) => {
                let dynamic = self.dynamic
                                  .get(id)
                                  .ok_or_else(|| DynamicInstanceNodeNotFound { dynamic_node_id: id.clone(), })
                                  .map_err(complete_error)?;

                let model = models.get(&dynamic.model_id)
                                  .ok_or_else(|| ModelNotFound { model_id: dynamic.model_id.clone(), })
                                  .map_err(complete_error)?;

                dynamic.validate_source_channels(channels, model).map_err(complete_error)
            }
            OutputPadId::TrackOutput(id) => self.tracks
                                                .get(id)
                                                .ok_or_else(|| TrackNodeNotFound { track_node_id: id.clone() })
                                                .and_then(|node| node.validate_source_channels(channels))
                                                .map_err(complete_error),
        }
    }

    fn check_destination_channel_exists(&self,
                                        connection_id: &NodeConnectionId,
                                        pad_id: &InputPadId,
                                        channels: ChannelMask,
                                        models: &HashMap<ModelId, Model>)
                                        -> Result<(), CloudError> {
        let complete_error = |error| ConnectionError { connection_id: connection_id.clone(),
                                                       error:         Box::new(error), };

        match pad_id {
            InputPadId::MixerInput(id) => self.mixers
                                              .get(id)
                                              .ok_or_else(|| MixerNodeNotFound { mixer_node_id: id.clone() })
                                              .and_then(|node| node.validate_destination_channels(channels))
                                              .map_err(complete_error),
            InputPadId::FixedInstanceInput(id) => {
                let fixed = self.fixed
                                .get(id)
                                .ok_or_else(|| FixedInstanceNodeNotFound { fixed_node_id: id.clone() })
                                .map_err(complete_error)?;

                let model = models.get(&fixed.instance_id.model_id())
                                  .ok_or_else(|| ModelNotFound { model_id: fixed.instance_id.model_id(), })
                                  .map_err(complete_error)?;

                fixed.validate_destination_channels(channels, model).map_err(complete_error)
            }
            InputPadId::DynamicInstanceInput(id) => {
                let dynamic = self.dynamic
                                  .get(id)
                                  .ok_or_else(|| DynamicInstanceNodeNotFound { dynamic_node_id: id.clone(), })
                                  .map_err(complete_error)?;

                let model = models.get(&dynamic.model_id)
                                  .ok_or_else(|| ModelNotFound { model_id: dynamic.model_id.clone(), })
                                  .map_err(complete_error)?;

                dynamic.validate_destination_channels(channels, model).map_err(complete_error)
            }
        }
    }

    fn check_channel_exists_mixer(&self, id: &NodeConnectionId, mixer_id: &MixerNodeId, channels: &ChannelMask) -> Result<(), CloudError> {
        let mixer =
            self.mixers
                .get(mixer_id)
                .ok_or_else(|| InternalInconsistency { message: format!("Connection {id} flow to mixer {mixer_id} does not exist"), })?;

        if !channels.is_subset_of(0..mixer.input_channels) {
            return Err(InternalInconsistency { message: format!("Connection {id} flow to mixer {mixer_id} has channels that do not exist"), });
        }

        Ok(())
    }

    fn check_channel_exists_fixed(&self,
                                  id: &NodeConnectionId,
                                  fixed_id: &FixedInstanceNodeId,
                                  channels: &ChannelMask,
                                  output: bool,
                                  models: &HashMap<ModelId, Model>)
                                  -> Result<(), CloudError> {
        let fixed = self.fixed.get(fixed_id).ok_or_else(|| InternalInconsistency { message: format!(
                    "Connection {id} references fixed {fixed_id} which does not exist"
                ), })?;

        let model_id = fixed.instance_id.model_id();
        let model = models.get(&model_id).ok_or_else(|| {
            InternalInconsistency { message: format!("Connection {id} references fixed instance labelled {fixed_id} which references model {model_id} which does not exist") }
        })?;

        if !channels.is_subset_of(0..(if output { model.outputs.len() } else { model.inputs.len() })) {
            return Err(InternalInconsistency { message: format!("Connection {id} references fixed instance labelled {fixed_id} which has channels that do not exist") });
        }

        Ok(())
    }

    fn check_channel_exists_dynamic(&self,
                                    id: &NodeConnectionId,
                                    dynamic_id: &DynamicInstanceNodeId,
                                    channels: &ChannelMask,
                                    output: bool,
                                    models: &HashMap<ModelId, Model>)
                                    -> Result<(), CloudError> {
        let dynamic = self.dynamic.get(dynamic_id).ok_or_else(|| {
            InternalInconsistency { message: format!("Connection {id} references dynamic instance labelled {dynamic_id} which does not exist") }
        })?;

        let model_id = &dynamic.model_id;
        let model = models.get(&model_id).ok_or_else(|| {
            InternalInconsistency { message: format!("Connection {id} references dynamic instance labelled {dynamic_id} which references model {model_id} which does not exist") }
        })?;

        if !channels.is_subset_of(0..(if output { model.outputs.len() } else { model.inputs.len() })) {
            return Err(InternalInconsistency { message: format!("Connection {id} references dynamic instance labelled {dynamic_id} which has channels that do not exist") });
        }

        Ok(())
    }

    fn check_channel_exists_track(&self, id: &NodeConnectionId, track_id: &TrackNodeId, channels: &ChannelMask) -> Result<(), CloudError> {
        let track = self.tracks.get(track_id).ok_or_else(|| InternalInconsistency { message: format!(
                    "Connection {id} references track {track_id} which does not exist"
                ), })?;

        if !channels.is_subset_of(0..track.channels.num_channels()) {
            return Err(InternalInconsistency { message: format!("Connection {id} references track {track_id} which has channels that do not exist") });
        }

        Ok(())
    }
}

/// Task information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Task {
    /// Domain executing the task
    pub domain_id:    DomainId,
    /// Task reservations
    pub reservations: TaskReservation,
    /// Task specification
    pub spec:         TaskSpec,
    /// Security keys and associateds permissions
    pub security:     TaskSecurity,
}

/// Information about access keys and permissions of a task
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TaskSecurity {
    /// Security settings per secure key
    pub security: HashMap<SecureKey, TaskPermissions>,
    /// Revision number - starts at zero and is incremented at every change of task security
    #[serde(default)]
    pub revision: u64,
}

impl From<CreateTaskSecurity> for TaskSecurity {
    fn from(other: CreateTaskSecurity) -> Self {
        TaskSecurity { security: other,
                       revision: 0, }
    }
}

/// Information about access keys and permissions of a task
pub type CreateTaskSecurity = HashMap<SecureKey, TaskPermissions>;

/// Timed resource reservations for the task (must contain all used resources)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TaskReservation {
    /// Start of the reservation time
    pub from:            Timestamp,
    /// End of the reservation time
    pub to:              Timestamp,
    /// Fixed instances reserved for the task
    pub fixed_instances: HashSet<FixedInstanceId>,
    /// Revision number - starts at zero and change of task reservation increments it
    pub revision:        u64,
}

impl TaskReservation {
    /// Returns true if the current UTC time is within the reservation time
    pub fn contains_now(&self) -> bool {
        self.time_range().contains_now()
    }

    /// Get a time range for this reservation
    pub fn time_range(&self) -> TimeRange {
        TimeRange::new(self.from, self.to)
    }
}

/// Timed resource reservations for the task (must contain all used resources)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CreateTaskReservation {
    /// Start of the reservation time
    pub from:            Timestamp,
    /// End of the reservation time
    pub to:              Timestamp,
    /// Fixed instances reserved for the task
    pub fixed_instances: HashSet<FixedInstanceId>,
}

impl Into<TaskReservation> for CreateTaskReservation {
    fn into(self) -> TaskReservation {
        let Self { from, to, fixed_instances } = self;
        TaskReservation { from,
                          to,
                          fixed_instances,
                          revision: 0 }
    }
}

impl From<CreateTask> for Task {
    fn from(source: CreateTask) -> Self {
        let CreateTask { domain_id,
                         reservations,
                         spec,
                         security,
                         .. } = source;

        Self { domain_id:    domain_id.into(),
               reservations: reservations.into(),
               spec:         spec.into(),
               security:     security.into(), }
    }
}

/// Mixer node specification
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct MixerNode {
    /// Numvber of input channels on the mixer node
    pub input_channels:  usize,
    /// Number of output channels on the mixer node
    pub output_channels: usize,
}

impl MixerNode {
    pub fn validate_source_channels(&self, mask: ChannelMask) -> Result<(), CloudError> {
        let Self { output_channels, .. } = *self;

        let half_output_channels = output_channels / 2;

        if matches!(mask, ChannelMask::Mono(i) if i < output_channels) || matches!(mask, ChannelMask::Stereo(i) if i < half_output_channels)
        {
            Ok(())
        } else {
            Err(ChannelMaskIncompatible { mask:     mask.clone(),
                                          channels: output_channels, })
        }
    }

    pub fn validate_destination_channels(&self, mask: ChannelMask) -> Result<(), CloudError> {
        let Self { input_channels, .. } = *self;

        let half_input_channels = input_channels / 2;

        if matches!(mask, ChannelMask::Mono(i) if i < input_channels) || matches!(mask, ChannelMask::Stereo(i) if i < half_input_channels) {
            Ok(())
        } else {
            Err(ChannelMaskIncompatible { mask:     mask.clone(),
                                          channels: input_channels, })
        }
    }
}

/// Dynamic node specification
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct DynamicInstanceNode {
    /// The manufacturer and name of the processing software
    pub model_id:   ModelId,
    /// Parameter values
    pub parameters: InstanceParameters,
}

impl DynamicInstanceNode {
    pub fn validate_source_channels(&self, mask: ChannelMask, model: &Model) -> Result<(), CloudError> {
        let output_channels = model.get_audio_output_channel_count();
        let half_output_channels = output_channels / 2;

        if matches!(mask, ChannelMask::Mono(i) if i < output_channels) || matches!(mask, ChannelMask::Stereo(i) if i < half_output_channels)
        {
            Ok(())
        } else {
            Err(ChannelMaskIncompatible { mask:     mask.clone(),
                                          channels: output_channels, })
        }
    }

    pub fn validate_destination_channels(&self, mask: ChannelMask, model: &Model) -> Result<(), CloudError> {
        let input_channels = model.get_audio_input_channel_count();
        let half_input_channels = input_channels / 2;

        if matches!(mask, ChannelMask::Mono(i) if i < input_channels) || matches!(mask, ChannelMask::Stereo(i) if i < half_input_channels) {
            Ok(())
        } else {
            Err(ChannelMaskIncompatible { mask:     mask.clone(),
                                          channels: input_channels, })
        }
    }
}

/// Fixed instance node specification
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct FixedInstanceNode {
    /// The manufacturer, name and instance identifier of the hardware device doing the processing
    pub instance_id: FixedInstanceId,
    /// parameters
    pub parameters:  InstanceParameters,
    /// Dry-wet percentage
    ///
    /// only applicable for instances with same number of inputs and outputs,
    /// having 1 or 2 channels.
    pub wet:         f64,
}

impl FixedInstanceNode {
    pub fn validate_source_channels(&self, mask: ChannelMask, model: &Model) -> Result<(), CloudError> {
        let input_channels = model.get_audio_input_channel_count();
        let half_input_channels = input_channels / 2;

        if matches!(mask, ChannelMask::Mono(i) if i < input_channels) || matches!(mask, ChannelMask::Stereo(i) if i < half_input_channels) {
            Ok(())
        } else {
            Err(ChannelMaskIncompatible { mask:     mask.clone(),
                                          channels: input_channels, })
        }
    }

    pub fn validate_destination_channels(&self, mask: ChannelMask, model: &Model) -> Result<(), CloudError> {
        let output_channels = model.get_audio_output_channel_count();
        let half_output_channels = output_channels / 2;

        if matches!(mask, ChannelMask::Mono(i) if i < output_channels) || matches!(mask, ChannelMask::Stereo(i) if i < half_output_channels)
        {
            Ok(())
        } else {
            Err(ChannelMaskIncompatible { mask:     mask.clone(),
                                          channels: output_channels, })
        }
    }
}

/// Connection between nodes in a task
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct NodeConnection {
    /// Source node pad
    pub from:          OutputPadId,
    /// Destination node pad
    pub to:            InputPadId,
    /// Source channel mask
    pub from_channels: ChannelMask,
    /// Destination channel mask
    pub to_channels:   ChannelMask,
    /// Volume adjustment as a factor
    pub volume:        f64,
    /// Panning adjustment
    ///
    /// Zero is centered, -1 is fully left, 1 is fully right
    pub pan:           f64,
}

pub type InstanceParameters = serde_json::Value;
pub type InstanceReports = serde_json::Value;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct ConnectionValues {
    pub volume: Option<f64>,
    pub pan:    Option<f64>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, IsVariant, Unwrap, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MixerChannels {
    Mono(usize),
    Stereo(usize),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, IsVariant, Unwrap, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ChannelMask {
    Mono(usize),
    Stereo(usize),
}

impl MixerChannels {
    pub fn to_count_and_index(self) -> (usize, usize) {
        match self {
            MixerChannels::Mono(ch) => (1, ch),
            MixerChannels::Stereo(ch) => (2, ch),
        }
    }

    pub fn is_subset_of(self, range: Range<usize>) -> bool {
        match self {
            MixerChannels::Mono(ch) => range.contains(&ch),
            MixerChannels::Stereo(ch) => range.contains(&ch) && range.contains(&(ch + 1)),
        }
    }
}

impl ChannelMask {
    pub fn to_count_and_index(self) -> (usize, usize) {
        match self {
            Self::Mono(ch) => (1, ch),
            Self::Stereo(ch) => (2, ch),
        }
    }

    pub fn is_subset_of(self, range: Range<usize>) -> bool {
        match self {
            Self::Mono(ch) => range.contains(&ch),
            Self::Stereo(ch) => range.contains(&ch) && range.contains(&(ch + 1)),
        }
    }
}

/// A pad that can receive connections on a node inside a task
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, IsVariant, Unwrap, Hash, Eq, PartialOrd, Ord, JsonSchema)]
pub enum InputPadId {
    /// Mixer node input
    #[serde(rename = "mixer")]
    MixerInput(MixerNodeId),

    /// Fixed instance node input
    #[serde(rename = "fixed")]
    FixedInstanceInput(FixedInstanceNodeId),

    /// Dynamic instance node input
    #[serde(rename = "dynamic")]
    DynamicInstanceInput(DynamicInstanceNodeId),
}

impl InputPadId {
    pub fn references(&self, node_id: &TaskNodeId) -> bool {
        match (self, node_id) {
            (Self::MixerInput(mixer_id), TaskNodeId::Mixer(ref_mixer_id)) => mixer_id == ref_mixer_id,
            (Self::FixedInstanceInput(fixed_id), TaskNodeId::FixedInstance(ref_fixed_id)) => fixed_id == ref_fixed_id,
            (Self::DynamicInstanceInput(dynamic_id), TaskNodeId::DynamicInstance(ref_dynamic_id)) => dynamic_id == ref_dynamic_id,
            _ => false,
        }
    }
}

/// A pad that can receive connections on a node inside a task
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, IsVariant, Unwrap, Hash, Eq, PartialOrd, Ord, JsonSchema)]
pub enum OutputPadId {
    /// Mixer node output
    #[serde(rename = "mixer")]
    MixerOutput(MixerNodeId),

    /// Fixed instance node output
    #[serde(rename = "fixed")]
    FixedInstanceOutput(FixedInstanceNodeId),

    /// Dynamic instance node output
    #[serde(rename = "dynamic")]
    DynamicInstanceOutput(DynamicInstanceNodeId),

    /// Track node output
    #[serde(rename = "track")]
    TrackOutput(TrackNodeId),
}

impl OutputPadId {
    pub fn references(&self, node_id: &TaskNodeId) -> bool {
        match (self, node_id) {
            (Self::TrackOutput(track_id), TaskNodeId::Track(ref_track_id)) => track_id == ref_track_id,
            (Self::DynamicInstanceOutput(instance_id), TaskNodeId::DynamicInstance(ref_instance_id)) => instance_id == ref_instance_id,
            (Self::FixedInstanceOutput(instance_id), TaskNodeId::FixedInstance(ref_instance_id)) => instance_id == ref_instance_id,
            (Self::MixerOutput(mixer_id), TaskNodeId::Mixer(ref_mixer_id)) => mixer_id == ref_mixer_id,
            _ => false,
        }
    }
}

impl std::fmt::Display for OutputPadId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MixerOutput(id) => write!(f, "mixer:{}", id),
            Self::FixedInstanceOutput(id) => write!(f, "fixed:{}", id),
            Self::DynamicInstanceOutput(id) => write!(f, "dynamic:{}", id),
            Self::TrackOutput(id) => write!(f, "track:{}", id),
        }
    }
}

impl std::fmt::Display for InputPadId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MixerInput(id) => write!(f, "mixer:{}", id),
            Self::FixedInstanceInput(id) => write!(f, "fixed:{}", id),
            Self::DynamicInstanceInput(id) => write!(f, "dynamic:{}", id),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, IsVariant, Unwrap, Hash, Eq, PartialOrd, Ord, JsonSchema)]
pub enum NodePadId {
    /// Mixer node output
    #[serde(rename = "out_mixer")]
    MixerOutput(MixerNodeId),

    /// Fixed instance node output
    #[serde(rename = "out_fixed")]
    FixedInstanceOutput(FixedInstanceNodeId),

    /// Dynamic instance node output
    #[serde(rename = "out_dynamic")]
    DynamicInstanceOutput(DynamicInstanceNodeId),

    /// Track node output
    #[serde(rename = "out_track")]
    TrackOutput(TrackNodeId),

    /// Mixer node input
    #[serde(rename = "in_mixer")]
    MixerInput(MixerNodeId),

    /// Fixed instance node input
    #[serde(rename = "in_fixed")]
    FixedInstanceInput(FixedInstanceNodeId),

    /// Dynamic instance node input
    #[serde(rename = "in_dynamic")]
    DynamicInstanceInput(DynamicInstanceNodeId),
}

impl NodePadId {
    pub fn is_input(&self) -> bool {
        matches!(self,
                 Self::MixerInput(_) | Self::FixedInstanceInput(_) | Self::DynamicInstanceInput(_))
    }

    pub fn is_output(&self) -> bool {
        !self.is_input()
    }

    pub fn as_ref(&self) -> &Self {
        self
    }
}

impl ToString for NodePadId {
    fn to_string(&self) -> String {
        match self {
            NodePadId::MixerOutput(id) => OutputPadId::MixerOutput(id.clone()).to_string(),
            NodePadId::FixedInstanceOutput(id) => OutputPadId::FixedInstanceOutput(id.clone()).to_string(),
            NodePadId::DynamicInstanceOutput(id) => OutputPadId::DynamicInstanceOutput(id.clone()).to_string(),
            NodePadId::TrackOutput(id) => OutputPadId::TrackOutput(id.clone()).to_string(),
            NodePadId::MixerInput(id) => InputPadId::MixerInput(id.clone()).to_string(),
            NodePadId::FixedInstanceInput(id) => InputPadId::FixedInstanceInput(id.clone()).to_string(),
            NodePadId::DynamicInstanceInput(id) => InputPadId::DynamicInstanceInput(id.clone()).to_string(),
        }
    }
}

impl From<OutputPadId> for NodePadId {
    fn from(src: OutputPadId) -> Self {
        match src {
            OutputPadId::MixerOutput(id) => Self::MixerOutput(id),
            OutputPadId::FixedInstanceOutput(id) => Self::FixedInstanceOutput(id),
            OutputPadId::DynamicInstanceOutput(id) => Self::DynamicInstanceOutput(id),
            OutputPadId::TrackOutput(id) => Self::TrackOutput(id),
        }
    }
}

impl From<InputPadId> for NodePadId {
    fn from(src: InputPadId) -> Self {
        match src {
            InputPadId::MixerInput(id) => Self::MixerInput(id),
            InputPadId::FixedInstanceInput(id) => Self::FixedInstanceInput(id),
            InputPadId::DynamicInstanceInput(id) => Self::DynamicInstanceInput(id),
        }
    }
}

/// Task node identifier
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema, From)]
#[serde(rename_all = "snake_case")]
pub enum TaskNodeId {
    Mixer(MixerNodeId),
    FixedInstance(FixedInstanceNodeId),
    DynamicInstance(DynamicInstanceNodeId),
    Track(TrackNodeId),
}

/// Track node specification
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct TrackNode {
    /// Number of channels
    pub channels: MediaChannels,
    /// Media items present on the track
    pub media:    HashMap<TrackMediaId, TrackMedia>,
}

impl TrackNode {
    pub fn validate_source_channels(&self, mask: ChannelMask) -> Result<(), CloudError> {
        let Self { channels, .. } = self;

        let channels = channels.num_channels();
        let half_channels = channels / 2;

        if matches!(mask, ChannelMask::Mono(i) if i < channels) {
            Ok(())
        } else if matches!(mask, ChannelMask::Stereo(i) if i < half_channels) {
            Ok(())
        } else {
            Err(ChannelMaskIncompatible { mask, channels })
        }
    }
}

/// Channel count for media items and track nodes
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MediaChannels {
    /// Single channel
    Mono,
    /// Two channels - left and right
    Stereo,
}

impl MediaChannels {
    pub fn num_channels(&self) -> usize {
        match self {
            MediaChannels::Mono => 1,
            MediaChannels::Stereo => 2,
        }
    }
}

/// Media item specification
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct TrackMedia {
    /// Number of channels
    pub channels:         MediaChannels,
    /// Media format
    pub format:           TrackMediaFormat,
    /// Subset of media that is used
    pub media_segment:    TimeSegment,
    /// Where to place the media in the task timeline
    pub timeline_segment: TimeSegment,
    /// Source media object id
    pub object_id:        MediaObjectId,
}

impl TrackMedia {
    pub fn update(&mut self, update: UpdateTaskTrackMedia) {
        let UpdateTaskTrackMedia { channels,
                                   media_segment,
                                   timeline_segment,
                                   object_id, } = update;

        if let Some(channels) = channels {
            self.channels = channels;
        }

        if let Some(media_segment) = media_segment {
            self.media_segment = media_segment;
        }

        if let Some(timeline_segment) = timeline_segment {
            self.timeline_segment = timeline_segment;
        }

        if let Some(object_id) = object_id {
            self.object_id = object_id;
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct UpdateTaskTrackMedia {
    pub channels:         Option<MediaChannels>,
    pub media_segment:    Option<TimeSegment>,
    pub timeline_segment: Option<TimeSegment>,
    pub object_id:        Option<MediaObjectId>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub enum TrackMediaFormat {
    #[serde(rename = "wave")]
    Wave,
    #[serde(rename = "mp3")]
    Mp3,
    #[serde(rename = "flac")]
    Flac,
    #[serde(rename = "wavpack")]
    WavPack,
}

impl Display for TrackMediaFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match serde_json::to_value(self).unwrap() {
            Value::String(s) => s,
            _ => unreachable!(),
        };
        f.write_str(&s)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct TimeSegment {
    pub start:  f64,
    pub length: f64,
}

impl TimeSegment {
    pub fn end(&self) -> f64 {
        self.start + self.length
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, JsonSchema)]
pub struct TaskPermissions {
    pub structure:  bool,
    pub media:      bool,
    pub parameters: bool,
    pub transport:  bool,
    pub audio:      bool,
}

impl TaskPermissions {
    pub const fn empty() -> Self {
        Self { structure:  false,
               media:      false,
               parameters: false,
               transport:  false,
               audio:      false, }
    }

    pub fn can(&self, other: TaskPermissions) -> bool {
        if !self.structure && other.structure {
            return false;
        }
        if !self.media && other.media {
            return false;
        }
        if !self.parameters && other.parameters {
            return false;
        }
        if !self.transport && other.transport {
            return false;
        }
        if !self.audio && other.audio {
            return false;
        }

        true
    }

    pub fn can_audio(&self) -> bool {
        self.audio
    }

    pub fn full() -> Self {
        TaskPermissions { structure:  true,
                          media:      true,
                          parameters: true,
                          transport:  true,
                          audio:      true, }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskEvent {
    PlayState {
        current:           Timestamped<TaskPlayState>,
        desired:           Timestamped<DesiredTaskPlayState>,
        waiting_instances: HashSet<FixedInstanceId>,
        waiting_media:     HashSet<AppMediaObjectId>,
    },
    StreamingPacket {
        packet: StreamingPacket,
    },
    Deleted,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct StreamingPacket {
    pub play_id:           PlayId,
    pub created_at:        Timestamp,
    pub audio:             Vec<DiffStamped<CompressedAudio>>,
    pub instance_metering: HashMap<FixedInstanceId, Vec<DiffStamped<serde_json::Value>>>,
    pub pad_metering:      HashMap<NodePadId, Vec<DiffStamped<PadMetering>>>,
    pub timeline_pos:      f64,
    pub streaming_pos:     u64,
    pub serial:            u64,
}

impl Default for StreamingPacket {
    fn default() -> Self {
        Self { play_id:           { PlayId::new(Default::default()) },
               audio:             { Default::default() },
               instance_metering: { Default::default() },
               pad_metering:      { Default::default() },
               created_at:        { now() },
               timeline_pos:      { 0.0 },
               streaming_pos:     { 0 },
               serial:            { 0 }, }
    }
}

impl StreamingPacket {
    pub fn next_of(packet: &StreamingPacket) -> Self {
        let mut rv = Self::default();
        rv.serial = packet.serial + 1;
        rv.play_id = packet.play_id.clone();
        rv.streaming_pos = packet.streaming_pos + packet.audio.iter().map(|audio| audio.value().num_samples).sum::<usize>() as u64;

        rv
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct PadMetering {
    pub volume: Vec<f64>,
}
