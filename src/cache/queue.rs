use crate::cache::api;

pub enum ApiAction {
    Get,
    QuickAdd,
    Complete,
    Edit
}

pub struct Action{
    sync: Option<String>,
    task: Option<api::Task>,
    quick_add: Option<String>,
    type: ApiAction
}

pub struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() {
        todo!();
    }

    pub fn enqueue(item: T) {
        todo!();
    }

    pub fn dequeue() -> T {
        todo!();
    }
}

