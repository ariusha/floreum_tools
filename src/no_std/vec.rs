use crate::{DirDescriptor, File, FileDescriptor};
use alloc::{boxed::Box, sync::Arc};
use floreum_parser::FloreumError;
use lock_api::{RawRwLock, RwLock};
#[derive(Debug, Clone)]
pub struct Vec<
    Flush: Fn(&[u8]) -> Result<(), FloreumError> + 'static + Clone,
    Rw: RawRwLock + 'static,
> {
    content: Arc<RwLock<Rw, Box<[u8]>>>,
    flush: Flush,
}
impl<Flush: Fn(&[u8]) -> Result<(), FloreumError> + 'static + Clone, Rw: RawRwLock + 'static> File
    for Vec<Flush, Rw>
{
    fn open(
        self: Arc<Self>,
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
        self: Arc<Self>,
        _read: bool,
        _write: bool,
        _append: bool,
        _truncate: bool,
    ) -> Result<Arc<dyn DirDescriptor>, FloreumError> {
        Err(FloreumError::NotADirectory)
    }
}
impl<Flush: Fn(&[u8]) -> Result<(), FloreumError> + 'static + Clone, Rw: RawRwLock + 'static>
    FileDescriptor for Vec<Flush, Rw>
{
    fn read(&self, offset: u64, count: u64) -> Result<(Arc<[u8]>, usize), FloreumError> {
        let offset_usize: usize = offset.try_into().map_err(|_| FloreumError::HostUsize)?;
        let count_usize: usize = count.try_into().map_err(|_| FloreumError::HostUsize)?;
        Ok((
            Arc::from(&self.content.read().as_ref()[offset_usize..offset_usize + count_usize]),
            offset.try_into().map_err(|_| FloreumError::HostUsize)?,
        ))
    }
    fn write(&self, offset: u64, content: &[u8]) -> Result<u64, FloreumError> {
        let mut content_write = self.content.write();
        if let Some(as_slice) =
            (*content_write).get_mut(offset.try_into().map_err(|_| FloreumError::HostUsize)?..)
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
            length
        } else {
            Err(FloreumError::FileTooBig)
        }
    }
    fn truncate(&self) -> Result<(), FloreumError> {
        Err(FloreumError::PermissionDenied)
    }
    fn flush(&self) -> Result<(), FloreumError> {
        (self.flush)(self.content.read().as_ref())
    }
}
