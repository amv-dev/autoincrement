#![doc = include_str!("../README.md")]
#[cfg(feature = "sync")]
mod sync;
#[cfg(feature = "sync")]
pub use sync::*;

#[cfg(feature = "async")]
mod atomic;
#[cfg(feature = "async")]
pub use atomic::*;

#[cfg(feature = "derive")]
pub use autoincrement_derive::{AsyncIncremental, Incremental};

pub mod prelude {
    #[cfg(feature = "sync")]
    pub use crate::Incremental;

    #[cfg(feature = "async")]
    pub use crate::AsyncIncremental;
}
