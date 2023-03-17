use crate::library::code::Project;
use crate::library::sql::update_project_last_commit;
/// call github to get most recent commits
/// /repos/{owner}/{repo}/commits
/// path parameters:
/// > owner (string)
/// > repo (string)
use reqwest::{header, ClientBuilder, Result};
use serde::Deserialize;
use std::fs;

use tokio;

#[derive(Deserialize, Debug)]
struct Commit {
    body: String,
}

#[tokio::main]
pub async fn request_string(project: &Project) -> Result<()> {
    let request = request(project).await;
    let string = request?.as_str().and_then(|x| Some(x.to_owned()));

    // println!("{}", string.unwrap());
    update_project_last_commit(string.unwrap());

    Ok(())
    // match result {
    //     Ok(x) =>
    //     {
    //         x.as_str().and_then(|x| Some(x.to_owned()))
    //         println!("{:#?}", x.as_str());
    //         // Ok(())
    //     }
    //     Err(e) => {
    //         println!("NO");
    //         println!("{:#?}", e);
    //         // Ok(())
    //     }
    // }
}

pub async fn request(project: &Project) -> Result<serde_json::Value> {
    // TODO this needs better error handling...
    let mut secret = fs::read_to_string("../pat/wyd_pat.txt").expect("A");
    secret.pop();

    let mut headers = header::HeaderMap::new();
    let mut auth_value = header::HeaderValue::from_str(&secret).unwrap();
    auth_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_value);
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_str("quffaro").unwrap(),
    );
    // TODO use user_agent function?

    // println!("{:#?}", headers);
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/commits",
        owner = project.owner,
        repo = project.repo,
    );

    // let timeout = Duration::new(5, 0);
    // let handle = SpinnerBuilder::new()
    //     .spinner(&DOTS)
    //     .text("Loading...")
    //     .start();
    let client = ClientBuilder::new()
        .default_headers(headers)
        // .timeout(timeout)
        .build()?;
    let response = client
        .get(&request_url)
        .query(&[("per_page", 1)])
        .send()
        .await
        .expect("AAA")
        .json::<serde_json::Value>()
        .await?;
    // handle.done();
    // println!("{:#?}", response);

    // TODO we need error handling!!!
    let result = response
        .get(0)
        .and_then(|v| v.get("commit"))
        .and_then(|v| v.get("committer"))
        .and_then(|v| v.get("date"))
        .unwrap();
    // TODO map as_str through option?

    // println!("{:#?}", result);
    Ok(result).cloned()
    // .cloned()
    //UTC time, ISO 8601
    // Ok(())
}
