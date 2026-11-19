use std::env;

fn main() {
    println!("cargo::rerun-if-env-changed=AZALEA_LOG_ALL_PACKET_FIELDS");

    let log_all_packet_fields_env = env::var("AZALEA_LOG_ALL_PACKET_FIELDS")
        .map(|s| s == "1" || s == "true")
        .unwrap_or(false);

    println!("cargo::rustc-env=__INTERNAL_AZALEA_LOG_ALL_PACKET_FIELDS={log_all_packet_fields_env}")
}
