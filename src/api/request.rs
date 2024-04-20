use http::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::blocking::Client;

pub fn get(token: &String, client: &Client, url: &str) -> String {
    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .unwrap();
    match response.status().as_str() {
        "200" => response.text().unwrap(),
        "400" => panic!("400 bad request"),
        "403" => panic!("invalid authentication header - check your token is valid"),
        _ => panic!(
            "unexpected response status: {}\nres: {}",
            response.status().as_str(),
            response.text().unwrap()
        ),
    }
}
pub fn delete(token: &String, client: &Client, url: String) {
    let response = client
        .delete(url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .unwrap();
    match response.status().as_str() {
        "204" => return,
        "400" => panic!("400 bad request"),
        "403" => panic!("invalid authentication header - check your token is valid"),
        _ => panic!(
            "unexpected response status: {}\nres: {}",
            response.status().as_str(),
            response.text().unwrap()
        ),
    }
}

pub fn post(token: &String, client: &Client, url: String, body: String) -> Option<String> {
    let response = client
        .post(url)
        .body(body)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, "application/json")
        .header("X-REQUEST-ID", "tetringsts")
        .send()
        .unwrap();
    match response.status().as_str() {
        "200" => return Some(response.text().unwrap()),
        "204" => return None,
        "400" => panic!("400 bad request"),
        "403" => panic!("invalid authentication header - check your token is valid"),
        _ => panic!(
            "unexpected response status: {}\nres: {}",
            response.status().as_str(),
            response.text().unwrap()
        ),
    }
}
