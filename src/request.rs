use crate::app::structs::config::load_config;
// use crate::json::project::{read_project_repos, update_project_last_commit};
use crate::{app::structs::projects::Project, home_path, PATH_DB, PATH_PAT};
use reqwest::{header, ClientBuilder, Result};
use serde::Deserialize;
use serde_json::json;
use std::fs;
use tokio;

#[derive(Deserialize, Debug)]
struct Commit {
    body: String,
}

#[tokio::main]
pub async fn request_commit() -> Result<()> {
    let projects = vec![];
    // read_project_repos().unwrap();

    let config = load_config();
    // println!("{:#?}", &config);
    match config {
        Some(conf) => {
            for p in projects {
                let request = request(&conf.owner, &p)
                    .await?
                    .as_str()
                    .and_then(|x| Some(x.to_owned()));
                // println!("{:#?}", request);
                // update_project_last_commit(&p, request.unwrap());
            }
        }
        None => (),
    }

    Ok(())
}

pub async fn request(owner: &String, project: &Project) -> Result<serde_json::Value> {
    // TODO this needs better error handling...

    match fs::read_to_string(home_path(PATH_PAT)) {
        // TODO I'd like to return an error
        Err(e) => Ok(().into()),
        Ok(mut secret) => {
            secret.pop();
            let mut headers = header::HeaderMap::new();
            let mut auth_value = header::HeaderValue::from_str(&secret).unwrap();
            auth_value.set_sensitive(true);
            headers.insert(header::AUTHORIZATION, auth_value);
            headers.insert(
                header::USER_AGENT,
                header::HeaderValue::from_str(owner).unwrap(), // TODO this should be your github user
            );
            // TODO use user_agent function?

            let request_url = format!(
                "https://api.github.com/repos/{owner}/{repo}/commits",
                owner = project.owner,
                repo = project.repo,
            );

            // let timeout = Duration::new(5, 0);
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
            let result = response
                .get(0)
                .and_then(|v| v.get("commit"))
                .and_then(|v| v.get("committer"))
                .and_then(|v| v.get("date"))
                .unwrap_or(&json!("N/A")) // TODO get last commit
                .to_owned();

            // println!("{:#?}", result);
            Ok(result)
            //UTC time, ISO 8601
        }
    }
}
