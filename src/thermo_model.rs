use chrono::{DateTime, Datelike, Local, Timelike};
use std::error::Error;

mod frame;
use frame::DataFrame;
mod plotter;

#[derive(Default, Debug)]
pub struct ThermoModel {
    raw_data: DataFrame,
    calc_data: DataFrame,
    source_path: String,
    serial_number: String,
    date: DateTime<Local>,
}

fn abs_path(path: &str, suffix: &str) -> Result<String, Box<dyn Error>> {
    let abs_path = std::env::current_dir()?.to_str().unwrap().to_owned() + "/" + path;

    let chunks: Vec<&str> = abs_path.split('.').collect();

    let mut new_path = "".to_string();
    for chunk in chunks.iter().take(chunks.len() - 1) {
        new_path += chunk;
    }

    Ok(new_path + suffix)
}

impl std::fmt::Display for ThermoModel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "|      |")?;
        for (i, _) in self.calc_data.rows.iter().enumerate() {
            write!(f, "  {:2} |", i)?;
        }
        write!(f, "\r\n")?;

        write!(f, "|:-----|")?;
        for (_, _) in self.calc_data.rows.iter().enumerate() {
            write!(f, "----:|")?;
        }
        write!(f, "\r\n")?;

        write!(f, "| temp |")?;
        for row in self.calc_data.rows.iter() {
            write!(f, " {:3} |", row.temp.unwrap())?;
        }
        write!(f, "\r\n")?;

        write!(f, "| dx   |")?;
        for row in self.calc_data.rows.iter() {
            write!(f, " {:3} |", row.x.unwrap())?;
        }
        write!(f, "\r\n")?;

        write!(f, "| dy   |")?;
        for row in self.calc_data.rows.iter() {
            write!(f, " {:3} |", row.y.unwrap())?;
        }
        write!(f, "\r\n")?;

        Ok(())
    }
}

impl ThermoModel {
    pub fn from_path(
        path: &str,
        recalc: bool,
        optional_path: Option<&str>,
    ) -> Result<Self, Box<dyn Error>> {
        let mut item = ThermoModel {
            raw_data: DataFrame::from_path(path)?,
            source_path: path.to_string(),
            date: Local::now(),
            ..Default::default()
        };

        if recalc {
            item.calc_data = item.raw_data.calc();
            item.save_auto_model(&abs_path(path, "_auto_model.txt")?)?;
        } else {
            item.calc_data = match optional_path {
                Some(optional_path) => DataFrame::from_path(optional_path)?,
                None => DataFrame::from_path(&abs_path(path, "_auto_model.txt")?)?,
            }
        };

        Ok(item)
    }

    pub fn plot(&self) -> Result<(), Box<dyn Error>> {
        let img_path = abs_path(&self.source_path, "_with_model.png")?;
        let header = format!(
            "{} ({}.{}.{})",
            self.serial_number,
            self.date.day(),
            self.date.month(),
            self.date.year()
        );

        plotter::plot(
            &img_path,
            &header,
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
        )?;
        opener::open(&img_path)?;
        Ok(())
    }

    pub fn md(&self) -> Result<(), Box<dyn Error>> {
        std::fs::write(
            &abs_path(&self.source_path, "_model.md")?,
            self.to_string().as_bytes(),
        )?;
        Ok(())
    }

    pub fn ct(&self) -> Result<(), Box<dyn Error>> {
        let f_name = format!(
            "tpk-k_{}_{}-{}-{}_{}-{}.ct",
            self.serial_number,
            self.date.year(),
            self.date.month(),
            self.date.day(),
            self.date.hour(),
            self.date.minute()
        );
        let path = std::env::current_dir()?.to_str().unwrap().to_owned() + "/" + &f_name;

        self.save_auto_model(&path)
    }

    pub fn with_serial_number(&mut self, serial_number: &str) {
        self.serial_number = serial_number.to_string();
    }
}

impl ThermoModel {
    fn save_auto_model(&self, path: &str) -> Result<(), Box<dyn Error>> {
        self.calc_data.save_file(path)
    }
}

#[test]
fn full_test() {
    // let model = ThermoModel::from_path("test/test_data.csv", true, None).unwrap();
    // model.md().unwrap();
    // model.plot().unwrap();

    let model = ThermoModel::from_path("test/old_data.txt", true, None).unwrap();
    model.md().unwrap();
    model.plot().unwrap();
}

// cargo test --package r-approx --bin r-approx -- thermo_model::full_test --exact --nocapture
