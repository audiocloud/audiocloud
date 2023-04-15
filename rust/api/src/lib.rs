use std::marker::PhantomData;

pub use chrono::{DateTime, Utc};

pub mod instance;
pub mod media;
pub mod task;
pub mod ws;

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
pub struct BucketKey<Ctrl> {
  pub phantom_ctrl: PhantomData<Ctrl>,
  pub key:          String,
}

pub trait IntoBucketKey<T> {
  fn to_bucket_key(&self) -> BucketKey<T>;
}

impl<T, Ctrl> IntoBucketKey<Ctrl> for T where T: ToString
{
  fn to_bucket_key(&self) -> BucketKey<Ctrl> {
    BucketKey::new(self)
  }
}

impl<Ctrl> BucketKey<Ctrl> {
  pub fn all() -> Self {
    BucketKey { phantom_ctrl: Default::default(),
                key:          "*".to_string(), }
  }

  pub fn new<T: ToString>(key: &T) -> Self {
    Self { phantom_ctrl: Default::default(),
           key:          key.to_string(), }
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
