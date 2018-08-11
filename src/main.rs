extern crate clap;
extern crate habitica_rust_client;

mod config;
mod task;

use clap::Arg;
use clap::ArgMatches;
use clap::{App, SubCommand};
use config::{API_KEY, API_USER};

const CONFIG: &str = "config";
const TASKS: &str = "tasks";
const TODO: &str = "todo";

fn main() {
    let matches = App::new("Habitica")
        .version("0.1")
        .about("Unnoficial Habitica CLI")
        .author("Dawson Israel")
        .subcommand(
            SubCommand::with_name(CONFIG)
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
                        .required(true),
                ])
                .about("Configure Habitica"),
        )
        .subcommand(
            SubCommand::with_name(TASKS)
                .about("List taks")
                .subcommand(SubCommand::with_name(TODO).about("List ToDo's tasks")),
        )
        .get_matches();

    process_matches(matches);
}

fn process_matches(matches: ArgMatches) {
    if let Some(config) = matches.subcommand_matches(CONFIG) {
        config::set_config(
            config.value_of(API_USER).unwrap(),
            config.value_of(API_KEY).unwrap(),
        );
    }

    if let Some(task) = matches.subcommand_matches(TASKS) {
        if let Some(_) = task.subcommand_matches(TODO) {
            task::get_todo_tasks();
        }
    }
}
