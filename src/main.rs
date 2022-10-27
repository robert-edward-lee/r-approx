use clap::{Arg, Command};
use std::error::Error;

mod thermo_model;
use thermo_model::ThermoModel;

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

    let mut model = ThermoModel::from_path(path, recalc)?;

    if args.contains_id("serial_number") {
        model.with_serial_number(args.get_one::<String>("serial_number").unwrap());
        model.ct()?;
    }

    model.plot()?;
    model.md()?;

    Ok(())
}
