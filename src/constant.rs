use alloc::sync::Arc;
use floreum_parser::FloreumError;
use crate::{DirDescriptor, File, FileDescriptor};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Constant(Arc<[u8]>);
impl FileDescriptor for Constant {
    fn read(&self, offset: u64, _count: u64) -> Result<(Arc<[u8]>, usize), FloreumError> {
        Ok((self.0.clone(), offset.try_into().map_err(|_| FloreumError::HostUsize)?))
    }
    fn write(&self, _offset: u64, _content: &[u8]) -> Result<u64, FloreumError> {
        Err(FloreumError::PermissionDenied)
    }
    fn truncate(&self) -> Result<(), FloreumError> {
        Err(FloreumError::PermissionDenied)
    }
    fn flush(&self) -> Result<(), FloreumError> {
        Err(FloreumError::PermissionDenied)
    }
}
impl File for Constant {
    fn open(&self, _read: bool, write: bool, append: bool, truncate: bool) -> Result<Arc<dyn FileDescriptor>, FloreumError> {
        if truncate | append | write {
            return Err(FloreumError::PermissionDenied)
        }
        Ok(Arc::new(self.clone()))
    }
    fn open_dir(&self, _read: bool, _write: bool, _append: bool, _truncate: bool) -> Result<Arc<dyn DirDescriptor>, FloreumError> {
        Err(FloreumError::NotADirectory)
    }
}