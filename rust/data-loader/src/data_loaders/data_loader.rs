use crate::error::DataLoaderError;
use crate::types::DataRecord;

pub trait DataLoader {
    type DataIterator: Iterator<Item = Result<DataRecord, DataLoaderError>>;

    fn load_data(&self, file_path: &std::path::Path)
    -> Result<Self::DataIterator, DataLoaderError>;

    //async fn load_data_async(&self, file_path: &std::path::Path) ->
}
