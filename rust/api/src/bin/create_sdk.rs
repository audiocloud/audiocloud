use std::fs;

use schemars_zod::convert;

use api::instance_driver;

fn main() {
  let schemas = [("instance", api::instance::schema()),
                 ("media", api::media::schema()),
                 ("task", api::task::schema())];

  for (name, schema) in schemas.into_iter() {
    println!("generating {name}");
    let mut content = String::new();
    content.push_str("import { z } from \"zod\";\n");
    content.push_str(&convert(schema).into_values().collect::<Vec<_>>().join("\n"));

    fs::write(format!("ts/domain_client/src/{name}.ts"), content).expect("success");
  }

  println!("done");
}
