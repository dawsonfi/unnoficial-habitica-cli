extern crate clap;
extern crate habitica_rust_client;

mod task;
mod config;

use clap::{App, SubCommand};
use clap::ArgMatches;
use clap::Arg;
use config::{API_USER, API_KEY};

fn main() {
    let matches = App::new("Habitica")
        .version("0.1")
        .about("Unnoficial Habitica CLI")
        .author("Dawson Israel")
        .subcommand(SubCommand::with_name("config")
            .args(&[
                Arg::with_name(API_USER)
                    .short("u")
                    .long("user")
                    .help("sets the api user")
                    .takes_value(true)
                    .required(true),
                Arg::with_name(API_KEY)
                    .short("k")
                    .long("key")
                    .help("sets the api key")
                    .takes_value(true)
                    .required(true)
            ]).about("Configure Habitica"))
        .subcommand(
            SubCommand::with_name("task")
                .about("List taks")
                .subcommand(SubCommand::with_name("todo").about("List ToDo's tasks")),
        ).get_matches();

    process_matches(matches);
}

fn process_matches(matches: ArgMatches) {

    if let Some(config) = matches.subcommand_matches("config") {
        config::set_config(config.value_of(API_USER).unwrap(),
                           config.value_of(API_KEY).unwrap());
    }

    if let Some(task) = matches.subcommand_matches("task") {
        if let Some(_) = task.subcommand_matches("todo") {
            task::process_tasks_todo_command();
        }
    }

}
