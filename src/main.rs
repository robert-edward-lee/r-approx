use clap::{Arg, Command};
use std::error::Error;
use thermo_model::ThermoModel;

mod thermo_model;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("predict")
                .short('p')
                .long("predict")
                .value_name("CSV FILE")
                .required_unless_present("validate"),
        )
        .arg(
            Arg::new("validate")
                .short('v')
                .long("validate")
                .value_name("CSV FILE")
                .required_unless_present("predict")
                .conflicts_with("predict"),
        )
        .arg(
            Arg::new("serial_number")
                .short('s')
                .long("serial_number")
                .value_name("SERIAL NUMBER")
                .required(false),
        )
        .get_matches();

    let recalc = args.contains_id("predict");

    let binding = "Something goes wrong".to_string();
    let path = args
        .get_one::<String>("predict")
        .unwrap_or_else(|| args.get_one::<String>("validate").unwrap_or(&binding));

    let binding = String::default();
    let serial_number = args.get_one::<String>("serial_number").unwrap_or(&binding);

    let mut model = ThermoModel::from_path(path, recalc)?;
    model.with_serial_number(serial_number);

    model.plot(serial_number)?;
    model.md()?;
    model.ct()?;

    Ok(())
}
