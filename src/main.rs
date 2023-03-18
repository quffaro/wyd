// TODO
// initialize db from schema script
// add priority to project
use git2::Repository;
use std::env;

mod library;
fn main() {
    library::viewer::viewer();
    // library::request::request_string();
    // let path = env::current_dir().unwrap().display().to_string();
    // match git2::Repository::discover(path) {
    //     Ok(repo) => {
    //         println!("{:#?}", repo.workdir().unwrap().to_str().unwrap());
    //     }
    //     _ => {}
    // }
}
