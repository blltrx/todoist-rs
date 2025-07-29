pub mod api;
mod queue;
mod sync;

pub struct LocalCache {
    client: api::Api,
    tasks: Vec<api::Task>,
    queue: queue::Queue<queue::Action>,
}

impl LocalCache {
    pub fn new(token: String) -> LocalCache {
        //! Returns a newly created LocalCache struct ready for speaking to the app
        //! and initiates the API client for external sync to server.
        //! Consumes a string for the todoist API token.
        let client = api::Api::new(token);
        // todo!(); // start sync thread with tx,rx
        LocalCache {
            client,
            tasks: Vec::new(),
            queue: queue::Queue::new(),
        }
    }

    pub fn force_sync(&mut self) -> Result<(), u16> {
        todo!();
    }

    pub fn get_tasks(&mut self) -> Result<Vec<api::Task>, u16> {
        todo!();
    }

    pub fn complete_task(&mut self, task: api::Task) -> Result<(), u16> {
        todo!();
    }

    pub fn add_task(&mut self, quick_add: String) -> Result<api::Task, u16> {
        todo!();
    }

    pub fn edit_task(&mut self, task: api::Task) -> Result<(), u16> {
        todo!();
    }
}
