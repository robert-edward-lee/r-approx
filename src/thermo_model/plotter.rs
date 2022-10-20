use plotters::{
    coord::Shift,
    prelude::*,
    style::full_palette::{DEEPORANGE_300, LIGHTBLUE_600},
};
use std::error::Error;

const MAIN_HEADER_SIZE: u32 = 40;
const FONT: &str = "sans-serif";
const X_HEADER: &str = "ГН";
const Y_HEADER: &str = "ВН";
const MARK_SIZE: u32 = 4;

pub fn plot(
    path: &str,
    header: &str,
    resolution: (u32, u32),
    raw_data_x: Vec<(i32, i32)>,
    calc_data_x: Vec<(i32, i32)>,
    raw_data_y: Vec<(i32, i32)>,
    calc_data_y: Vec<(i32, i32)>,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(path, resolution).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.titled(header, (FONT, MAIN_HEADER_SIZE))?;

    let (upper, lower) = root.split_vertically((resolution.1 - MAIN_HEADER_SIZE) / 2);

    plot_area(upper, X_HEADER, raw_data_x, calc_data_x)?;
    plot_area(lower, Y_HEADER, raw_data_y, calc_data_y)?;

    root.present()?;
    Ok(())
}

fn plot_area(
    area: DrawingArea<BitMapBackend, Shift>,
    header: &str,
    raw_data: Vec<(i32, i32)>,
    calc_data: Vec<(i32, i32)>,
) -> Result<(), Box<dyn Error>> {
    use std::cmp::{max, min};

    let y_min = min(
        raw_data.iter().map(|(_, y)| y).min().unwrap_or(&0),
        calc_data.iter().map(|(_, y)| y).min().unwrap_or(&0),
    ) - 1;
    let y_max = max(
        raw_data.iter().map(|(_, y)| y).max().unwrap_or(&0),
        calc_data.iter().map(|(_, y)| y).max().unwrap_or(&0),
    ) + 1;

    let mut chart = ChartBuilder::on(&area)
        .margin(10)
        .caption(header, (FONT, MAIN_HEADER_SIZE / 2))
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Right, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(-50..70, y_min..y_max)?;

    chart
        .configure_mesh()
        .x_labels(21 + 2)
        .max_light_lines(4)
        .draw()?;

    chart.draw_series(
        raw_data
            .iter()
            .map(|(temp, x)| Circle::new((*temp, *x), MARK_SIZE, LIGHTBLUE_600.filled())),
    )?;

    chart.draw_series(
        calc_data
            .iter()
            .map(|(temp, x)| Circle::new((*temp, *x), MARK_SIZE, DEEPORANGE_300.filled())),
    )?;

    Ok(())
}
