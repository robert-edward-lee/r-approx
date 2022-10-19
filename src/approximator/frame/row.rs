use std::{error::Error, fmt::Display};

/// Структура данных для отдельной строки csv файла
#[derive(Default, Debug, PartialEq, Eq)]
pub struct DataRow {
    pub temp: Option<i32>,
    pub x: Option<i32>,
    pub y: Option<i32>,
}

impl Display for DataRow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.temp {
            Some(val) => write!(f, "{};", val)?,
            None => write!(f, "nan;")?,
        };

        match self.x {
            Some(val) => write!(f, "{};", val)?,
            None => write!(f, "nan;")?,
        };

        match self.y {
            Some(val) => write!(f, "{}", val)?,
            None => write!(f, "nan")?,
        };

        Ok(())
    }
}

impl DataRow {
    pub fn from_str(string: &str) -> Result<Self, Box<dyn Error>> {
        let chunks: Vec<&str> = string.split(';').collect();
        if chunks.len() != 3 {
            Err(format!("Error! Invalid row length {}", chunks.len()))?
        }

        let mut item = Self::default();

        let temp = chunks[0].parse::<i32>();
        item.temp = temp.is_ok().then(|| temp.unwrap());

        let x = chunks[1].parse::<i32>();
        item.x = x.is_ok().then(|| x.unwrap());

        let y = chunks[2].parse::<i32>();
        item.y = y.is_ok().then(|| y.unwrap());

        Ok(item)
    }
}

#[test]
fn string_to_row() {
    let row = DataRow::from_str("12;34;56").unwrap();
    assert_eq!(
        row,
        DataRow {
            temp: Some(12),
            x: Some(34),
            y: Some(56)
        }
    );

    let row = DataRow::from_str("kek;34;56").unwrap();
    assert_eq!(
        row,
        DataRow {
            temp: None,
            x: Some(34),
            y: Some(56)
        }
    );

    let row = DataRow::from_str("kek;;").unwrap();
    assert_eq!(
        row,
        DataRow {
            temp: None,
            x: None,
            y: None
        }
    );
}

#[test]
fn row_to_string() {
    let row = DataRow {
        temp: Some(12),
        x: Some(34),
        y: Some(56),
    };
    assert_eq!("12;34;56", row.to_string());

    let row = DataRow {
        temp: None,
        x: Some(34),
        y: Some(56),
    };
    assert_eq!("nan;34;56", row.to_string());

    let row = DataRow {
        temp: None,
        x: None,
        y: None,
    };
    assert_eq!("nan;nan;nan", row.to_string());
}
