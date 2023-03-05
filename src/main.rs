mod initialize;
mod request;
mod viewer;
fn main() {
    initialize::initialize();
    // viewer::viewer();
    request::request();
}
