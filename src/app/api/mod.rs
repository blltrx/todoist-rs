use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
pub mod request;

#[derive(Clone, Serialize, Deserialize)]
pub struct Due {
    pub date: String,
    pub string: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub content: String,
    pub description: String,
    pub checked: bool,
    pub priority: u8,
    pub due: Option<Due>,
}

#[derive(Clone, Serialize, Deserialize)]
struct SyncResponse {
    full_sync: bool,
    items: Vec<Task>,
    sync_token: String,
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

    pub fn complete_task(&self, task: &Task) {
        let command = format!(
            "[{{
                \"type\": \"item_close\", 
                \"uuid\": \"{}\", 
                \"args\": {{
                    \"id\": \"{}\" 
                }}
            }}]",
            uuid::Uuid::new_v4(),
            task.id
        );
        let url = "https://api.todoist.com/sync/v9/sync".to_string();
        let _ = request::sync_post(
            &self.token,
            &self.client,
            url,
            &[(String::from("commands"), command)],
        );
    }

    pub fn quick_add(&self, quick: String) {
        let url = "https://api.todoist.com/sync/v9/quick/add".to_string();
        let _ = request::sync_post(
            &self.token,
            &self.client,
            url,
            &[(String::from("text"), quick)],
        );
    }

    pub fn get_tasks(&self) -> Vec<Task> {
        let url = "https://api.todoist.com/sync/v9/sync".to_string();
        let res = request::sync_post(
            &self.token,
            &self.client,
            url,
            &[
                (String::from("sync_token"), String::from("*")),
                (String::from("resource_types"), String::from("[\"items\"]")),
            ],
        );

        let mut task_list = serde_json::from_str::<SyncResponse>(&res.unwrap())
            .expect("json does not match SyncResponse struct")
            .items;
        task_list.sort_by_key(|task| task.priority);
        task_list.reverse();

        task_list
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
