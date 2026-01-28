use crate::error::DataLoaderError;
use crate::types::DataRecord;
use std::path::Path;

pub trait DataLoader {
    fn load_data(&self, file_path: &std::path::Path) -> Result<Vec<DataRecord>, DataLoaderError>;
}
