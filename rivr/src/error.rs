#[derive(Debug)]
pub enum Error {
    Rendering(RenderingError),
}

#[derive(Debug)]
pub enum RenderingError {
    /// A panel was too large for the renderer. This likely means you've got a Max size on a panel
    /// without anything else constraining its size.
    PanelTooLarge,
    Other(Box<::std::error::Error>)
}
