// #![feature(result_cloned)]

// TODO
// initialize db from schema script

mod other;
fn main() {
    other::initialize::initialize().unwrap();
    // other::viewer::viewer().unwrap();
    other::request::request_string();

    // other::sql::print_project();
    // other::sql::write_tmp_to_project();
}
