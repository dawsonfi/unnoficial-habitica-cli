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
