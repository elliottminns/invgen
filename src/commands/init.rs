extern crate clap;

use clap::{Arg, ArgMatches, App, SubCommand};

pub fn init_commands<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("init")
        .about("command to initialise an invgen directory")
        .arg(
            Arg::with_name("path")
                .required(false)
                .help("The path to store the invgen data")
                .value_name("PATH")
                .default_value("~/.invgen")
        )
}

pub fn run(matcher: &ArgMatches) -> Result<String, String> {
    let path = matcher.value_of("path").expect("Path must exist");
    Ok(format!("Initialising at {}", path))
}
