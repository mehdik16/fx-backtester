use crate::data_loaders::data_loader::DataLoader;
use crate::error::DataLoaderError;
use crate::types::DataRecord;
use crate::types::{Ohlcv, Tick};
use std::path::Path;

pub struct CsvDataLoader {}

pub struct CsvDataIterator {
    reader: csv::Reader<std::fs::File>,
    data_format: DataFormat,
}

#[derive(Debug)]
enum DataFormat {
    Ohlcv,
    Tick,
}

impl Iterator for CsvDataIterator {
    type Item = Result<DataRecord, DataLoaderError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.data_format {
            DataFormat::Ohlcv => match self.reader.deserialize::<Ohlcv>().next() {
                Some(Ok(record)) => Some(Ok(DataRecord::Ohlcv(record))),
                Some(Err(e)) => Some(Err(DataLoaderError::Csv(e))),
                None => None,
            },
            DataFormat::Tick => match self.reader.deserialize::<Tick>().next() {
                Some(Ok(record)) => Some(Ok(DataRecord::Tick(record))),
                Some(Err(e)) => Some(Err(DataLoaderError::Csv(e))),
                None => None,
            },
        }
    }
}

impl DataLoader for CsvDataLoader {
    type DataIterator = CsvDataIterator;

    fn load_data(&self, file_path: &Path) -> Result<Self::DataIterator, DataLoaderError> {
        let mut reader = csv::Reader::from_path(file_path)?;
        let headers = reader.headers()?.clone();

        let data_format = if headers.iter().any(|h| h == "open") {
            DataFormat::Ohlcv
        } else if headers.iter().any(|h| h == "bid") {
            DataFormat::Tick
        } else {
            return Err(DataLoaderError::UnsupportedFormat);
        };

        Ok(CsvDataIterator {
            reader,
            data_format,
        })
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
        let mut file = File::create(file_path).unwrap();
        file.write_all(data.as_bytes()).unwrap();

        let data_loader = CsvDataLoader {};
        let mut iterator = data_loader.load_data(file_path).unwrap();

        let first_record = iterator.next().unwrap().unwrap();
        match first_record {
            DataRecord::Ohlcv(ohlcv) => {
                assert_eq!(ohlcv.open, 1.1);
            }
            _ => panic!("Expected OHLCV data"),
        }

        assert!(iterator.next().is_none()); // Should be no more records

        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_load_tick_data() {
        let data = "timestamp,bid,ask\n2023-01-01T00:00:00Z,1.1,1.2\n";
        let file_path = Path::new("test_tick.csv");
        let mut file = File::create(file_path).unwrap();
        file.write_all(data.as_bytes()).unwrap();

        let data_loader = CsvDataLoader {};
        let mut iterator = data_loader.load_data(file_path).unwrap();

        let first_record = iterator.next().unwrap().unwrap();
        match first_record {
            DataRecord::Tick(tick) => {
                assert_eq!(tick.bid, 1.1);
            }
            _ => panic!("Expected Tick data"),
        }

        assert!(iterator.next().is_none()); // Should be no more records

        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_load_unsupported_format() {
        let data = "timestamp,foo,bar\n2023-01-01T00:00:00Z,1,2\n";
        let file_path = Path::new("test_unsupported.csv");
        let mut file = File::create(file_path).unwrap();
        file.write_all(data.as_bytes()).unwrap();

        let data_loader = CsvDataLoader {};
        let result = data_loader.load_data(file_path);
        assert!(matches!(result, Err(DataLoaderError::UnsupportedFormat)));

        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_lazy_iteration() {
        let data = "timestamp,open,high,low,close,volume\n2023-01-01T00:00:00Z,1.1,1.2,1.0,1.1,100.0\n2023-01-01T01:00:00Z,1.2,1.3,1.1,1.25,200.0\n";
        let file_path = Path::new("test_lazy.csv");
        let mut file = File::create(file_path).unwrap();
        file.write_all(data.as_bytes()).unwrap();

        let data_loader = CsvDataLoader {};
        let iterator = data_loader.load_data(file_path).unwrap();

        // Collect all records to verify lazy loading works
        let records: Result<Vec<_>, _> = iterator.collect();
        let records = records.unwrap();
        assert_eq!(records.len(), 2);

        std::fs::remove_file(file_path).unwrap();
    }
}
