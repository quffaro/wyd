use regex::Regex;
pub fn regex_repo(path: String) -> String {
    let re = Regex::new(r"(.+)/([^/]+)").expect("AAAA");
    let caps = re.captures(&path).unwrap();

    caps.get(0).unwrap().as_str().to_string()
}

///
pub fn regex_last_dir(path: String) -> String {
    let re = Regex::new(r#".*/([^/]+)/"#).expect("AAAA");
    let caps = re.captures(&path).unwrap();

    caps.get(1).unwrap().as_str().to_string()
}
pub fn find_repo(path: String) -> String {
    let result = match git2::Repository::discover(path) {
        Ok(found) => found.workdir().unwrap().to_str().unwrap().to_string(),
        Err(didnt) => "N/A".to_owned(),
    };

    result
}
