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

impl ScriptingEngine {
  pub fn new() -> anyhow::Result<Self> {
    let mut context = Context::builder().build();
    context.eval("function gainFactorToDb(gainFactor) { return 20 * Math.log10(gainFactor); }")
           .map_err(|e| anyhow!("failed to register gainFactorToDb"))?;

    Ok(Self { context })
  }

  pub fn compile(&mut self, code: &str) -> anyhow::Result<Script> {
    let parsed = self.context.parse(code).map_err(|e| anyhow!("failed to parse '{code}': {e:?}"))?;

    self.context
        .compile(&parsed)
        .map_err(|e| anyhow!("failed to compile '{code}': {e:?}"))
        .map(Script)
  }

  pub fn eval_f64_to_f64(&mut self, script: &Script, value: f64) -> f64 {
    self.context.eval(format!("value = {value};")).unwrap();

    let result = self.context.execute(script.0.clone()).unwrap_or_default();
    result.to_number(&mut self.context).unwrap_or_default()
  }

  pub fn eval_f64_to_string(&mut self, script: &Script, value: f64) -> String {
    self.context.eval(format!("value = {value};")).unwrap();

    let result = self.context.execute(script.0.clone()).unwrap_or_default();
    result.to_string(&mut self.context)
          .map(|jss| jss.to_string())
          .unwrap_or_else(|_| "".to_string())
  }
}
