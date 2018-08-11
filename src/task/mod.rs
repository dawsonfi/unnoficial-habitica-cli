mod printer;

use self::printer::Printer;
use config::read_config;
use config::{API_KEY, API_USER};
use habitica_rust_client::task::api_credentials::ApiCredentials;
use habitica_rust_client::task::habitica_client::HabiticaClient;
use habitica_rust_client::task::tasks::Task;

pub fn get_todo_tasks() {
    let api_credentials = ApiCredentials::new(read_config(API_USER), read_config(API_KEY));
    let habitica_client = HabiticaClient::new(api_credentials);

    let tasks = habitica_client.get_all_tasks().unwrap();
    let todo_tasks = tasks
        .get_tasks()
        .into_iter()
        .filter(|task| task.get_task_type() == &Some("todo".to_string()))
        .collect();

    print_tasks(todo_tasks);
}

pub fn print_tasks(tasks: Vec<&Task>) {
    tasks.into_iter().for_each(Task::print)
}
