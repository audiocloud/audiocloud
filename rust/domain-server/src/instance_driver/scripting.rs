use anyhow::anyhow;
use boa_engine::prelude::*;

pub struct ScriptingEngine {
  context: Context,
}

unsafe impl Send for ScriptingEngine {}

impl ScriptingEngine {
  pub fn new() -> anyhow::Result<Self> {
    let mut context = Context::builder().build();
    context.parse("function gainFactorToDb(gainFactor) { return 20 * Math.log10(gainFactor); }")
           .map_err(|e| anyhow!(e))?;

    let context = context;

    Ok(Self { context })
  }

  pub fn process(&mut self, script: &str, value: f64) -> f64 {
    self.context.eval(format!("value = {};", value)).unwrap();
    let result = self.context.eval(script).unwrap_or_default();
    result.to_number(&mut self.context).unwrap_or_default()
  }
}
