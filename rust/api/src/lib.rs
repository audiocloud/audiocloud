use std::marker::PhantomData;

pub use chrono::{DateTime, Utc};
use schemars::schema::RootSchema;
use schemars_zod::merge_schemas;

pub mod instance;
pub mod media;
pub mod rt;
pub mod task;
pub mod user;

pub type Timestamp = DateTime<Utc>;

pub mod time {
  pub use super::{DateTime, Timestamp, Utc};

  pub fn new() -> Timestamp {
    Utc::now()
  }
}

#[derive(Clone)]
pub struct Request<Req, Res> {
  pub phantom_req: PhantomData<Req>,
  pub phantom_res: PhantomData<Res>,
  pub subject:     String,
}

impl<Req, Res> Request<Req, Res> {
  pub fn new(subject: impl ToString) -> Self {
    Self { phantom_req: Default::default(),
           phantom_res: Default::default(),
           subject:     subject.to_string(), }
  }
}

#[derive(Clone)]
pub struct Events<Evt> {
  pub phantom_evt: PhantomData<Evt>,
  pub subject:     String,
}

impl<Evt> Events<Evt> {
  pub fn new(subject: impl ToString) -> Self {
    Self { phantom_evt: Default::default(),
           subject:     subject.to_string(), }
  }
}

#[derive(Clone)]
pub struct BucketKey<Key, Content> {
  pub phantom_key:     PhantomData<Key>,
  pub phantom_content: PhantomData<Content>,
  pub key:             String,
}

impl<Key, Content> BucketKey<Key, Content> {
  pub fn all() -> Self {
    BucketKey { phantom_key:     Default::default(),
                phantom_content: Default::default(),
                key:             "*".to_string(), }
  }

  pub fn new<T: ToString>(key: &T) -> Self where T: ?Sized {
    Self { phantom_key:     Default::default(),
           phantom_content: Default::default(),
           key:             key.to_string(), }
  }
}

impl<Key, Content> From<String> for BucketKey<Key, Content> {
  fn from(value: String) -> Self {
    BucketKey { phantom_key:     Default::default(),
                phantom_content: Default::default(),
                key:             value, }
  }
}

pub struct BucketName<Content> {
  pub phantom_content: PhantomData<Content>,
  pub name:            &'static str,
}

impl<Content> BucketName<Content> {
  pub const fn new(name: &'static str) -> Self {
    Self { phantom_content: PhantomData,
           name }
  }
}

pub fn schema() -> RootSchema {
  merge_schemas([instance::schema(), media::schema(), rt::schema(), task::schema(), user::schema()].into_iter())
}
