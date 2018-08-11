mod printer;

use habitica_rust_client::task::api_credentials::ApiCredentials;
use habitica_rust_client::task::habitica_client::HabiticaClient;
use habitica_rust_client::task::tasks::Task;
use config::{API_USER, API_KEY};
use config::read_config;
use self::printer::Printer;

pub fn process_tasks_todo_command() {
    let api_credentials = ApiCredentials::new(
        read_config(API_USER),
        read_config(API_KEY));
    let habitica_client = HabiticaClient::new(api_credentials);

    let tasks = habitica_client.get_all_tasks().unwrap();
    let todo_tasks = tasks.get_tasks()
        .into_iter()
        .filter(|task| task.get_task_type() == &Some("todo".to_string()))
        .collect();

    print_tasks(todo_tasks);
}

pub fn print_tasks(tasks: Vec<&Task>) {
    tasks.into_iter()
         .for_each(Task::print)
}