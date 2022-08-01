use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyDbError {
    #[error("No such row with id \"{id:?}\"")]
    NoSuchRow { id: String }, // Need to add table name to this
    #[error("Row with id \"{id:?}\" in table \"{table_name:?}\" did not update properly")]
    BadUpdate { id: String, table_name: String },
    #[error("Row in table \"{table_name:?}\" was not inserted row properly")]
    BadInsert { table_name: String },
}

// #[derive(Error, Debug)]
// pub enum MyAuthError {
//     #[error("Request contains no JWT")]
//     NoJwt,
//     #[error("Jwt is invalid")]
//     InvalidJwt,
// }
