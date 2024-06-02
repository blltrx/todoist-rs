use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
pub mod request;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Due {
    pub date: String,
    pub string: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: String,
    pub content: String,
    pub description: String,
    pub is_completed: bool,
    pub priority: u8,
    pub due: Option<Due>,
}

#[allow(dead_code)]
pub struct Api {
    token: String,
    client: Client,
}

impl Api {
    pub fn new(token: String) -> Api {
        Api {
            token,
            client: Client::new(),
        }
    }

    pub fn get_tasks(&self) -> Vec<Task> {
        let json = request::get(
            &self.token,
            &self.client,
            "https://api.todoist.com/rest/v2/tasks",
        );
        let mut task_list: Vec<Task> = serde_json::from_str::<Vec<Task>>(&json).unwrap();
        task_list.sort_by_key(|task| task.priority);
        task_list.reverse();

        task_list
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
        let _ = request::post(&self.token, &self.client, url, String::new());
    }

    pub fn quick_add(&self, quick: String) {
        let url = String::from("https://api.todoist.com/sync/v9/quick/add");
        let _ = request::post_quickadd(&self.token, &self.client, url, quick);
    }
}

pub fn tasklist_to_strings(tasklist: &[Task], width: u16) -> Vec<String> {
    let content_length: usize = (width as f32 * 0.6).round() as usize;
    let (mut spacer_length, overflow) = usize::overflowing_sub(width as usize, content_length + 17);
    if overflow {
        spacer_length = 2;
    };
    tasklist
        .iter()
        .map(|task| {
            format!(
                "{:content_length$}{:spacer_length$}{:10}  {:1}",
                task.content
                    .chars()
                    .take(content_length)
                    .collect::<String>(),
                " ",
                match &task.due {
                    None => String::from("not due"),
                    Some(x) => x.date.to_owned(),
                },
                task.priority,
            )
        })
        .collect()
}

pub fn task_to_string(task: &Task) -> String {
    format!(
        "!!{} - {}\n\n{}\n\n---\n{}",
        task.priority,
        task.content,
        task.description,
        match &task.due {
            None => String::from("not due"),
            Some(x) => x.date.to_owned(),
        },
    )
}
