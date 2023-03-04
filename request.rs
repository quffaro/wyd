// call github to get most recent commits
// /repos/{owner}/{repo}/commits
// path parameters:
// > owner (string)
// > repo (string)

use reqwest::{header, ClientBuilder, Result};
use serde::Deserialize;
use std::fs;
use std::time::Duration;
use tokio;

#[derive(Deserialize, Debug)]
struct Commit {
    body: String,
}

#[tokio::main]
pub async fn request() -> Result<()> {
    let mut secret = fs::read_to_string("../pat/nav_pat.txt").expect("A");
    secret.pop();

    let mut headers = header::HeaderMap::new();
    let mut auth_value = header::HeaderValue::from_str(&secret).unwrap();
    auth_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_value);
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_str("request").unwrap(),
    );
    // TODO use user_agent function?

    println!("{:#?}", headers);
    let owner = "rust-lang";
    let repo = "cargo";
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/commits",
        owner = owner,
        repo = repo
    );

    // let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new()
        .default_headers(headers)
        // .timeout(timeout)
        .build()?;
    let response = client
        .get(&request_url)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("{:#?}", response);

    // if response.status().is_success() {
    //     println!("{:#?} is a user!", response.json().await?);
    // } else {
    //     println!("{:#?} is not a user!", response.status());
    // }

    Ok(())
}
