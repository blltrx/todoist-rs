use crate::cache::api;

pub enum ApiAction {
    Get,
    QuickAdd,
    Complete,
    Edit,
}

pub struct Action {
    sync: Option<String>,
    task: Option<api::Task>,
    quick_add: Option<String>,
    r#type: ApiAction,
}

pub struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { queue: Vec::new() }
    }

    pub fn enqueue(&mut self, item: T) {
        self.queue.push(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.queue.first()
    }
}
