extern crate clap;

use clap::App;

mod commands;

fn main() {
    let matches = App::new("invgen")
        .version("1.0")
        .author("Elliott Minns <elliott.minns@me.com>")
        .about("An invoice and time management tool")
        .subcommand(commands::init::init_commands())
        .subcommand(commands::client::client_commands())
        .get_matches();

    let result = match matches.subcommand() {
        ("init", Some(sub)) => commands::init::run(sub),
        ("client", Some(sub)) => commands::client::run(sub),
        _ => Err(String::from("No command available"))
    };

    match result {
        Err(x) => println!("{}", x),
        Ok(x) => println!("{}", x)
    }
}
