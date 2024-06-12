use http::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
/// Represents due json object
pub struct Due {
    date: String,
    string: String,
}

#[derive(Clone, Serialize, Deserialize)]
/// Task representation: DOES NOT YET IMPLEMENT LABELS
pub struct Task {
    id: String,
    content: String,
    description: String,
    checked: bool,
    labels: Vec<String>,
    priority: u8,
    due: Option<Due>,
}

#[derive(Clone, Serialize, Deserialize)]
/// Represents json returned from a sync request
struct SyncResponse {
    full_sync: bool,
    items: Vec<Task>,
    sync_token: String,
}

#[derive(Clone, Serialize, Deserialize)]
/// Represents json returned from a write request
struct WriteResponse {
    sync_token: String,
    sync_status: std::collections::HashMap<String, String>,
}

/// API client struct
pub struct Api {
    token: String,
    client: reqwest::blocking::Client,
}

impl Api {
    pub fn new(token: String) -> Api {
        //! Create new API client struct. Consumes an API auth token as a String
        Api {
            token,
            client: reqwest::blocking::Client::new(),
        }
    }

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

    pub fn get_tasks(&self, sync_token: &str) -> Result<(Vec<Task>, String), u16> {
        //! Get a vector of all tasks and an updated sync token
        let url = "https://api.todoist.com/sync/v9/sync".to_string();
        let result = self.post(
            url,
            &[
                (String::from("sync_token"), String::from(sync_token)),
                (String::from("resource_types"), String::from("[\"items\"]")),
            ],
        );
        let syncresponse = match serde_json::from_str::<SyncResponse>(&result?) {
            Ok(syncresponse) => syncresponse,
            Err(_) => return Err(2),
        };
        let mut task_list = syncresponse.items;
        task_list.sort_by_key(|task| task.priority);
        task_list.reverse();

        Ok((task_list, syncresponse.sync_token))
    }

    pub fn complete_task(&self, task: &Task) -> Result<String, u16> {
        //! Mark task as complete based on Task object, returning the new sync token
        let todoist_command = format!(
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
        let result = self.post(url, &[(String::from("commands"), todoist_command)]);
        let writeresponse = match serde_json::from_str::<WriteResponse>(&result?) {
            Ok(syncresponse) => syncresponse,
            Err(_) => return Err(2),
        };
        Ok(writeresponse.sync_token)
    }

    pub fn quick_add(&self, quick: String) -> Result<Task, u16> {
        //! Create a new task using the quick add method, allowing for shorthand for due date, label, and priority, returning the added task
        let url = "https://api.todoist.com/sync/v9/quick/add".to_string();
        let result = self.post(url, &[(String::from("text"), quick)]);
        let writeresponse = match serde_json::from_str::<Task>(&result?) {
            Ok(syncresponse) => syncresponse,
            Err(_) => return Err(2),
        };
        Ok(writeresponse)
    }

    pub fn edit(&self, task: Task) -> Result<String, u16> {
        //! Mark task as complete based on Task object, returning the new sync token
        let todoist_command = format!(
            "[{{
                \"type\": \"item_update\", 
                \"uuid\": \"{}\", 
                \"args\": {{
                    \"id\": \"{}\" ,
                    \"content\": \"{}\",
                    \"description\": \"{}\" ,
                    \"labels\": [{}],
                    \"priority\": \"{}\"
                    {}
                }}
            }}]",
            uuid::Uuid::new_v4(),
            task.id,
            task.content,
            task.description,
            task.labels
                .iter()
                .map(|x| format!("\"{}\"", x))
                .collect::<Vec<String>>()
                .join(","),
            task.priority,
            match task.due {
                Some(due) => format!(
                    "
                        ,\"due\": {{
                            \"date\": \"{}\"
                        }}
                    ",
                    due.date
                ),
                None => String::new(),
            }
        );
        let url = "https://api.todoist.com/sync/v9/sync".to_string();
        let result = self.post(url, &[(String::from("commands"), todoist_command)]);
        let writeresponse = match serde_json::from_str::<WriteResponse>(&result?) {
            Ok(syncresponse) => syncresponse,
            Err(_) => return Err(2),
        };
        Ok(writeresponse.sync_token)
    }
}

impl Task {
    pub fn create_task_obj(
        id: String,
        content: String,
        description: String,
        date: String,
        labels: Vec<String>,
        priority: u8,
    ) -> Task {
        Task {
            id,
            content,
            description,
            checked: false,
            labels,
            priority,
            due: Some(Due {
                date,
                string: String::new(),
            }),
        }
    }

    pub fn to_info_string(&self) -> String {
        //! Produce a string suitable for the infomation pane based on a task object
        format!(
            "!!{} - {}\n@{}\n\n{}\n\n---\n{}",
            self.priority,
            self.content,
            self.labels.join(","),
            self.description,
            match &self.due {
                None => String::from("not due"),
                Some(x) => x.date.to_owned(),
            },
        )
    }

    pub fn to_list_string(&self, width: u16) -> String {
        //! Produce a string suitable for the task list based on a task object
        let content_length = (width as f32 * 0.6).round() as usize;
        let spacer_length = match usize::overflowing_sub(width as usize, content_length + 17) {
            (length, overflow) if !overflow => length,
            _ => 2,
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

    pub fn get_details(&self) -> (String, String, String, Vec<String>, String, u8) {
        (
            self.id.clone(),
            self.content.clone(),
            self.description.clone(),
            self.labels.clone(),
            match &self.due {
                None => String::new(),
                Some(x) => x.date.clone(),
            },
            self.priority,
        )
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}
