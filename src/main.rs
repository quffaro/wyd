#![feature(result_cloned)]

mod other;
fn main() {
    other::initialize::initialize();
    other::viewer::viewer();
    // other::request::request();
    // other::sql::print_project();
}
