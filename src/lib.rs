#![no_std]
extern crate alloc;
mod array;
mod constant;
pub use array::*;
pub use constant::*;
use alloc::{string::String, sync::Arc};
use floreum_parser::{Entry, FloreumError,};
pub trait FileDescriptor {
    fn read(&self, offset: u64, count: u64) -> Result<(Arc<[u8]>, usize), FloreumError>;
    fn write(&self, offset: u64, content: &[u8]) -> Result<u64, FloreumError>;
    fn truncate(&self) -> Result<(), FloreumError>;
    fn flush(&self) -> Result<(), FloreumError>;
}
pub trait DirDescriptor {
    fn read(&self, offset: u64, count: u64) -> Result<(Arc<[Entry<String>]>, usize), FloreumError>;
    fn create_file(&self, read: bool, write: bool, append: bool, new: bool, name: String) -> Result<Arc<dyn FileDescriptor>, FloreumError>;
    fn create_dir(&self, read: bool, write: bool, append: bool, new: bool, name: String) -> Result<Arc<dyn DirDescriptor>, FloreumError>;
}
pub trait File {
    fn open(&self, read: bool, write: bool, append: bool, truncate: bool) -> Result<Arc<dyn FileDescriptor>, FloreumError>;
    fn open_dir(&self, read: bool, write: bool, append: bool, truncate: bool) -> Result<Arc<dyn DirDescriptor>, FloreumError>;
}