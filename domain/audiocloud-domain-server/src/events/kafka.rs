use rdkafka::config::RDKafkaLogLevel;
use rdkafka::ClientConfig;

pub mod commands;
pub mod events;

pub fn create_config(bootstrap_servers: &str, username: &str, password: &str) -> ClientConfig {
    let mut config = ClientConfig::default();

    config
        .set("bootstrap.servers", bootstrap_servers)
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanisms", "SCRAM-SHA-256")
        .set("sasl.username", username)
        .set("sasl.password", password)
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("group.id", "audiocloud-domain-server")
        .set_log_level(RDKafkaLogLevel::Debug);

    config
}
