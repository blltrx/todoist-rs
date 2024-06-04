use http::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
/// Represents due json object
pub struct Due {
    pub date: String,
    pub string: String,
}

#[derive(Clone, Serialize, Deserialize)]
/// Task representation: DOES NOT YET IMPLEMENT LABELS
pub struct Task {
    pub id: String,
    pub content: String,
    pub description: String,
    pub checked: bool,
    pub priority: u8,
    pub due: Option<Due>,
}

#[derive(Clone, Serialize, Deserialize)]
/// Represents json returned from a sync request
struct SyncResponse {
    full_sync: bool,
    items: Vec<Task>,
    sync_token: String,
}

/// API client struct
pub struct Api {
    token: String,
    client: reqwest::blocking::Client,
}

impl Api {
    fn post(&self, url: String, form_fields: &[(String, String)]) -> Result<String, u16> {
        let response = match self
            .client
            .post(url)
            .form(form_fields)
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .send()
        {
            Ok(res) => res,
            Err(_) => return Err(3),
        };
        match response.status().as_u16() {
            200 => Ok(response.text().unwrap()),
            _ => Err(response.status().as_u16()),
        }
    }

    pub fn new(token: String) -> Api {
        //! Create new API client struct. Consumes an API auth token as a String
        Api {
            token,
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn complete_task(&self, task: &Task) {
        //! Mark task as complete based on Task object
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
        let _ = self.post(url, &[(String::from("commands"), command)]);
    }

    pub fn quick_add(&self, quick: String) {
        //! Create a new task using the quick add method, allowing for shorthand for due date, label, and priority
        let url = "https://api.todoist.com/sync/v9/quick/add".to_string();
        let _ = self.post(url, &[(String::from("text"), quick)]);
    }

    pub fn get_tasks(&self) -> Result<Vec<Task>, u16> {
        //! Get a vector of all tasks
        let url = "https://api.todoist.com/sync/v9/sync".to_string();
        let res = self.post(
            url,
            &[
                (String::from("sync_token"), String::from("*")),
                (String::from("resource_types"), String::from("[\"items\"]")),
            ],
        );
        let mut task_list = match serde_json::from_str::<SyncResponse>(&res?) {
            Ok(syncresponse) => syncresponse.items,
            Err(_) => return Err(2),
        };
        task_list.sort_by_key(|task| task.priority);
        task_list.reverse();

        Ok(task_list)
    }
}

impl Task {
    pub fn to_info_string(&self) -> String {
        //! Produce a string suitable for the task list based on a task object
        format!(
            "!!{} - {}\n\n{}\n\n---\n{}",
            self.priority,
            self.content,
            self.description,
            match &self.due {
                None => String::from("not due"),
                Some(x) => x.date.to_owned(),
            },
        )
    }

    pub fn to_list_string(&self, width: u16) -> String {
        //! Produce a string suitable for the infomation pane based on a task object
        let content_length: usize = (width as f32 * 0.6).round() as usize;
        let (mut spacer_length, overflow) =
            usize::overflowing_sub(width as usize, content_length + 17);
        if overflow {
            spacer_length = 2;
        };
        format!(
            "{:content_length$}{:spacer_length$}{:10}  {:1}",
            self.content
                .chars()
                .take(content_length)
                .collect::<String>(),
            " ",
            match &self.due {
                None => String::from("not due"),
                Some(x) => x.date.to_owned(),
            },
            self.priority,
        )
    }
}
