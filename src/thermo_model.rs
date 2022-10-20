use std::{error::Error, fmt::Display};

mod frame;
use frame::DataFrame;
mod plotter;

#[derive(Default, Debug)]
pub struct ThermoModel {
    raw_data: DataFrame,
    calc_data: DataFrame,
    source_path: String,
}

fn path_with_suffix(path: &str, suffix: &str) -> String {
    let chunks: Vec<&str> = path.split('.').collect();

    let mut new_path = "".to_string();
    for chunk in chunks.iter().take(chunks.len() - 1) {
        new_path += chunk;
    }
    new_path + suffix
}

impl Display for ThermoModel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Raw data:")?;
        writeln!(f, "{}", self.raw_data)?;
        writeln!(f, "Calc data:")?;
        writeln!(f, "{}", self.calc_data)?;
        Ok(())
    }
}

impl ThermoModel {
    pub fn from_path(path: &str, recalc: bool) -> Result<Self, Box<dyn Error>> {
        let mut item = ThermoModel {
            raw_data: DataFrame::from_path(path)?,
            ..Default::default()
        };

        if recalc {
            item.calc_data = item.raw_data.calc();
            item.save_auto_model()?;
        } else {
            item.calc_data = DataFrame::from_path(&path_with_suffix(path, "_auto_model.txt"))?;
        };
        item.source_path = path.to_string();

        Ok(item)
    }

    fn save_auto_model(&self) -> Result<(), Box<dyn Error>> {
        self.calc_data
            .save_file(&path_with_suffix(&self.source_path, "_auto_model.txt"))
    }

    pub fn plot(&self, serial_number: &str) -> Result<(), Box<dyn Error>> {
        const RESOLUTION: (u32, u32) = (1800, 1100);

        plotter::plot(
            &path_with_suffix(&self.source_path, "_with_model.png"),
            serial_number,
            RESOLUTION,
            self.raw_data
                .rows
                .iter()
                .map(|row| (row.temp.unwrap(), row.x.unwrap()))
                .collect::<Vec<(i32, i32)>>(),
            self.calc_data
                .rows
                .iter()
                .map(|row| (row.temp.unwrap(), row.x.unwrap()))
                .collect::<Vec<(i32, i32)>>(),
            self.raw_data
                .rows
                .iter()
                .map(|row| (row.temp.unwrap(), row.y.unwrap()))
                .collect::<Vec<(i32, i32)>>(),
            self.calc_data
                .rows
                .iter()
                .map(|row| (row.temp.unwrap(), row.y.unwrap()))
                .collect::<Vec<(i32, i32)>>(),
        )
    }
}

#[test]
fn test_auto_model() {
    let test_auto_model = DataFrame::from_path("test/test_data.csv").unwrap().calc();
    let auto_model = DataFrame::from_path("test/test_data_auto_model.txt").unwrap();
    assert_eq!(test_auto_model.rows, auto_model.rows);
}

#[test]
fn test_plotter() {
    let model = ThermoModel::from_path("test/test_data.csv", false).unwrap();
    model.plot("БЛН...").unwrap();
}
