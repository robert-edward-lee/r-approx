use serde_derive::{Deserialize, Serialize};
use std::fmt::Display;

const ABS_HEADERS: [&str; 3] = ["temp", "x", "y"];

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
struct DataRow {
    temp: Option<i32>,
    x: Option<i32>,
    y: Option<i32>,
}

impl Display for DataRow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "temp: ").unwrap();
        if self.temp == None {
            write!(f, "nan; ").unwrap();
        } else {
            write!(f, "{:3}; ", self.temp.unwrap()).unwrap();
        }

        write!(f, "x: ").unwrap();
        if self.x == None {
            write!(f, "nan; ").unwrap();
        } else {
            write!(f, "{:3}; ", self.x.unwrap()).unwrap();
        }

        write!(f, "y: ").unwrap();
        if self.y == None {
            write!(f, "nan;").unwrap();
        } else {
            write!(f, "{:3};", self.y.unwrap()).unwrap();
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
            write!(f, "temp: ").unwrap();
            if frame.temp == None {
                write!(f, "nan; ").unwrap();
            } else {
                write!(f, "{:3}; ", frame.temp.unwrap()).unwrap();
            }

            write!(f, "{}: ", if self.diff {"dx"} else {"x"}).unwrap();
            if frame.x == None {
                write!(f, "nan; ").unwrap();
            } else {
                write!(f, "{:3}; ", frame.x.unwrap()).unwrap();
            }

            write!(f, "{}: ", if self.diff {"dy"} else {"y"}).unwrap();
            if frame.y == None {
                write!(f, "nan;").unwrap();
            } else {
                write!(f, "{:3};", frame.y.unwrap()).unwrap();
            }
            writeln!(f).unwrap();
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
        self.frames.sort_by(|a, b| a.temp.unwrap().partial_cmp(&b.temp.unwrap()).unwrap());
    }
}

#[test]
fn open_csv() {
    const TEST_FILE: &str = "test/test_data.csv";
    const WRONG_FILE: &str = "test/wrong_data.csv";

    let data_frame = DataFrame::from_path(TEST_FILE);
    println!("{}", data_frame);

    let data_frame = DataFrame::from_path(WRONG_FILE);
    println!("{}", data_frame);
}
