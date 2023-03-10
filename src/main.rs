// #![feature(result_cloned)]

mod other;
fn main() {
    other::initialize::initialize().unwrap();
    other::viewer::viewer().unwrap();
    // other::request::request();
    // other::sql::print_project();
    // other::sql::write_tmp_to_project();
}
