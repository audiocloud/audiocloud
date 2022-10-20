use anyhow::anyhow;

use trim_margin::MarginTrimmable;

use crate::netio::power_pdu_4c::{NetioPowerOutputAction, NetioPowerRequest, NetioPowerResponse, PowerAction};

#[test]
fn deserialize_captured_response() -> anyhow::Result<()> {
    let raw_json = r#"|{
                      |	"Agent":	{
                      |		"Model":	"PowerPDU 4C",
                      |		"Version":	"3.4.2",
                      |		"JSONVer":	"2.1",
                      |		"DeviceName":	"rack1-up-right",
                      |		"VendorID":	0,
                      |		"OemID":	0,
                      |		"SerialNumber":	"24:A4:2C:39:3F:05",
                      |		"Uptime":	4521231,
                      |		"Time":	"2022-10-11T15:11:46+01:00",
                      |		"NumOutputs":	4
                      |	},
                      |	"GlobalMeasure":	{
                      |		"Voltage":	222.4,
                      |		"Frequency":	50.0,
                      |		"TotalCurrent":	0,
                      |		"OverallPowerFactor":	0.00,
                      |		"TotalLoad":	0,
                      |		"TotalEnergy":	233220,
                      |		"EnergyStart":	"1970-01-01T01:00:00+01:00"
                      |	},
                      |	"Outputs":	[{
                      |			"ID":	1,
                      |			"Name":	"output_1",
                      |			"State":	1,
                      |			"Action":	6,
                      |			"Delay":	5000,
                      |			"Current":	0,
                      |			"PowerFactor":	0.00,
                      |			"Load":	0,
                      |			"Energy":	233203
                      |		}, {
                      |			"ID":	2,
                      |			"Name":	"mac_mini",
                      |			"State":	1,
                      |			"Action":	6,
                      |			"Delay":	5000,
                      |			"Current":	0,
                      |			"PowerFactor":	0.00,
                      |			"Load":	0,
                      |			"Energy":	0
                      |		}, {
                      |			"ID":	3,
                      |			"Name":	"output_3",
                      |			"State":	1,
                      |			"Action":	6,
                      |			"Delay":	5000,
                      |			"Current":	0,
                      |			"PowerFactor":	0.00,
                      |			"Load":	0,
                      |			"Energy":	8
                      |		}, {
                      |			"ID":	4,
                      |			"Name":	"output_4",
                      |			"State":	1,
                      |			"Action":	6,
                      |			"Delay":	5000,
                      |			"Current":	0,
                      |			"PowerFactor":	0.00,
                      |			"Load":	0,
                      |			"Energy":	8
                      |		}]
                      |}"#.trim_margin()
                          .ok_or_else(|| anyhow!("Failed to trim margin from captured JSON"))?;

    let _netio_response =
        serde_json::from_str::<NetioPowerResponse>(raw_json.as_str()).map_err(|error| {
                                                                         anyhow!("Captured response should deserialize: {error}")
                                                                     })?;

    // TODO: assert values

    Ok(())
}

#[test]
fn serialize_request() -> anyhow::Result<()> {
    // TODO: add test when implementation exists

    let request = NetioPowerRequest { outputs: vec![NetioPowerOutputAction { id:     { 0 },
                                                                             action: { PowerAction::Off }, },
                                                    NetioPowerOutputAction { id:     { 1 },
                                                                             action: { PowerAction::On }, },], };

    let json = serde_json::to_string_pretty(&request).expect("Request should serialize");

    assert_eq!(json,
               r#"|{
                  |  "Outputs": [
                  |    {
                  |      "ID": 0,
                  |      "Action": 0
                  |    },
                  |    {
                  |      "ID": 1,
                  |      "Action": 1
                  |    }
                  |  ]
                  |}"#.trim_margin()
                      .ok_or_else(|| anyhow!("Failed to extract margins from comparison JSON"))?);

    Ok(())
}

#[cfg(feature = "test_distopik_hq")]
#[actix::test]
async fn test_integration_distopik_hq() -> anyhow::Result<()> {
    let config = Config { address: "http://10.1.3.100".to_string(),
                          auth:    None, };

    let mut pdu = PowerPdu4c::new(FixedInstanceId { manufacturer: "netio".to_owned(),
                                                    name:         "power_pdu_4c".to_owned(),
                                                    instance:     "rack1-up-right".to_owned(), },
                                  config)?;

    pdu.set_power_channel(3, false);
    sleep(Duration::from_secs(15)).await;
    pdu.set_power_channel(3, true);
    sleep(Duration::from_secs(1)).await;

    Ok(())
}
