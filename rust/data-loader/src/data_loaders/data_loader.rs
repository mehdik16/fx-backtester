use crate::error::DataLoaderError;
use crate::types::DataRecord;

pub trait DataLoader {
    fn load_data(&self, file_path: &std::path::Path) -> Result<Vec<DataRecord>, DataLoaderError>;
}
