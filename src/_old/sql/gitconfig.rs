use crate::library::sql::regex_last_dir;
use ini::Ini;

pub fn read_git_config(path: String) -> String {
    let config_path = format!("{}{}", path, ".git/config").to_owned();
    // dbg!(&config_path);
    let config = Ini::load_from_file(config_path).expect(path.as_str());
    let url = config
        .get_from(Some("remote \"origin\""), "url")
        .unwrap()
        .to_owned();

    url
}

pub fn guess_git_owner(path: String) -> String {
    let url = read_git_config(path);
    let result = regex_last_dir(url);
    // dbg!(&result);
    result
}
