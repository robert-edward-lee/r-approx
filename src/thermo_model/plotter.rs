use plotters::{
    coord::Shift,
    prelude::*,
    style::full_palette::{DEEPORANGE, LIGHTBLUE_600, TEAL_400},
};
use std::cmp::{max, min};
use std::error::Error;

const RESOLUTION: (u32, u32) = (1800, 1100);

const MAIN_HEADER_SIZE: u32 = 40;
const FONT: &str = "sans-serif";
const X_HEADER: &str = "ГН";
const Y_HEADER: &str = "ВН";

const CENTER_LINE_STYLE: ShapeStyle = ShapeStyle {
    color: RGBAColor(BLACK.0, BLACK.1, BLACK.2, 0.8),
    filled: true,
    stroke_width: 2,
};
const STEPPED_LINE_STYLE: ShapeStyle = ShapeStyle {
    color: RGBAColor(TEAL_400.0, TEAL_400.1, TEAL_400.2, 1.0),
    filled: true,
    stroke_width: 2,
};

const MARK_SIZE: u32 = 5;
const RAW_MARK_STYLE: ShapeStyle = ShapeStyle {
    color: RGBAColor(LIGHTBLUE_600.0, LIGHTBLUE_600.1, LIGHTBLUE_600.2, 1.0),
    filled: true,
    stroke_width: 1,
};
const CALC_MARK_STYLE: ShapeStyle = ShapeStyle {
    color: RGBAColor(DEEPORANGE.0, DEEPORANGE.1, DEEPORANGE.2, 1.0),
    filled: true,
    stroke_width: 1,
};

pub fn plot(
    path: &str,
    header: &str,
    raw_data_x: Vec<(i32, i32)>,
    calc_data_x: Vec<(i32, i32)>,
    raw_data_y: Vec<(i32, i32)>,
    calc_data_y: Vec<(i32, i32)>,
) -> Result<(), Box<dyn Error>> {
    let canvas = BitMapBackend::new(path, RESOLUTION).into_drawing_area();
    canvas.fill(&WHITE)?;
    let canvas = canvas.titled(header, (FONT, MAIN_HEADER_SIZE))?;

    let (upper, lower) = canvas.split_vertically((RESOLUTION.1 - MAIN_HEADER_SIZE) / 2);

    plot_area(upper, X_HEADER, raw_data_x, calc_data_x)?;
    plot_area(lower, Y_HEADER, raw_data_y, calc_data_y)?;

    canvas.present()?;
    Ok(())
}

fn plot_area(
    area: DrawingArea<BitMapBackend, Shift>,
    header: &str,
    raw_data: Vec<(i32, i32)>,
    calc_data: Vec<(i32, i32)>,
) -> Result<(), Box<dyn Error>> {
    let y_min = min(
        raw_data.iter().map(|(_, y)| y).min().unwrap_or(&0),
        calc_data.iter().map(|(_, y)| y).min().unwrap_or(&0),
    ) - 1;
    let y_max = max(
        raw_data.iter().map(|(_, y)| y).max().unwrap_or(&0),
        calc_data.iter().map(|(_, y)| y).max().unwrap_or(&0),
    ) + 1;

    let mut chart = ChartBuilder::on(&area)
        .caption(header, (FONT, MAIN_HEADER_SIZE / 2))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Right, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d((-56..76).step(6), y_min..y_max)?;

    chart
        .configure_mesh()
        .x_labels(21)
        .y_labels((y_max.abs() + y_min.abs()) as usize)
        .label_style((FONT, MAIN_HEADER_SIZE / 2))
        .x_label_formatter(&|x| {
            if *x == -56 {
                String::default()
            } else {
                format!("{}", x)
            }
        })
        .draw()?;

    chart.draw_series(LineSeries::new(
        (-56..=76).map(|x| (x, 0)),
        CENTER_LINE_STYLE,
    ))?;

    chart.draw_series(
        raw_data
            .iter()
            .map(|coord| Circle::new(*coord, MARK_SIZE, RAW_MARK_STYLE)),
    )?;

    chart.draw_series(
        calc_data
            .iter()
            .map(|coord| TriangleMarker::new(*coord, MARK_SIZE, CALC_MARK_STYLE)),
    )?;

    for pairs in calc_data.windows(2) {
        if pairs[0].1 == pairs[1].1 {
            chart.draw_series(LineSeries::new(
                (pairs[0].0..=pairs[1].0).map(|_x| (_x, pairs[0].1)),
                STEPPED_LINE_STYLE,
            ))?;
        } else {
            let average = (pairs[0].0 + pairs[1].0) / 2;

            chart.draw_series(LineSeries::new(
                (pairs[0].0..=average).map(|_x| (_x, pairs[0].1)),
                STEPPED_LINE_STYLE,
            ))?;

            chart.draw_series(LineSeries::new(
                (min(pairs[0].1, pairs[1].1)..=max(pairs[0].1, pairs[1].1)).map(|_y| (average, _y)),
                STEPPED_LINE_STYLE,
            ))?;

            chart.draw_series(LineSeries::new(
                (average..=pairs[1].0).map(|_x| (_x, pairs[1].1)),
                STEPPED_LINE_STYLE,
            ))?;
        }
    }

    let poly = crate::polynomial::Polynomial::lagrange(calc_data).unwrap();

    chart.draw_series(LineSeries::new(
        (-56..=76).map(|x| (x, poly.f(x as f64) as i32)),
        CENTER_LINE_STYLE,
    ))?;

    Ok(())
}
