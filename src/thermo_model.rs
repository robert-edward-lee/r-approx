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
        writeln!(f)?;

        write!(f, "|:-----|")?;
        for (_, _) in self.calc_data.rows.iter().enumerate() {
            write!(f, "----:|")?;
        }
        writeln!(f)?;

        write!(f, "| temp |")?;
        for row in self.calc_data.rows.iter() {
            write!(f, " {:3} |", row.temp.unwrap())?;
        }
        writeln!(f)?;

        write!(f, "| dx   |")?;
        for row in self.calc_data.rows.iter() {
            write!(f, " {:3} |", row.x.unwrap())?;
        }
        writeln!(f)?;

        write!(f, "| dy   |")?;
        for row in self.calc_data.rows.iter() {
            write!(f, " {:3} |", row.y.unwrap())?;
        }
        writeln!(f)?;

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
            item.save_auto_model(&abs_path(path, "_auto_model.txt")?)?;
        } else {
            item.calc_data = DataFrame::from_path(&abs_path(path, "_auto_model.txt")?)?;
        };

        Ok(item)
    }

    pub fn plot(&self) -> Result<(), Box<dyn Error>> {
        const RESOLUTION: (u32, u32) = (1800, 1100);
        let img_path = abs_path(&self.source_path, "_with_model.png")?;

        plotter::plot(
            &img_path,
            &self.serial_number,
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
        use chrono::{Datelike, Timelike};

        let dt = chrono::Local::now();
        let f_name = format!(
            "tpk-k_{}_{}-{}-{}_{}-{}.ct",
            self.serial_number,
            dt.year(),
            dt.month(),
            dt.day(),
            dt.hour(),
            dt.minute()
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
    let model = ThermoModel::from_path("test/test_data.csv", true).unwrap();
    model.md().unwrap();
    model.plot().unwrap();

    let model = ThermoModel::from_path("test/old_data.txt", true).unwrap();
    model.md().unwrap();
    model.plot().unwrap();
}
