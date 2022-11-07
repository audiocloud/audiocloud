/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

//! Various IDs and wrappers

use std::fmt::Formatter;
use std::marker::PhantomData;
use std::str::FromStr;

use derive_more::{Constructor, Deref, Display, From, FromStr, IsVariant};
use once_cell::sync::OnceCell;
use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::cloud::CloudError;
use crate::{InputPadId, OutputPadId};

/// Id of a fixed instance
#[derive(Clone, Debug, Eq, PartialEq, Hash, Display, Constructor)]
#[display(fmt = "{manufacturer}/{name}/{instance}")]
pub struct FixedInstanceId {
    /// manufacturer name, may not contain ':' or whitespace
    pub manufacturer: String,
    /// product name, may not contain ':' or whitespace
    pub name:         String,
    /// unique instance name (given the same manufacturer and product name), may not contain ':' or whitespace
    pub instance:     String,
}

impl FixedInstanceId {
    pub fn driver_event_subject(&self) -> String {
        format!("ac.inst.{}.{}.{}.evts", &self.manufacturer, &self.name, &self.instance)
    }
}

impl FixedInstanceId {
    pub fn model_id(&self) -> ModelId {
        ModelId { manufacturer: self.manufacturer.to_string(),
                  name:         self.name.to_string(), }
    }

    pub fn from_model_id(model_id: ModelId, instance: String) -> Self {
        let ModelId { manufacturer, name } = model_id;
        Self::new(manufacturer, name, instance)
    }
}

impl<'de> Deserialize<'de> for FixedInstanceId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let err = |msg| serde::de::Error::custom(msg);

        let s = String::deserialize(deserializer)?;
        let mut s = s.split('/');
        let manufacturer = s.next().ok_or(err("expected manufacturer"))?;
        let name = s.next().ok_or(err("expected manufacturer"))?;
        let instance = s.next().ok_or(err("expected instance"))?;

        Ok(Self { manufacturer: manufacturer.to_string(),
                  name:         name.to_string(),
                  instance:     instance.to_string(), })
    }
}

impl Serialize for FixedInstanceId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&format!("{}/{}/{}", &self.manufacturer, &self.name, &self.instance))
    }
}

/// Id of a product that may be instanced, either dynamically (software) or in fixed instances (hardware)
#[derive(Clone, Debug, Display, Eq, PartialEq, Hash, Constructor)]
#[display(fmt = "{manufacturer}:{name}")]
pub struct ModelId {
    /// manufacturer name, may not contain ':' or whitespace
    pub manufacturer: String,
    /// product name, may not contain ':' or whitespace
    pub name:         String,
}

impl ModelId {
    pub fn instance(self, instance: String) -> FixedInstanceId {
        FixedInstanceId::from_model_id(self, instance)
    }
}

impl From<(String, String)> for ModelId {
    fn from((manufacturer, name): (String, String)) -> Self {
        Self::new(manufacturer, name)
    }
}

impl<'de> Deserialize<'de> for ModelId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_str(Tuple2Visitor::new())
    }
}

impl Serialize for ModelId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// What kind of filter
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FilterId {
    HighPass,
    Low,
    LowMid,
    Mid,
    HighMid,
    High,
    LowPass,
    BandPass,
    Dynamics,
    DeEsser,
}

struct Tuple2Visitor<K, V, T>(PhantomData<K>, PhantomData<V>, PhantomData<T>);

impl<K, V, T> Tuple2Visitor<K, V, T> {
    pub fn new() -> Self {
        Self(PhantomData, PhantomData, PhantomData)
    }
}

impl<'de, K, V, T> serde::de::Visitor<'de> for Tuple2Visitor<K, V, T>
    where T: From<(K, V)>,
          K: From<String>,
          V: From<String>
{
    type Value = T;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Expected string of format string/string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where E: serde::de::Error
    {
        let mut split = v.split(':');
        let first = split.next().ok_or(E::custom("could not extract first string"))?;
        let second = split.next().ok_or(E::custom("could not extract second string"))?;

        Ok(T::from((K::from(first.to_string()), V::from(second.to_string()))))
    }
}

/// Id of a media track node in a task
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct TrackNodeId(String);

impl TrackNodeId {
    pub fn source(self) -> OutputPadId {
        OutputPadId::TrackOutput(self)
    }
}

/// Media item on a track
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct TrackMediaId(String);

/// Id of a mixer node in a task
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct MixerNodeId(String);

impl MixerNodeId {
    pub fn input_flow(self) -> InputPadId {
        InputPadId::MixerInput(self)
    }
    pub fn output_flow(self) -> OutputPadId {
        OutputPadId::MixerOutput(self)
    }
}

/// Id of a dynamic instance node in a task
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct DynamicInstanceNodeId(String);

impl DynamicInstanceNodeId {
    pub fn input_flow(self) -> InputPadId {
        InputPadId::DynamicInstanceInput(self)
    }
    pub fn output_flow(self) -> OutputPadId {
        OutputPadId::DynamicInstanceOutput(self)
    }
}

/// Id of a fixed instance node in a task
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct FixedInstanceNodeId(String);

impl FixedInstanceNodeId {
    pub fn input_flow(self) -> InputPadId {
        InputPadId::FixedInstanceInput(self)
    }
    pub fn output_flow(self) -> OutputPadId {
        OutputPadId::FixedInstanceOutput(self)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct NodeConnectionId(String);

/// Id of an app registered with the cloud
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From)]
#[repr(transparent)]
pub struct AppId(String);

impl AppId {
    pub fn is_admin(&self) -> bool {
        self.0 == "admin"
    }

    pub fn admin() -> AppId {
        AppId("admin".to_string())
    }

    pub fn test() -> AppId {
        AppId("test".to_string())
    }
}

/// Id of a task
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct TaskId(String);

/// Id of a request
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct RequestId(String);

/// Id of an audio engine (there may be more than one in a domain)
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct EngineId(String);

/// Id of an fixed instance driver (there may be more than one in a domain)
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct InstanceDriverId(String);

impl EngineId {
    pub fn engine_command_subject(&self) -> String {
        format!("ac.engn.{}.cmds", self)
    }

    pub fn engine_event_subject(&self) -> String {
        format!("ac.engn.{}.evts", self)
    }
}

/// Id of a socket (there may be more than one streaming connection per task in a domain)
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct SocketId(String);

/// A client identifier to group sockets belonging to the same client
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct ClientId(String);

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Constructor, Hash, JsonSchema)]
#[display(fmt = "{client_id}.{socket_id}")]
pub struct ClientSocketId {
    pub client_id: ClientId,
    pub socket_id: SocketId,
}

impl TaskId {
    pub fn validate(self) -> Result<Self, CloudError> {
        static VALIDATION: OnceCell<Regex> = OnceCell::new();

        VALIDATION.get_or_init(|| Regex::new(r"^[a-zA-Z0-9_\-]+$").unwrap())
                  .find(&self.0)
                  .ok_or_else(|| CloudError::InvalidAppTaskId { task_id: self.to_string() })?;

        Ok(self)
    }
}

/// A task by an app
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Display, Constructor, Hash, From)]
#[display(fmt = "{app_id}:{task_id}")]
pub struct AppTaskId {
    /// App registering the task
    pub app_id:  AppId,
    /// Task id
    pub task_id: TaskId,
}

impl FromStr for AppTaskId {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
    }
}

impl<'de> Deserialize<'de> for AppTaskId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_str(Tuple2Visitor::new())
    }
}

impl Serialize for AppTaskId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// Id of a media object
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct MediaObjectId(String);

impl MediaObjectId {
    pub fn for_app(self, app_id: AppId) -> AppMediaObjectId {
        AppMediaObjectId { app_id, media_id: self }
    }

    pub fn validate(self) -> Result<Self, CloudError> {
        static VALIDATION: OnceCell<Regex> = OnceCell::new();

        VALIDATION.get_or_init(|| Regex::new(r"^[a-zA-Z0-9_\-]+$").unwrap())
                  .find(&self.0)
                  .ok_or_else(|| CloudError::InvalidAppMediaObjectId { object_id: self.to_string(), })?;

        Ok(self)
    }
}

/// Media object owned by an app
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Display, Constructor, Hash)]
#[display(fmt = "{app_id}:{media_id}")]
pub struct AppMediaObjectId {
    /// App owner
    pub app_id:   AppId,
    /// Media object Id
    pub media_id: MediaObjectId,
}

impl From<(AppId, MediaObjectId)> for AppMediaObjectId {
    fn from((app_id, media_id): (AppId, MediaObjectId)) -> Self {
        Self::new(app_id, media_id)
    }
}

impl FromStr for AppMediaObjectId {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
    }
}

impl Serialize for AppMediaObjectId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for AppMediaObjectId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_str(Tuple2Visitor::new())
    }
}

impl AppMediaObjectId {
    pub fn slashed(&self) -> String {
        format!("{}/{}", &self.app_id, &self.media_id)
    }
}

/// A password for direct task control on the domain
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct SecureKey(String);

/// Domain Id
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct DomainId(String);

/// Parameter Id within a model
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct ParameterId(String);

impl From<&str> for ParameterId {
    fn from(s: &str) -> Self {
        Self::new(s.to_string())
    }
}

/// Report Id within a model
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct ReportId(String);

impl From<&str> for ReportId {
    fn from(s: &str) -> Self {
        Self::new(s.to_string())
    }
}

#[macro_export]
macro_rules! json_schema_new_type {
    ($($i:ident), *) => {
        $(
            impl schemars::JsonSchema for $i {
                fn schema_name() -> String {
                    std::stringify!($i).to_string()
                }

                fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                        schemars::schema::SchemaObject { instance_type: Some(schemars::schema::InstanceType::String.into()),
                                                         ..Default::default() }.into()
                }
            }
        )*
    }
}

json_schema_new_type!(AppId,
                      AppTaskId,
                      MediaObjectId,
                      AppMediaObjectId,
                      FixedInstanceId,
                      TrackNodeId,
                      TrackMediaId,
                      MixerNodeId,
                      DynamicInstanceNodeId,
                      FixedInstanceNodeId,
                      SecureKey,
                      DomainId,
                      ParameterId,
                      ReportId,
                      ModelId,
                      TaskId,
                      ClientId,
                      SocketId,
                      RequestId,
                      EngineId,
                      InstanceDriverId);
