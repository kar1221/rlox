use thiserror::Error;

use crate::runtime::error::RuntimeError;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Runtime(#[from] RuntimeError),
}
