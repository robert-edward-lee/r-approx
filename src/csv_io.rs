use serde_derive::{Deserialize, Serialize};
use std::fmt::Display;

const ABS_HEADERS: [&str; 3] = ["temp", "x", "y"];
const DIF_HEADERS: [&str; 3] = ["temp", "dx", "dy"];

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
struct DataRow {
    temp: Option<i32>,
    #[serde(rename(serialize = "dx"))]
    x: Option<i32>,
    #[serde(rename(serialize = "dy"))]
    y: Option<i32>,
}

impl Display for DataRow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "temp: ")?;
        if self.temp == None {
            write!(f, "nan; ")?;
        } else {
            write!(f, "{:3}; ", self.temp.unwrap())?;
        }

        write!(f, "x: ")?;
        if self.x == None {
            write!(f, "nan; ")?;
        } else {
            write!(f, "{:3}; ", self.x.unwrap())?;
        }

        write!(f, "y: ")?;
        if self.y == None {
            write!(f, "nan;")?;
        } else {
            write!(f, "{:3};", self.y.unwrap())?;
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
struct DataFrame {
    frames: Vec<DataRow>,
    diff: bool,
}

impl Display for DataFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for frame in self.frames.iter() {
            write!(f, "temp: ")?;
            if frame.temp == None {
                write!(f, "nan; ")?;
            } else {
                write!(f, "{:3}; ", frame.temp.unwrap())?;
            }

            write!(f, "{}: ", if self.diff { "dx" } else { "x" })?;
            if frame.x == None {
                write!(f, "nan; ")?;
            } else {
                write!(f, "{:3}; ", frame.x.unwrap())?;
            }

            write!(f, "{}: ", if self.diff { "dy" } else { "y" })?;
            if frame.y == None {
                write!(f, "nan;")?;
            } else {
                write!(f, "{:3};", frame.y.unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl DataFrame {
    pub fn from_path(path: &str) -> Self {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b';')
            .from_path(path)
            .unwrap();

        let mut data_frame = Self::default();
        for result in reader.deserialize() {
            let row: DataRow = result.unwrap_or_default();
            data_frame.frames.push(row);
        }

        let headers = reader.headers().unwrap().iter().collect::<Vec<&str>>();
        if headers == ABS_HEADERS {
            data_frame.to_dif()
        } else {
            println!("Wrong headers!");
            Self::default()
        }
    }

    fn to_dif(&self) -> Self {
        let mut item = Self::default();
        let mut x0 = 0;
        let mut y0 = 0;
        for (i, row) in self.frames.iter().enumerate() {
            if *row == DataRow::default() {
                x0 = self.frames[i + 1].x.unwrap();
                y0 = self.frames[i + 1].y.unwrap();
            } else {
                let mut dif_row = *row;
                dif_row.x = Some(row.x.unwrap() - x0);
                dif_row.y = Some(row.y.unwrap() - y0);
                item.frames.push(dif_row)
            }
        }

        item.sort();
        item.diff = true;
        item
    }

    fn sort(&mut self) {
        self.frames
            .sort_by(|a, b| a.temp.unwrap().partial_cmp(&b.temp.unwrap()).unwrap());
    }

    pub fn calc(&self) -> Self {
        let mut item = Self::default();

        for i in (-50..=70).step_by(6) {
            let tail: Vec<DataRow> = self
                .frames
                .iter()
                .filter(|row| i - 3 <= row.temp.unwrap() && row.temp.unwrap() <= i + 3)
                .cloned()
                .collect();
            let x = tail.iter().fold(0, |x, row| x + row.x.unwrap()) / (tail.len() as i32);
            let y = tail.iter().fold(0, |y, row| y + row.y.unwrap()) / (tail.len() as i32);
            item.frames.push(DataRow {
                temp: Some(i),
                x: Some(x),
                y: Some(y),
            });
        }
        item
    }

    pub fn save_file(self, path: &str) {
        let mut writer = csv::Writer::from_path(path).unwrap();

        for data in self.frames.into_iter() {
            writer.serialize(data).unwrap();
        }
        writer.flush().unwrap();
    }
}

#[test]
fn open_csv() {
    const TEST_FILE: &str = "test/test_data.csv";
    const WRONG_FILE: &str = "test/wrong_data.csv";

    let data_frame = DataFrame::from_path(TEST_FILE);
    println!("{}", data_frame);
    let calc_frame = data_frame.calc();
    println!("{}", calc_frame);

    let new_path = &TEST_FILE.replace(".csv", "_auto_model.txt")[..];
    calc_frame.save_file(new_path);
}
