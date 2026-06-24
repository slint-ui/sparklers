pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Invalid feed URL: {0}")]
    InvalidFeedUrl(String),

    #[error("Sparkle initialization failed: {0}")]
    SparkleInit(String),

    #[error("Updater not ready")]
    UpdaterNotReady,
}
