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

fn abs_path(path: &str, suffix: &str) -> Result<String, Box<dyn Error>> {
    let abs_path = std::env::current_dir().unwrap().to_str().unwrap().to_owned() + "/" + path;

    let chunks: Vec<&str> = abs_path.split('.').collect();

    let mut new_path = "".to_string();
    for chunk in chunks.iter().take(chunks.len() - 1) {
        new_path += chunk;
    }

    Ok(new_path + suffix)
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
            source_path: path.to_string(),
            ..Default::default()
        };

        if recalc {
            item.calc_data = item.raw_data.calc();
            item.save_auto_model()?;
        } else {
            item.calc_data = DataFrame::from_path(&abs_path(path, "_auto_model.txt")?)?;
        };

        Ok(item)
    }

    fn save_auto_model(&self) -> Result<(), Box<dyn Error>> {
        self.calc_data
            .save_file(&abs_path(&self.source_path, "_auto_model.txt")?)
    }

    pub fn plot(&self, serial_number: &str) -> Result<(), Box<dyn Error>> {
        const RESOLUTION: (u32, u32) = (1800, 1100);

        plotter::plot(
            &abs_path(&self.source_path, "_with_model.png")?,
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
    const TEST_PATH: &str = "test/test_data.csv";

    let model = ThermoModel::from_path(TEST_PATH, false).unwrap();
    model.plot("TEST").unwrap();

    opener::open(abs_path(TEST_PATH, "_with_model.png").unwrap()).unwrap();
}