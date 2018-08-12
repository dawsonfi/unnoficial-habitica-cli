use habitica_rust_client::task::tasks::Task;

pub trait Printer {
    fn print(&self);
}

impl Printer for Task {
    fn print(&self) {
        println!(
            "| {} | - {}",
            self.get_task_type().as_ref().unwrap(),
            self.get_text().as_ref().unwrap()
        );
    }
}

pub fn print_tasks(tasks: Vec<&Task>) {
    tasks.into_iter().for_each(Task::print)
}