use crate::{DirDescriptor, File, FileDescriptor};
use alloc::{sync::Arc, vec::Vec as AllocVec};
use floreum_parser::FloreumError;
use std::sync::RwLock;
#[derive(Debug, Clone)]
pub struct Vec<Flush: Fn(&[u8]) -> Result<(), FloreumError> + 'static + Clone> {
    content: Arc<RwLock<AllocVec<u8>>>,
    flush: Flush,
}
impl<Flush: Fn(&[u8]) -> Result<(), FloreumError> + 'static + Clone> File for Vec<Flush> {
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
impl<Flush: Fn(&[u8]) -> Result<(), FloreumError> + 'static + Clone> FileDescriptor for Vec<Flush> {
    fn read(&self, offset: u64, count: u64) -> Result<(Arc<[u8]>, usize), FloreumError> {
        let offset_usize: usize = offset.try_into().map_err(|_| FloreumError::HostUsize)?;
        let count_usize: usize = count.try_into().map_err(|_| FloreumError::HostUsize)?;
        Ok((
            Arc::from(
                self.content
                    .read()
                    .map_err(|_| FloreumError::SyncPoison)?
                    .get(offset_usize..offset_usize + count_usize)
                    .unwrap_or(&[]),
            ),
            offset.try_into().map_err(|_| FloreumError::HostUsize)?,
        ))
    }
    fn write(&self, offset: u64, content: &[u8]) -> Result<u64, FloreumError> {
        let offset_usize: usize = offset.try_into().map_err(|_| FloreumError::HostUsize)?;
        if offset_usize + content.len() > isize::MAX.try_into().unwrap() {
            // todo: test if this needs to be `>=`
            Err(FloreumError::FileTooBig)
        } else {
            let mut content_write = self.content.write().map_err(|_| FloreumError::SyncPoison)?;
            content_write.resize(offset_usize + content.len(), 0);
            (*content_write)[offset_usize..]
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
                .map_err(|_| FloreumError::HostUsize)
        }
    }
    fn truncate(&self) -> Result<(), FloreumError> {
        Err(FloreumError::PermissionDenied)
    }
    fn flush(&self) -> Result<(), FloreumError> {
        (self.flush)(&self.content.read().map_err(|_| FloreumError::SyncPoison)?)
    }
}
