use crate::library::sql::regex_last_dir;
use ini::Ini;
pub fn read_git_config(path: String) -> String {
    let config = Ini::load_from_file(format!("{}{}", path, ".git/config").to_owned()).unwrap();
    let url = config
        .get_from(Some("remote \"origin\""), "url")
        .unwrap()
        .to_owned();

    url
    // println!("{}", contents)
}

pub fn guess_git_owner(path: String) -> String {
    let url = read_git_config(path);
    regex_last_dir(url)
}
