use http::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub struct Api {
    token: String,
    client: Client
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Due {
    date: String,
    string: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: String,
    content: String,
    description:  String,
    is_completed: bool,
    priority: u8,
    due: Option<Due>
}

impl Api {
    pub fn new(token: String) -> Api {
        return Api {
            token,
            client: Client::new()
        }
    }

    fn get(&self, url: &str) -> String {
        let response = self.client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .send()
            .unwrap();
        match response.status().as_str() {
            "200" => response.text().unwrap(),
            "403" => panic!("invalid authentication header - check your token is valid"),
            _ => panic!("unexpected response status: {}\nres: {}", response.status().as_str(), response.text().unwrap())
            
        }
    }
    fn delete(&self, url: String) {
        let response = self.client
            .delete(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .send()
            .unwrap();
        match response.status().as_str() {
            "204" => return,
            "403" => panic!("invalid authentication header - check your token is valid"),
            _ => panic!("unexpected response status: {}\nres: {}", response.status().as_str(), response.text().unwrap())
        }
    }

    fn post(&self, url: String, body: String) {
        let response = self.client
            .post(url)
            .body(body)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(CONTENT_TYPE, "application/json")
            .header("X-REQUEST-ID", "tetringsts")
            .send()
            .unwrap();
        match response.status().as_str() {
            "204" => return,
            "403" => panic!("invalid authentication header - check your token is valid"),
            _ => panic!("unexpected response status: {}\nres: {}", response.status().as_str(), response.text().unwrap())
        }
    }

    pub fn get_tasks(&self) -> Vec<Task> {
        let json = self.get("https://api.todoist.com/rest/v2/tasks");
        let task_list: Vec<Task> = serde_json::from_str::<Vec<Task>>(&json).unwrap();
        return task_list        
    }

    pub fn delete_tasks(&self, task: Task) {
        let url = format!("https://api.todoist.com/rest/v2/tasks/{}", task.id);
        self.delete(url);
    }

    fn add_task(&self, task: Task) {
        let json = format!("
            \"content\": \"{}\",
            \"due_string\": \"{}\",
            \"due_lang\" : \"en\",
            \"description\": \"{}\",
            \"priority\": {}
            ");
        self.post("https://api.todoist.com/rest/v2/tasks")
    }

    pub fn new_task(
        &self,
        content: String,
        description: String,
        mut priority: u8,
        due_string: String
    ) {
        if priority > 4 {priority = 4};
        let task = Task {
            id: String::from("placeholder"),
            content,
            description,
            is_completed: false,
            priority,
            due: Some(Due {
                date: String::from("placeholder"),
                string: due_string
            })
        };
        self.add_task(task);
        
    }

    pub fn complete_task(&self, task: Task) {
        let url = format!("https://api.todoist.com/rest/v2/tasks/{}/close", task.id);
        self.post(url, String::new());
    }
}
