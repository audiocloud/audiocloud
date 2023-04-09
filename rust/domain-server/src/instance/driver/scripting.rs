use std::num::NonZeroUsize;
use std::thread::{sleep, spawn, JoinHandle};
use std::time::{Duration, Instant};

use boa_engine::prelude::*;
use boa_engine::vm::CodeBlock;
use gc::Gc;
use lru::LruCache;
use serde_json::Value;
use tokio::sync::{mpsc, oneshot};
use tokio::time::timeout;

pub enum ScriptingEngineCommand {
  Eval(String, Value, oneshot::Sender<Value>),
}

#[derive(Clone)]
pub struct ScriptingEngine(mpsc::Sender<ScriptingEngineCommand>);

impl ScriptingEngine {
  pub async fn execute(&self, script: String, args: Value) -> Value {
    let (tx, rx) = oneshot::channel();
    let _ = self.0.send(ScriptingEngineCommand::Eval(script, args, tx)).await;
    rx.await.unwrap_or_default()
  }

  pub async fn execute_timeout(&self, script: String, args: Value, duration: Duration) -> Value {
    let (tx, rx) = oneshot::channel();
    let _ = self.0.send(ScriptingEngineCommand::Eval(script, args, tx)).await;
    match timeout(duration, rx).await {
      | Ok(rv) => rv.unwrap_or_default(),
      | Err(_) => Value::Null, // timed out
    }
  }

  pub fn execute_sync(&self, script: String, env: Value) -> Value {
    let (tx, rx) = oneshot::channel();
    let _ = self.0.blocking_send(ScriptingEngineCommand::Eval(script, env, tx));
    rx.blocking_recv().unwrap_or_default()
  }

  pub fn execute_sync_timeout(&self, script: String, env: Value, duration: Duration) -> Value {
    let (tx, mut rx) = oneshot::channel();
    let start = Instant::now();
    let _ = self.0.blocking_send(ScriptingEngineCommand::Eval(script, env, tx));

    while start.elapsed() < duration {
      if let Ok(rv) = rx.try_recv() {
        return rv;
      }

      sleep(Duration::from_millis(1));
    }

    Value::Null // timed out
  }
}

pub fn run_scripts(mut rx_cmds: mpsc::Receiver<ScriptingEngineCommand>) {
  let mut context = Context::builder().build();
  let mut lru = LruCache::<String, Gc<CodeBlock>>::new(NonZeroUsize::new(0x1000).unwrap());

  let _ = context.eval(include_str!("stdlib.js"));

  while let Some(cmd) = rx_cmds.blocking_recv() {
    match cmd {
      | ScriptingEngineCommand::Eval(script, env, tx) => {
        let script = lru.get_or_insert(script.clone(), || {
                          let parsed = context.parse(&script).unwrap();
                          context.compile(&parsed).unwrap()
                        });

        let global = context.global_object().to_owned();
        if let Some(object) = env.as_object() {
          for (key, value) in object {
            let Ok(value) = JsValue::from_json(&value, &mut context) else { continue; };
            let _ = global.set(key.as_str(), value, false, &mut context);
          }
        }

        let rv = context.execute(script.clone())
                        .map(|value| if value.is_undefined() { JsValue::Null } else { value })
                        .unwrap_or_else(|_| JsValue::Null);

        let rv = rv.to_json(&mut context).unwrap_or_default();
        let _ = tx.send(rv);
      }
    }
  }
}

pub fn new_scripting_engine() -> (ScriptingEngine, JoinHandle<()>) {
  let (tx, rx) = mpsc::channel(0x100);
  let engine = ScriptingEngine(tx);
  let handle = spawn(move || run_scripts(rx));

  (engine, handle)
}
