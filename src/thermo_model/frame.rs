use std::{error::Error, fs};

mod row;
use row::DataRow;

const ABS_HEADERS: [&str; 3] = ["temp", "x", "y"];
const DIF_HEADERS: [&str; 3] = ["temp", "dx", "dy"];

/// Структура данных для таблицы с записями
#[derive(Default, Debug)]
pub struct DataFrame {
    pub rows: Vec<DataRow>,
}

impl std::fmt::Display for DataFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.rows.is_empty() {
            write!(f, "<nil>")?;
            return Ok(());
        }

        write!(
            f,
            "{};{};{}\r\n",
            DIF_HEADERS[0], DIF_HEADERS[1], DIF_HEADERS[2]
        )?;

        for row in self.rows.iter().take(self.rows.len() - 1) {
            write!(f, "{}\r\n", row)?;
        }
        write!(f, "{}", self.rows[self.rows.len() - 1])?;

        Ok(())
    }
}

impl DataFrame {
    fn from_str(table: &str) -> Result<Self, Box<dyn Error>> {
        let mut item = DataFrame::default();

        let strings: Vec<&str> = table.lines().collect();
        if strings.len() < 2 {
            Err("Error! Amount of table rows less 2")?
        }

        let header: Vec<&str> = strings[0].split(';').collect();
        if header.len() != 3 {
            Err(format!("Error! Invalid headers length: {:?}", header))?
        }

        let mut diff = false;
        if header == DIF_HEADERS {
            diff = true;
        } else if header != ABS_HEADERS {
            Err(format!("Error! Invalid headers format: {:?}", header))?
        }

        for row in strings.iter().skip(1) {
            item.rows.push(DataRow::from_str(row)?);
        }

        if diff {
            item.rows.retain(|x| *x != DataRow::default());
            item.sort();
            Ok(item)
        } else {
            item.to_dif()
        }
    }

    /// загрузка таблицы из файла
    pub fn from_path(path: &str) -> Result<Self, Box<dyn Error>> {
        DataFrame::from_str(&fs::read_to_string(path)?)
    }

    /// перевод координат x, y в относительные координаты
    pub fn to_dif(&self) -> Result<Self, Box<dyn Error>> {
        let mut item = Self::default();
        let mut x0 = 0;
        let mut y0 = 0;

        for (i, row) in self.rows.iter().enumerate() {
            match row {
                DataRow {
                    temp: None,
                    x: None,
                    y: None,
                } => {
                    x0 = self.rows[i + 1].x.unwrap();
                    y0 = self.rows[i + 1].y.unwrap();
                }
                DataRow {
                    temp: Some(temp),
                    x: Some(x),
                    y: Some(y),
                } => {
                    item.rows.push(DataRow {
                        temp: Some(*temp),
                        x: Some(x - x0),
                        y: Some(y - y0),
                    });
                }
                _ => Err(format!("Invalid row format: `{}`", row))?,
            }
        }
        item.sort();

        Ok(item)
    }

    /// сортировка по возрастанию температуры
    fn sort(&mut self) {
        self.rows
            .sort_by(|a, b| a.temp.unwrap().partial_cmp(&b.temp.unwrap()).unwrap());
        self.rows.dedup();
    }

    /// вычисление аппроксимированных координат
    pub fn calc(&self) -> Self {
        use crate::polynomial::Polynomial;

        let x_data: Vec<(i32, i32)> = self
            .rows
            .iter()
            .map(|row| (row.temp.unwrap(), row.x.unwrap()))
            .collect();
        let poly_x = Polynomial::lagrange(x_data).unwrap();
println!("poly_x: ({})", poly_x);
        let y_data: Vec<(i32, i32)> = self
            .rows
            .iter()
            .map(|row| (row.temp.unwrap(), row.y.unwrap()))
            .collect();
        let poly_y = Polynomial::lagrange(y_data).unwrap();
println!("poly_y: ({})", poly_y);
        let item: Vec<DataRow> = (-50..=70).step_by(6).map(|t|

        DataRow {
            temp: Some(t),
            x: Some(poly_x.f(t as f64) as i32),
            y: Some(poly_y.f(t as f64) as i32),
        }

        ).collect();

        Self { rows: item }
    }

    /// сохранить csv файл
    pub fn save_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        fs::write(path, self.to_string().as_bytes())?;
        Ok(())
    }
}

fn median(mut data: Vec<i32>) -> Option<i32> {
    let len = data.len();
    if len == 0 {
        None?
    }

    data.sort();
    if len % 2 == 1 {
        Some(data[len / 2])
    } else {
        Some((data[len / 2] + data[len / 2]) / 2)
    }
}

#[test]
fn string_to_frame() {
    let table = "temp;x;y\r\n12;34;56";
    let frame = DataFrame::from_str(table).unwrap();
    assert_eq!(
        frame.rows,
        vec![DataRow {
            temp: Some(12),
            x: Some(34),
            y: Some(56),
        }]
    );

    let table = "temp;dx;dy\r\n12;34;56";
    let frame = DataFrame::from_str(table).unwrap();
    assert_eq!(
        frame.rows,
        vec![DataRow {
            temp: Some(12),
            x: Some(34),
            y: Some(56),
        }]
    );

    let table = "temp;x;y\r\nnan;nan;nan\r\n12;34;56";
    let frame = DataFrame::from_str(table).unwrap();
    assert_eq!(
        frame.rows,
        vec![DataRow {
            temp: Some(12),
            x: Some(0),
            y: Some(0)
        }]
    );
}

#[test]
fn frame_to_string() {
    let frame = DataFrame {
        rows: vec![DataRow {
            temp: Some(12),
            x: Some(34),
            y: Some(56),
        }],
    };
    assert_eq!(frame.to_string(), "temp;dx;dy\r\n12;34;56".to_string());

    let frame = DataFrame {
        rows: vec![DataRow {
            temp: Some(12),
            x: Some(34),
            y: Some(56),
        }],
    };
    assert_eq!(frame.to_string(), "temp;dx;dy\r\n12;34;56".to_string());

    let frame = DataFrame {
        rows: vec![DataRow {
            temp: None,
            x: Some(34),
            y: Some(56),
        }],
    };
    assert_eq!(frame.to_string(), "temp;dx;dy\r\nnan;34;56".to_string());
}

#[test]
fn file_to_frame() {
    let frame = DataFrame::from_path("test/test_data.csv").unwrap();
    assert_eq!(
        frame.rows,
        vec![
            DataRow {
                temp: Some(-48),
                x: Some(2),
                y: Some(-4)
            },
            DataRow {
                temp: Some(-46),
                x: Some(2),
                y: Some(-4)
            },
            DataRow {
                temp: Some(-44),
                x: Some(2),
                y: Some(-3)
            },
            DataRow {
                temp: Some(-43),
                x: Some(0),
                y: Some(-2)
            },
            DataRow {
                temp: Some(-43),
                x: Some(2),
                y: Some(-3)
            },
            DataRow {
                temp: Some(-42),
                x: Some(0),
                y: Some(-2)
            },
            DataRow {
                temp: Some(-40),
                x: Some(0),
                y: Some(-2)
            },
            DataRow {
                temp: Some(-38),
                x: Some(0),
                y: Some(-2)
            },
            DataRow {
                temp: Some(-36),
                x: Some(0),
                y: Some(-2)
            },
            DataRow {
                temp: Some(-34),
                x: Some(0),
                y: Some(-2)
            },
            DataRow {
                temp: Some(-32),
                x: Some(0),
                y: Some(-1)
            },
            DataRow {
                temp: Some(-30),
                x: Some(0),
                y: Some(-1)
            },
            DataRow {
                temp: Some(-28),
                x: Some(0),
                y: Some(-1)
            },
            DataRow {
                temp: Some(-26),
                x: Some(0),
                y: Some(-1)
            },
            DataRow {
                temp: Some(-24),
                x: Some(0),
                y: Some(-1)
            },
            DataRow {
                temp: Some(-22),
                x: Some(0),
                y: Some(-1)
            },
            DataRow {
                temp: Some(-20),
                x: Some(0),
                y: Some(-1)
            },
            DataRow {
                temp: Some(-18),
                x: Some(0),
                y: Some(-1)
            },
            DataRow {
                temp: Some(-16),
                x: Some(0),
                y: Some(-1)
            },
            DataRow {
                temp: Some(-14),
                x: Some(0),
                y: Some(-1)
            },
            DataRow {
                temp: Some(-12),
                x: Some(0),
                y: Some(-1)
            },
            DataRow {
                temp: Some(-10),
                x: Some(0),
                y: Some(-1)
            },
            DataRow {
                temp: Some(-8),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(-6),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(-4),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(-2),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(0),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(2),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(4),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(6),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(8),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(10),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(12),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(14),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(16),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(18),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(20),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(23),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(25),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(27),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(29),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(31),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(33),
                x: Some(-1),
                y: Some(0)
            },
            DataRow {
                temp: Some(35),
                x: Some(-1),
                y: Some(0)
            },
            DataRow {
                temp: Some(37),
                x: Some(-1),
                y: Some(0)
            },
            DataRow {
                temp: Some(39),
                x: Some(-1),
                y: Some(0)
            },
            DataRow {
                temp: Some(41),
                x: Some(-1),
                y: Some(0)
            },
            DataRow {
                temp: Some(43),
                x: Some(-1),
                y: Some(0)
            },
            DataRow {
                temp: Some(45),
                x: Some(-1),
                y: Some(0)
            },
            DataRow {
                temp: Some(47),
                x: Some(-2),
                y: Some(0)
            },
            DataRow {
                temp: Some(49),
                x: Some(-2),
                y: Some(0)
            },
            DataRow {
                temp: Some(51),
                x: Some(-2),
                y: Some(0)
            },
            DataRow {
                temp: Some(53),
                x: Some(-2),
                y: Some(0)
            },
            DataRow {
                temp: Some(55),
                x: Some(-2),
                y: Some(0)
            },
            DataRow {
                temp: Some(57),
                x: Some(-2),
                y: Some(0)
            },
            DataRow {
                temp: Some(59),
                x: Some(-2),
                y: Some(0)
            },
            DataRow {
                temp: Some(61),
                x: Some(-2),
                y: Some(0)
            },
            DataRow {
                temp: Some(63),
                x: Some(-2),
                y: Some(0)
            },
            DataRow {
                temp: Some(65),
                x: Some(-3),
                y: Some(0)
            },
            DataRow {
                temp: Some(67),
                x: Some(-3),
                y: Some(0)
            }
        ]
    );
    let frame = DataFrame::from_path("test/test_data_auto_model.txt").unwrap();
    assert_eq!(
        frame.rows,
        vec![
            DataRow {
                temp: Some(-50),
                x: Some(2),
                y: Some(-4)
            },
            DataRow {
                temp: Some(-44),
                x: Some(2),
                y: Some(-3)
            },
            DataRow {
                temp: Some(-38),
                x: Some(0),
                y: Some(-2)
            },
            DataRow {
                temp: Some(-32),
                x: Some(0),
                y: Some(-1)
            },
            DataRow {
                temp: Some(-26),
                x: Some(0),
                y: Some(-1)
            },
            DataRow {
                temp: Some(-20),
                x: Some(0),
                y: Some(-1)
            },
            DataRow {
                temp: Some(-14),
                x: Some(0),
                y: Some(-1)
            },
            DataRow {
                temp: Some(-8),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(-2),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(4),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(10),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(16),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(22),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(28),
                x: Some(0),
                y: Some(0)
            },
            DataRow {
                temp: Some(34),
                x: Some(-1),
                y: Some(0)
            },
            DataRow {
                temp: Some(40),
                x: Some(-1),
                y: Some(0)
            },
            DataRow {
                temp: Some(46),
                x: Some(-1),
                y: Some(0)
            },
            DataRow {
                temp: Some(52),
                x: Some(-2),
                y: Some(0)
            },
            DataRow {
                temp: Some(58),
                x: Some(-2),
                y: Some(0)
            },
            DataRow {
                temp: Some(64),
                x: Some(-2),
                y: Some(0)
            },
            DataRow {
                temp: Some(70),
                x: Some(-3),
                y: Some(0)
            }
        ]
    );
}

#[test]
fn calc_auto_model() {
    DataFrame::from_path("test/test_data.csv")
        .unwrap()
        .calc()
        .save_file("test/test_data_auto_model.txt")
        .unwrap();
}

#[test]
fn test_auto_model() {
    let test_auto_model = DataFrame::from_path("test/test_data.csv").unwrap().calc();
    let auto_model = DataFrame::from_path("test/test_data_auto_model.txt").unwrap();
    assert_eq!(test_auto_model.rows, auto_model.rows);
}
