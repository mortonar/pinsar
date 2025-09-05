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
    // TODO All metric types "could" be optional since we don't know if sar was run with -A.
    //      This could also handle (at least for now) the empty first placeholder statistics entry.

    // TODO Start with CPU load and implement a "vertical slice" down to graphing the data.
}
