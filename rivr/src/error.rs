#[derive(Debug)]
pub enum Error {
    Rendering { error: Box<::std::error::Error> },
}
