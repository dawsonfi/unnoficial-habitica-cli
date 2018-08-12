extern crate colored;

use self::colored::*;
use habitica_rust_client::task::tasks::Task;

pub trait Printer {
    fn print(&self);
}

impl Printer for Task {
    fn print(&self) {

        let completed = match self.is_completed() {
            Some(_) => "true".green(),
            None => "false".red(),
        };

        println!(
            "| {:5} | [{:5}] - {}",
            self.get_task_type().as_ref().unwrap().cyan(),
            completed,
            self.get_text().as_ref().unwrap()
        );
    }
}

pub fn print_tasks(tasks: Vec<&Task>) {
    tasks.into_iter().for_each(Task::print)
}