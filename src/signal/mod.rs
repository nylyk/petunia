mod command;
mod event;
mod status;
mod store;
pub mod subscription;
mod worker;

pub use command::Command;
pub use event::Event;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error("store error: {0}")]
    Store(#[from] presage_store_sqlite::SqliteStoreError),
    #[error("status store error: {0}")]
    StatusStore(#[from] sqlx::Error),
    #[error("signal error: {0}")]
    Signal(#[from] presage::Error<presage_store_sqlite::SqliteStoreError>),
}
