use crate::{DirDescriptor, File, FileDescriptor};
use alloc::{boxed::Box, sync::Arc};
use floreum_parser::FloreumError;
use std::sync::RwLock;
#[derive(Debug, Clone)]
pub struct Array<
    const SIZE: usize,
    Flush: Fn(Arc<[u8; SIZE]>) -> Result<(), FloreumError> + 'static + Clone,
> {
    content: Arc<RwLock<Arc<[u8; SIZE]>>>,
    flush: Flush,
}
impl<const SIZE: usize, Flush: Fn(Arc<[u8; SIZE]>) -> Result<(), FloreumError> + 'static + Clone>
    File for Array<SIZE, Flush>
{
    fn open(
        self: &Arc<Self>,
        _read: bool,
        _write: bool,
        append: bool,
        truncate: bool,
    ) -> Result<Arc<dyn FileDescriptor>, FloreumError> {
        if truncate | append {
            Err(FloreumError::PermissionDenied)
        } else {
            Ok(self.clone())
        }
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
impl<const SIZE: usize, Flush: Fn(Arc<[u8; SIZE]>) -> Result<(), FloreumError> + 'static + Clone>
    FileDescriptor for Array<SIZE, Flush>
{
    fn read(&self, offset: u64, _count: u64) -> Result<(Arc<[u8]>, usize), FloreumError> {
        Ok((
            self.content.read().map_err(|_| FloreumError::SyncPoison)?.clone(),
            offset.try_into().map_err(|_| FloreumError::HostUsize)?,
        ))
    }
    fn write(&self, offset: u64, content: &[u8]) -> Result<u64, FloreumError> {
        let mut content_write = self.content.write().map_err(|_| FloreumError::SyncPoison)?;
        let mut content_mut = Box::new(**content_write);
        if let Some(as_slice) =
            (*content_mut).get_mut(offset.try_into().map_err(|_| FloreumError::HostUsize)?..)
        {
            let length = as_slice
                .iter_mut()
                .zip(content)
                .enumerate()
                .map(|(index, (to, from))| {
                    *to = *from;
                    index
                })
                .max()
                .unwrap_or(0)
                .try_into()
                .map_err(|_| FloreumError::HostUsize);
            *content_write = content_mut.into();
            length
        } else {
            Err(FloreumError::FileTooBig)
        }
    }
    fn truncate(&self) -> Result<(), FloreumError> {
        Err(FloreumError::PermissionDenied)
    }
    fn flush(&self) -> Result<(), FloreumError> {
        (self.flush)(self.content.read().map_err(|_| FloreumError::SyncPoison)?.clone())
    }
}
