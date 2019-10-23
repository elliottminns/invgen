extern crate clap;

use clap::{Arg, ArgMatches, App, SubCommand};

pub fn client_commands<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("client")
        .about("Command to manage invgen clients")
        .subcommand(add_command())
}

fn add_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("add")
        .about("Adds a new client to invgen")
        .arg(
            Arg::with_name("name")
                .index(1)
                .help("The client name")
                .required(true)
                .value_name("NAME")
        )
}

pub fn run(matches: &ArgMatches) -> Result<String, String> {
    match matches.subcommand() {
        ("add", Some(x)) => add(x),
        _ => Err(String::from("No command found"))
    }
}

fn add(matches: &ArgMatches) -> Result<String, String> {
    let name = &matches.value_of("name").expect("Name is required for client creation");
    Ok(format!("Added client \"{}\"", name))
}
