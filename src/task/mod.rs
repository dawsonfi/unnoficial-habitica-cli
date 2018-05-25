use habitica_rust_client::task::api_credentials::ApiCredentials;
use habitica_rust_client::task::habitica_client::HabiticaClient;
use std::env;

pub fn process_tasks_todo_command() {
    let api_credentials = ApiCredentials::new(
        env::var("API_USER").unwrap().to_string(),
        env::var("API_KEY").unwrap().to_string());
    let habitica_client = HabiticaClient::new(api_credentials);

    let tasks = habitica_client.get_all_tasks().unwrap();
    tasks.get_tasks()
        .into_iter()
        .for_each(|task| println!("{}", task.get_text()));
}
