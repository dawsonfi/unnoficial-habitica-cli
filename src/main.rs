extern crate clap;
extern crate habitica_rust_client;

mod task;

use clap::{App, SubCommand};

fn main() {
    let matches = App::new("Habitica")
        .version("0.1")
        .about("Unnoficial Habitica CLI")
        .author("Dawson Israel")
        .subcommand(SubCommand::with_name("config").about("Configure Habitica"))
        .subcommand(
            SubCommand::with_name("task")
                .about("List taks")
                .subcommand(SubCommand::with_name("todo").about("List ToDo's tasks")),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("task") {
        if let Some(_matches) = matches.subcommand_matches("todo") {
            task::process_tasks_todo_command();
        }
    }
}
