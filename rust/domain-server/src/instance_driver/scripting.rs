use std::collections::HashMap;

pub struct ScriptingEngine {
  engine: rhai::Engine,
  scripts: HashMap<String, ()>
}

impl ScriptingEngine {
  pub fn new() -> Self {
    let mut engine = rhai::Engine::new();
    engine.register_fn("")

    Self { engine }
  }

  pub fn process(&mut self, value: f64) -> f64 {
    self.engine.eval("value = ").unwrap()
  }
}
