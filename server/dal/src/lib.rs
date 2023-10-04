use thiserror::Error;

pub mod database;
pub mod s3;
pub mod storage_engine;

#[derive(Debug, Error)]
pub enum DalError {
    #[error("{0}")]
    Db(#[from] database::DatabaseError),
    #[error("{0}")]
    Storage(#[from] storage_engine::StorageEngineError),
}