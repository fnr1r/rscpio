use std::{any::Any, fmt::Debug, io::Error as IoError};

use binrw::error::CustomError as CustomErrorT;
use derive_more::AsRef;
use thiserror::Error;
use trait_set::trait_set;

trait_set! {
    pub trait BadMagicT = Any + Debug + Send + Sync;
}

#[derive(Debug, Error, AsRef)]
#[error("bad magic: {:X?}", _0)]
pub struct BadMagicError(Box<dyn BadMagicT>);

impl BadMagicError {
    pub fn new(value: impl BadMagicT + 'static) -> Self {
        Self(Box::new(value))
    }
    pub fn into_inner(self) -> Box<dyn BadMagicT> {
        self.0
    }
    pub fn as_inner(&self) -> &dyn BadMagicT {
        self.0.as_ref()
    }
}

#[derive(Debug, Error)]
#[error("{:?}", _0)]
pub struct CustomError(Box<dyn CustomErrorT>);

impl CustomError {
    pub fn new(value: impl CustomErrorT + 'static) -> Self {
        Self(Box::new(value))
    }
    pub fn into_inner(self) -> Box<dyn CustomErrorT> {
        self.0
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum MiniBinError {
    BadMagic(#[from] BadMagicError),
    Custom(#[from] CustomError),
    IoError(#[from] IoError),
}

pub type Result<T, E = MiniBinError> = std::result::Result<T, E>;
