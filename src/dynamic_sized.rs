use alloc::sync::Arc;
use floreum_parser::FloreumError;
use lock_api::{RawRwLock, RwLock};
use crate::{DirDescriptor, File, FileDescriptor};
#[derive(Debug, Clone)]
pub struct DynamicSized<Rw: RawRwLock + 'static, const SIZE: usize, const MUTABLE: bool = false> {
    content: Arc<RwLock<Rw, Arc<[u8; SIZE]>>>,
}
impl<Rw: RawRwLock + 'static, const SIZE: usize, const MUTABLE: bool> File for DynamicSized<Rw, SIZE, MUTABLE> {
    fn open(&self, _read: bool, write: bool, append: bool, truncate: bool) -> Result<Arc<dyn FileDescriptor>, FloreumError> {
        if truncate | append | (write & !MUTABLE) {
            Err(FloreumError::PermissionDenied)
        } else if write {
            Ok(Arc::new(DynamicSizedMutableDescriptor::<Rw, SIZE, MUTABLE> {
                file: self.content.clone(),
                content: RwLock::new(**self.content.read()),
            }))
        } else {
            Ok(Arc::new(DynamicSizedDescriptor::<SIZE>(self.content.read().clone())))
        }
    }
    fn open_dir(&self, _read: bool, _write: bool, _append: bool, _truncate: bool) -> Result<Arc<dyn DirDescriptor>, FloreumError> {
        Err(FloreumError::NotADirectory)
    }
}
pub struct DynamicSizedDescriptor<const SIZE: usize>(Arc<[u8; SIZE]>);
impl<const SIZE: usize> FileDescriptor for DynamicSizedDescriptor<SIZE> {
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
pub struct DynamicSizedMutableDescriptor<Rw: RawRwLock + 'static, const SIZE: usize, const MUTABLE: bool = false> {
    file: Arc<RwLock<Rw, Arc<[u8; SIZE]>>>,
    content: RwLock<Rw, [u8; SIZE]>,
}
impl<Rw: RawRwLock + 'static, const SIZE: usize, const MUTABLE: bool> FileDescriptor for DynamicSizedMutableDescriptor<Rw, SIZE, MUTABLE> {
    fn read(&self, offset: u64, _count: u64) -> Result<(Arc<[u8]>, usize), FloreumError> {
        Ok((Arc::new(self.content.read().clone()), offset.try_into().map_err(|_| FloreumError::HostUsize)?))
    }
    fn write(&self, offset: u64, content: &[u8]) -> Result<u64, FloreumError> {
        let mut content_write = self.content.write();
        if let Some(as_slice) = content_write.get_mut(offset.try_into().map_err(|_| FloreumError::HostUsize)?..) {
            as_slice.iter_mut().zip(content).enumerate().map(|(index, (to, from))| {*to = *from; index}).max().unwrap_or(0).try_into().map_err(|_| FloreumError::HostUsize)
        } else {
            Err(FloreumError::FileTooBig)
        }
    }
    fn truncate(&self) -> Result<(), FloreumError> {
        Err(FloreumError::PermissionDenied)
    }
    fn flush(&self) -> Result<(), FloreumError> {
        let as_arc = Arc::new(self.content.read().clone());
        *self.file.write() = as_arc;
        Ok(())
    }
}