use http::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::blocking::Client;

pub fn sync_post(
    token: &String,
    client: &Client,
    url: String,
    form_fields: &[(String, String)],
) -> Option<String> {
    let response = client
        .post(url)
        .form(form_fields)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .send()
        .expect(
            "Could not make an API request, most lighly an issue with your internet connection",
        );
    match response.status().as_u16() {
        200 => Some(response.text().unwrap()),
        400 => panic!("400 bad request"),
        401 => panic!("invalid authentication header - check your token is valid"),
        403 => panic!("Forbidden"),
        404 => panic!("404 not found"),
        429 => panic!("rate limited"),
        500..=599 => panic!("TODOIST server had an internal error"),
        _ => panic!(
            "unexpected response status: {}\nres: {}",
            response.status().as_str(),
            response.text().unwrap()
        ),
    }
}
