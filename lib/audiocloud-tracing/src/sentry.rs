/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */

use std::any::Any;
use std::borrow::Cow;

use anyhow::anyhow;
use sentry_tracing::SentryLayer;
use tracing::Subscriber;
use tracing_subscriber::registry::LookupSpan;

use crate::O11yOpts;

pub fn sentry_tracing_layer<S>(opts: &O11yOpts, _service_name: &str, instance_name: &str) -> anyhow::Result<(SentryLayer<S>, Box<dyn Any>)>
    where S: Subscriber + for<'a> LookupSpan<'a>
{
    let dsn = opts.sentry_dsn.as_ref().ok_or_else(|| anyhow!("Sentry DSN not set"))?;

    let guard = sentry::init((dsn.as_str(),
                              sentry::ClientOptions { // Set this a to lower value in production
                                                      traces_sample_rate: 1.0,
                                                      auto_session_tracking: true,
                                                      server_name: Some(Cow::from(instance_name.to_owned())),
                                                      ..sentry::ClientOptions::default() }));

    let layer = sentry_tracing::layer();

    Ok((layer, Box::new(guard)))
}
