use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyDbError {
    #[error("No such row with id \"{id:?}\"")]
    NoSuchRow {
        id: String,
    }
}
