use std::fs;

use api::driver;
use schemars::schema_for;
use schemars_zod::{convert, merge_schemas};

fn main() {
  let schema = merge_schemas([schema_for!(driver::BinaryPosition),
                              schema_for!(driver::InstanceDriverConfig),
                              schema_for!(driver::InstanceDriverEvent),
                              schema_for!(driver::Remap),
                              schema_for!(driver::Rescale),
                              schema_for!(driver::Clamp),
                              schema_for!(driver::UsbHidDriverConfig),
                              schema_for!(driver::UsbHidParameterConfig),
                              schema_for!(driver::UsbHidParameterPage),
                              schema_for!(driver::UsbHidReportConfig),
                              schema_for!(driver::UsbHidReportPage),
                              schema_for!(driver::SerialDriverConfig),
                              schema_for!(driver::SerialFlowControl),
                              schema_for!(driver::SerialParameterConfig),
                              schema_for!(driver::SerialReportConfig),
                              schema_for!(driver::SerialReportMatcher),
                              schema_for!(driver::SerialReportValueInterpretation),
                              schema_for!(driver::SerialRequestTimer),
                              schema_for!(driver::SetInstanceParameterRequest),
                              schema_for!(driver::SetInstancePowerRequest),
                              schema_for!(driver::SetInstancePlayRequest),
                              schema_for!(driver::WsDriverEvent),
                              schema_for!(driver::WsDriverRequest)].into_iter());

  let mut content = String::new();
  content.push_str("import { z } from \"zod\";\n");
  content.push_str(&convert(schema).into_values().collect::<Vec<_>>().join("\n"));

  fs::write("ts/domain_client/src/types.ts", content).expect("success");
}
