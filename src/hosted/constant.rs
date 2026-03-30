use crate::{DirDescriptor, File, FileDescriptor};
use alloc::sync::Arc;
use floreum_parser::FloreumError;
#[derive(Debug, Clone)]
pub struct Constant {
    content: Arc<[u8]>
}
impl FileDescriptor for Constant {
    fn read(&self, offset: u64, _count: u64) -> Result<(Arc<[u8]>, usize), FloreumError> {
        Ok((
            self.content.clone(),
            offset.try_into().map_err(|_| FloreumError::HostUsize)?,
        ))
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
    fn open(
        self: &Arc<Self>,
        _read: bool,
        write: bool,
        append: bool,
        truncate: bool,
    ) -> Result<Arc<dyn FileDescriptor>, FloreumError> {
        if write | append | truncate {
            return Err(FloreumError::PermissionDenied);
        }
        Ok(self.clone())
    }
    fn open_dir(
        self: &Arc<Self>,
        _read: bool,
        _write: bool,
        _append: bool,
        _truncate: bool,
    ) -> Result<Arc<dyn DirDescriptor>, FloreumError> {
        Err(FloreumError::NotADirectory)
    }
}
