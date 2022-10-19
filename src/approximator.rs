use std::{error::Error, fmt::Display};

mod frame;
use frame::DataFrame;

#[derive(Default, Debug)]
struct Approximator {
    raw_data: DataFrame,
    calc_data: DataFrame,
    source_path: String,
}

fn auto_model_path(path: &str) -> String {
    let chunks: Vec<&str> = path.split('.').collect();

    let mut new_path = "".to_string();
    for chunk in chunks.iter().take(chunks.len() - 1) {
        new_path += chunk;
    }
    new_path + "_auto_model.txt"
}

impl Display for Approximator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Raw data:")?;
        writeln!(f, "{}", self.raw_data)?;
        writeln!(f, "Calc data:")?;
        writeln!(f, "{}", self.calc_data)?;
        Ok(())
    }
}

impl Approximator {
    pub fn from_path(path: &str, recalc: bool) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            raw_data: DataFrame::from_path(path)?,
            calc_data: if recalc {
                // TODO: добавить расчёт
                DataFrame::default()
            } else {
                DataFrame::from_path(&auto_model_path(path))?
            },
            source_path: path.to_string(),
        })
    }

    fn save_auto_model(&self) -> Result<(), Box<dyn Error>> {
        self.calc_data
            .save_file(&auto_model_path(&self.source_path))
    }
}
