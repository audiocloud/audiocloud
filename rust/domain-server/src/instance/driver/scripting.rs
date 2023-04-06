use std::ops::Deref;

use anyhow::anyhow;
use boa_engine::prelude::*;
use boa_engine::vm::CodeBlock;
use gc::Gc;

pub struct ScriptingEngine {
  context: Context,
}

unsafe impl Send for ScriptingEngine {}

#[derive(Debug, Clone)]
pub struct Script(Gc<CodeBlock>);

unsafe impl Send for Script {}

impl Deref for Script {
  type Target = Gc<CodeBlock>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

pub mod state {
  const STOPPED: u32 = 0;
  const PLAYING: u32 = 1;
  const REWINDING: u32 = 2;
  const PREPARING_TO_STOP: u32 = 3;
  const PREPARING_TO_PLAY: u32 = 4;
  const BUSY: u32 = 5;
}

impl ScriptingEngine {
  pub fn new() -> anyhow::Result<Self> {
    let mut context = Context::builder().build();

    context.eval("function gainFactorToDb(gainFactor) { return 20 * Math.log10(gainFactor); }")
           .map_err(|e| anyhow!("failed to register gainFactorToDb: {e:?}"))?;

    context.eval("function dbToGainFactor(db) { return Math.pow(10, db / 20); }")
           .map_err(|e| anyhow!("failed to register gainFactorToDb: {e:?}"))?;

    Ok(Self { context })
  }

  pub fn compile(&mut self, code: &str) -> anyhow::Result<Script> {
    let parsed = self.context.parse(code).map_err(|e| anyhow!("failed to parse '{code}': {e:?}"))?;

    self.context
        .compile(&parsed)
        .map_err(|e| anyhow!("failed to compile '{code}': {e:?}"))
        .map(Script)
  }

  pub fn execute(&mut self, script: &Script, value: JsValue) -> JsValue {
    let global = self.context.global_object().to_owned();
    global.set("value", value, false, &mut self.context).expect("failed to set value");
    self.context.execute(script.0.clone()).unwrap_or_default()
  }

  pub fn execute_with_env(&mut self, script: &Script, values: impl Iterator<Item = (String, JsValue)>) -> JsValue {
    let global = self.context.global_object().to_owned();
    for (key, value) in values {
      global.set(key, value, false, &mut self.context).expect("failed to set value");
    }
    self.context.execute(script.0.clone()).unwrap_or_default()
  }

  pub fn get_position(&mut self) -> Option<f64> {
    let global = self.context.global_object().to_owned();
    global.get("position", &mut self.context).ok().and_then(|v| v.as_number())
  }

  pub fn get_state(&mut self) -> Option<u32> {
    let global = self.context.global_object().to_owned();
    global.get("state", &mut self.context)
          .ok()
          .and_then(|v| v.as_number().map(|v| v as u32))
  }

  pub fn convert_to_f64(&mut self, value: JsValue) -> f64 {
    value.to_number(&mut self.context).unwrap_or_default()
  }

  pub fn convert_to_string(&mut self, value: JsValue) -> String {
    value.to_string(&mut self.context).unwrap_or_default().to_string()
  }
}
