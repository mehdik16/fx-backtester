use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataLoaderError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("CSV Error: {0}")]
    Csv(#[from] csv::Error),
    #[error("Unsupported data format")]
    UnsupportedFormat,
}
