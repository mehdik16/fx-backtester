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

#[cfg(test)]
mod tests {
    use crate::data_loaders::csv_loader::CsvDataLoader;
    use crate::data_loaders::data_loader::DataLoader;
    use crate::error::DataLoaderError;
    use crate::types::DataRecord;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn test_load_ohlcv_data() {
        let data =
            "timestamp,open,high,low,close,volume\n2023-01-01T00:00:00Z,1.1,1.2,1.0,1.1,100.0\n";
        let file_path = Path::new("test_ohlcv.csv");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(data.as_bytes()).unwrap();

        let data_loader = CsvDataLoader {};

        let records = data_loader.load_data(&file_path).unwrap();
        assert_eq!(records.len(), 1);
        match &records[0] {
            DataRecord::Ohlcv(ohlcv) => {
                assert_eq!(ohlcv.open, 1.1);
            }
            _ => panic!("Expected OHLCV data"),
        }

        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_load_tick_data() {
        let data = "timestamp,bid,ask\n2023-01-01T00:00:00Z,1.1,1.2\n";
        let file_path = Path::new("test_tick.csv");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(data.as_bytes()).unwrap();

        let data_loader = CsvDataLoader {};

        let records = data_loader.load_data(&file_path).unwrap();
        assert_eq!(records.len(), 1);
        match &records[0] {
            DataRecord::Tick(tick) => {
                assert_eq!(tick.bid, 1.1);
            }
            _ => panic!("Expected Tick data"),
        }

        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_load_unsupported_format() {
        let data = "timestamp,foo,bar\n2023-01-01T00:00:00Z,1,2\n";
        let file_path = Path::new("test_unsupported.csv");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(data.as_bytes()).unwrap();

        let data_loader = CsvDataLoader {};

        let result = data_loader.load_data(&file_path);
        assert!(matches!(result, Err(DataLoaderError::UnsupportedFormat)));

        std::fs::remove_file(file_path).unwrap();
    }
}
