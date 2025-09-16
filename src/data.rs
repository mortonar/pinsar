use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SarData {
    sysstat: Sysstat,
}

impl SarData {
    pub fn cpu_load_sys(&self) -> Vec<(DateTime<Utc>, (f32, f32))> {
        self.sysstat.hosts[0]
            .statistics
            .iter()
            .skip(1)
            // TODO Hack because some data can be missing
            .map(
                |stat| match (stat.timestamp.as_ref(), stat.cpu_load.as_ref()) {
                    (Some(timestamp), Some(load)) => {
                        Some((timestamp.into(), (load[0].usr, load[0].sys)))
                    }
                    _ => None,
                },
            )
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect()
    }
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

// TODO Handle interpreting in desired timezone
impl From<&Timestamp> for DateTime<Utc> {
    fn from(value: &Timestamp) -> Self {
        let datetime_str = format!("{}T{}Z", value.date, value.time);
        DateTime::parse_from_rfc3339(&datetime_str)
            .expect("Failed to parse datetime")
            .with_timezone(&Utc)
    }
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
