//! binrw, but without `Seek` requirement
//!
//! and with no derive macros

mod error;
mod macros;
mod traits;

pub use error::{BadMagicError, CustomError, MiniBinError, Result};
pub use traits::{BinReadable, BinWritable, MiniBinRead, MiniBinWrite};

pub use binrw;
