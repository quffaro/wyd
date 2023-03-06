mod initialize;
mod request;
mod viewer;
fn main() {
    /// this populates the tmp table with config files
    initialize::initialize();
    viewer::viewer();
    // request::request();
}
