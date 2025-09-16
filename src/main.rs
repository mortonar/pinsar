mod clioptions;
mod data;

use crate::clioptions::CliOptions;
use chrono::{DateTime, Local, Utc};
use clap::Parser;

use plotters::prelude::*;

fn main() -> std::io::Result<()> {
    let opts = CliOptions::parse();
    let stream = opts.input_stream()?;

    let parse_start = Local::now();
    let sar_data: data::SarData = serde_json::from_reader(stream)?;
    let duration = Local::now() - parse_start;
    println!("Finished parsing JSON: {}ms", duration.num_milliseconds());

    chart_cpu_load_sys(&sar_data);

    Ok(())
}

fn chart_cpu_load_sys(data: &data::SarData) {
    let cpu_load_sys = data.cpu_load_sys();

    let root_area = BitMapBackend::new("images/cpu_sys.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let start_date = cpu_load_sys.first().unwrap().0;
    let end_date = cpu_load_sys.last().unwrap().0;
    let max_sys = 100.0f32;

    let mut chart = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("CPU load", ("sans-serif", 40))
        .build_cartesian_2d(start_date..end_date, 0.0..max_sys)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(AreaSeries::new(
            cpu_load_sys
                .iter()
                .map(|(time, (usr, sys))| (time.clone(), *usr + *sys))
                .collect::<Vec<(DateTime<Utc>, f32)>>(),
            0.,
            &RED,
        ))
        .unwrap();
    chart
        .draw_series(AreaSeries::new(
            cpu_load_sys
                .iter()
                .map(|(time, (usr, sys))| (time.clone(), *usr))
                .collect::<Vec<(DateTime<Utc>, f32)>>(),
            0.,
            &GREEN,
        ))
        .unwrap();
}
