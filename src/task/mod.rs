mod printer;

use config::read_config;
use config::{API_KEY, API_USER};
use habitica_rust_client::task::api_credentials::ApiCredentials;
use habitica_rust_client::task::habitica_client::HabiticaClient;
use habitica_rust_client::task::tasks::Tasks;
use habitica_rust_client::task::tasks::Task;

pub fn print_all_tasks() {
    let tasks = get_all_tasks();
    let mut all_tasks = tasks.get_tasks()
        .into_iter()
        .collect::<Vec<&Task>>();

    all_tasks.sort_by_key(|task| task.get_task_type().as_ref().unwrap());

    printer::print_tasks(all_tasks);
}

pub fn print_filtered_tasks(task_type: &str) {
    let tasks = get_all_tasks();
    let todo_tasks = tasks
        .get_tasks()
        .into_iter()
        .filter(|task| task.get_task_type() == &Some(task_type.to_lowercase().to_string()))
        .collect();

    printer::print_tasks(todo_tasks);
}

fn get_all_tasks() -> Tasks {
    get_client().get_all_tasks().unwrap()
}

fn get_client() -> HabiticaClient {
    let api_credentials = ApiCredentials::new(read_config(API_USER), read_config(API_KEY));

    HabiticaClient::new(api_credentials)
}