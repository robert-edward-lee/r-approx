use clap::{Arg, ArgAction, Command};
use std::error::Error;

mod thermo_model;
use thermo_model::ThermoModel;

mod polynomial;

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
                .conflicts_with("predict")
                .action(ArgAction::Append)
                .num_args(1..=2),
        )
        .arg(
            Arg::new("serial_number")
                .short('s')
                .long("serial_number")
                .value_name("SERIAL NUMBER")
                .required(false),
        )
        .get_matches();

    let mut recalc = false;
    let mut path: &str = "";
    let mut optional_path: Option<&str> = None;

    if args.contains_id("predict") {
        recalc = true;
        path = args
            .get_one::<String>("predict")
            .ok_or("Invalid argument")?;
    }

    if args.contains_id("validate") {
        let values: Vec<&str> = args
            .get_many::<String>("validate")
            .unwrap_or_default()
            .map(|x| x.as_str())
            .collect();

        path = values[0];
        if values.len() == 2 {
            optional_path = Some(values[1]);
        }
    }

    let mut model = ThermoModel::from_path(path, recalc, optional_path)?;

    if args.contains_id("serial_number") {
        model.with_serial_number(args.get_one::<String>("serial_number").unwrap());
        model.ct()?;
    }

    model.plot()?;
    model.md()?;

    Ok(())
}
