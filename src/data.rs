use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SarData {
    sysstat: Sysstat,
}

impl SarData {
    pub fn cpu_load_all(&self) -> Vec<(DateTime<Utc>, &CpuLoad)> {
        self.sysstat.hosts[0]
            .statistics
            .iter()
            .skip(1)
            // TODO Hack because some data can be missing
            .map(
                |stat| match (stat.timestamp.as_ref(), stat.cpu_load.as_ref()) {
                    (Some(timestamp), Some(load)) => Some((timestamp.into(), &load[0])),
                    _ => None,
                },
            )
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect()
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Sysstat {
    hosts: Vec<Host>,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Statistics {
    timestamp: Option<Timestamp>,
    #[serde(rename = "cpu-load")]
    cpu_load: Option<Vec<CpuLoad>>,
    // TODO Start with CPU load and implement a "vertical slice" down to graphing the data.
}

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct CpuLoad {
    pub cpu: String,
    pub usr: f32,
    pub nice: f32,
    pub sys: f32,
    pub iowait: f32,
    pub steal: f32,
    pub irq: f32,
    pub soft: f32,
    pub guest: f32,
    pub gnice: f32,
    pub idle: f32,
}
