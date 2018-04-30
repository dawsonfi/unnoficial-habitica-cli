extern crate clap;

use clap::{App, SubCommand};

fn main() {
    let matches = App::new("Habitica")
        .version("0.1")
        .about("Unnoficial Habitica CLI")
        .author("Dawson Israel")
        .subcommand(SubCommand::with_name("todo").about("List ToDo's taks"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("todo") {
        println!("Foi");
    }
}
