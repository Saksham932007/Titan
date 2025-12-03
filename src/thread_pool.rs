use std::thread;

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {
            // Placeholder: will add job execution logic in next commit
        });

        Worker { id, thread }
    }
}
