use std::fs;

use schemars_zod::convert;

fn main() {
  let schemas = [("types", api::schema())];

  for (name, schema) in schemas.into_iter() {
    println!("generating {name}");
    let mut content = String::new();
    content.push_str("import memoizeOne from \"memoize-one\";\n");
    content.push_str("import { z } from \"zod\";\n");

    content.push_str(&convert(schema).into_values().collect::<Vec<_>>().join("\n"));

    fs::write(format!("ts/domain_client/src/{name}.ts"), content).expect("success");
  }

  println!("done");
}
