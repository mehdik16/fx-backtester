use crate::data_loaders::data_loader::DataLoader;
use crate::error::DataLoaderError;
use crate::types::DataRecord;
use crate::types::{Ohlcv, Tick};
use std::path::Path;

pub struct CsvDataLoader {}

impl DataLoader for CsvDataLoader {
    fn load_data(&self, file_path: &Path) -> Result<Vec<DataRecord>, DataLoaderError> {
        let mut reader = csv::Reader::from_path(file_path)?;
        let headers = reader.headers()?.clone();

        let records = if headers.iter().any(|h| h == "open") {
            // Assume OHLCV format
            reader
                .deserialize::<Ohlcv>()
                .map(|result| result.map(DataRecord::Ohlcv))
                .collect::<Result<Vec<_>, _>>()?
        } else if headers.iter().any(|h| h == "bid") {
            // Assume Tick format
            reader
                .deserialize::<Tick>()
                .map(|result| result.map(DataRecord::Tick))
                .collect::<Result<Vec<_>, _>>()?
        } else {
            return Err(DataLoaderError::UnsupportedFormat);
        };

        Ok(records)
    }
}
