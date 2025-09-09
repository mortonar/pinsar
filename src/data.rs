use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SarData {
    sysstat: Sysstat,
}

#[derive(Deserialize, Debug)]
struct Sysstat {
    hosts: Vec<Host>,
}

#[derive(Deserialize, Debug)]
struct Host {
    #[serde(rename = "nodename")]
    node_name: String,
    #[serde(rename = "sysname")]
    sys_name: String,
    release: String,
    machine: String,
    #[serde(rename = "number-of-cpus")]
    number_of_cpus: u32,
    #[serde(rename = "file-date")]
    file_date: String,
    #[serde(rename = "file-utc-time")]
    file_utc_time: String,
    statistics: Vec<Statistics>,
}

#[derive(Deserialize, Debug)]
struct Statistics {
    timestamp: Option<Timestamp>,
    #[serde(rename = "cpu-load")]
    cpu_load: Option<Vec<CpuLoad>>,
    // TODO Start with CPU load and implement a "vertical slice" down to graphing the data.
}

#[derive(Deserialize, Debug)]
struct Timestamp {
    date: String,
    time: String,
    utc: u32,
    interval: u32,
}

#[derive(Deserialize, Debug)]
struct CpuLoad {
    cpu: String,
    usr: f32,
    nice: f32,
    sys: f32,
    iowait: f32,
    steal: f32,
    irq: f32,
    soft: f32,
    guest: f32,
    gnice: f32,
    idle: f32,
}
