use habitica_rust_client::task::api_credentials::ApiCredentials;
use habitica_rust_client::task::habitica_client::HabiticaClient;
use habitica_rust_client::task::tasks::Task;
use std::env;

pub fn process_tasks_todo_command() {
    let api_credentials = ApiCredentials::new(
        env::var("API_USER").unwrap().to_string(),
        env::var("API_KEY").unwrap().to_string());
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
         .for_each(print_task)
}

pub fn print_task(task: &Task) {
    println!("| {} | - {}", task.get_task_type().as_ref().unwrap(), task.get_text().as_ref().unwrap());
}