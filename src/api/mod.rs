use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
pub mod request;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Due {
    date: String,
    string: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Task {
    id: String,
    content: String,
    description: String,
    is_completed: bool,
    priority: u8,
    due: Option<Due>,
}

#[allow(dead_code)]
pub struct Api {
    token: String,
    client: Client,
}

impl Api {
    pub fn new(token: String) -> Api {
        return Api {
            token,
            client: Client::new(),
        };
    }

    pub fn get_tasks(&self) -> Vec<Task> {
        let json = request::get(
            &self.token,
            &self.client,
            "https://api.todoist.com/rest/v2/tasks",
        );
        let task_list: Vec<Task> = serde_json::from_str::<Vec<Task>>(&json).unwrap();
        return task_list;
    }

    pub fn delete_task(&self, task: &Task) {
        let url = format!("https://api.todoist.com/rest/v2/tasks/{}", task.id);
        request::delete(&self.token, &self.client, url);
    }

    fn add_task(&self, task: Task) -> Task {
        let json = format!(
            "{{
            \"content\": \"{}\",
            \"due_string\": \"{}\",
            \"due_lang\" : \"en\",
            \"description\": \"{}\",
            \"priority\": {}
            }}",
            task.content,
            <Option<Due> as Clone>::clone(&task.due).unwrap().string,
            task.description,
            task.priority
        );
        let response = request::post(
            &self.token,
            &self.client,
            String::from("https://api.todoist.com/rest/v2/tasks"),
            json,
        )
        .unwrap();
        let task: Task = serde_json::from_str(&response).unwrap();
        task
    }

    pub fn new_task(
        &self,
        content: String,
        description: String,
        mut priority: u8,
        due_string: String,
    ) {
        if priority > 4 {
            priority = 4
        };
        let task = Task {
            id: String::from("placeholder"),
            content,
            description,
            is_completed: false,
            priority,
            due: Some(Due {
                date: String::from("placeholder"),
                string: due_string,
            }),
        };
        self.add_task(task);
    }

    pub fn complete_task(&self, task: &Task) {
        let url = format!("https://api.todoist.com/rest/v2/tasks/{}/close", task.id);
        let res = request::post(&self.token, &self.client, url, String::new());
        match res {
            Some(x) => println!("res: {}", x),
            None => return,
        }
    }
}
