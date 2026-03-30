#![cfg_attr(feature = "lock_api", no_std)]
extern crate alloc;
#[cfg(not(feature = "lock_api"))]
mod hosted;
#[cfg(not(feature = "lock_api"))]
pub use hosted::*;
#[cfg(feature = "lock_api")]
mod no_std;
#[cfg(feature = "lock_api")]
pub use no_std::*;
mod constant_dir;
mod array_dir;
pub use constant_dir::*;
pub use array_dir::*;
use alloc::{string::String, sync::Arc};
use floreum_parser::{Entry, FloreumError, OpenOptions};
pub trait FileDescriptor {
    fn read(&self, offset: u64, count: u64) -> Result<(Arc<[u8]>, usize), FloreumError>;
    fn write(&self, offset: u64, content: &[u8]) -> Result<u64, FloreumError>;
    fn truncate(&self) -> Result<(), FloreumError>;
    fn flush(&self) -> Result<(), FloreumError>;
}
pub trait DirDescriptor {
    fn read(&self, offset: u64, count: u64) -> Result<(Arc<[(Entry<String>, Arc<dyn File>)]>, usize), FloreumError>;
    fn find(&self, name: &str, options: OpenOptions) -> Result<Arc<dyn File>, FloreumError>;
}
pub trait File {
    fn open(
        self: Arc<Self>,
        read: bool,
        write: bool,
        append: bool,
        truncate: bool,
    ) -> Result<Arc<dyn FileDescriptor>, FloreumError>;
    fn open_dir(
        self: Arc<Self>,
        read: bool,
        write: bool,
        append: bool,
        truncate: bool,
    ) -> Result<Arc<dyn DirDescriptor>, FloreumError>;
}