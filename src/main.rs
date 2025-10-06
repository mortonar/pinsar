mod clioptions;
mod data;

use crate::clioptions::CliOptions;
use chrono::{DateTime, Local, Utc};
use clap::Parser;
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
use plotters::style::full_palette::{ORANGE, ORANGE_50, PURPLE};
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let opts = CliOptions::parse();

    let stream = opts.input_stream()?;

    let output_dir = opts.output_dir;
    std::fs::create_dir_all(&output_dir)?;

    // TODO Parsing sar JSON can be slow for large files.
    //      We need a more efficient format to parse (e.g. sadf -d)
    //      -> Try dropping serde dependency and just parse sadf -d output manually
    let parse_start = Local::now();
    let sar_data: data::SarData = serde_json::from_reader(stream)?;
    let duration = Local::now() - parse_start;
    println!("Finished parsing JSON: {}ms", duration.num_milliseconds());

    chart_cpu_load_all(&sar_data, output_dir);

    Ok(())
}

fn chart_cpu_load_all(data: &data::SarData, output_dir: PathBuf) {
    let cpu_load_all = data.cpu_load_all();

    let cpu_all_output = output_dir.join("cpu_all.png");
    let root_area = BitMapBackend::new(&cpu_all_output, (1920, 1080)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let start_date = cpu_load_all.first().unwrap().0;
    let end_date = cpu_load_all.last().unwrap().0;
    let max_sys = 100.0f32;

    // TODO Improve image resolution (so we can see more/all the colors)
    // TODO Chart "Used [%]" label
    let mut chart = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("CPU load (all)", ("sans-serif", 40))
        .build_cartesian_2d(start_date..end_date, 0.0..max_sys)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    // TODO Match colors with kSar?
    chart_cpu_load(&cpu_load_all, &mut chart, &CYAN, |l| l.usr + l.guest);
    chart_cpu_load(&cpu_load_all, &mut chart, &BLUE, |l| l.usr + l.soft);
    chart_cpu_load(&cpu_load_all, &mut chart, &ORANGE_50, |l| l.usr + l.irq);
    chart_cpu_load(&cpu_load_all, &mut chart, &ORANGE, |l| l.usr + l.steal);
    chart_cpu_load(&cpu_load_all, &mut chart, &PURPLE, |l| l.usr + l.iowait);
    chart_cpu_load(&cpu_load_all, &mut chart, &RED, |l| l.usr + l.sys);
    chart_cpu_load(&cpu_load_all, &mut chart, &YELLOW, |l| l.usr + l.nice);
    chart_cpu_load(&cpu_load_all, &mut chart, &GREEN, |l| l.usr);

    // TODO Chart "Idle [%]"
}

fn chart_cpu_load<F: Fn(&data::CpuLoad) -> f32>(
    data: &[(DateTime<Utc>, &data::CpuLoad)],
    chart: &mut ChartContext<
        BitMapBackend,
        Cartesian2d<RangedDateTime<DateTime<Utc>>, RangedCoordf32>,
    >,
    color: &RGBColor,
    field: F,
) {
    let series = data.iter().map(|(time, load)| (*time, field(load)));
    chart
        .draw_series(AreaSeries::new(series, 0., color))
        .unwrap();
}
