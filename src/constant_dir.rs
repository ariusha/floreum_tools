use alloc::{string::String, sync::Arc};
use floreum_parser::{Entry, FloreumError, OpenOptions};
use crate::{DirDescriptor, File, FileDescriptor};
#[derive(Clone)]
pub struct ConstantDir<const SIZE: usize> {
    content: Arc<[(Entry<String>, Arc<dyn File>); SIZE]>,
}
impl<const SIZE: usize> File for ConstantDir<SIZE> {
    fn open(
        self: Arc<Self>,
        _read: bool,
        _write: bool,
        _append: bool,
        _truncate: bool,
    ) -> Result<Arc<dyn FileDescriptor>, FloreumError> {
        Err(FloreumError::NotAFile)
    }
    fn open_dir(
        self: Arc<Self>,
        _read: bool,
        write: bool,
        append: bool,
        truncate: bool,
    ) -> Result<Arc<dyn DirDescriptor>, FloreumError> {
        if write | append | truncate {
            Err(FloreumError::PermissionDenied)
        } else {
            Ok(self.clone())
        }
    }
}
impl<const SIZE: usize> DirDescriptor for ConstantDir<SIZE> {
    fn read(&self, offset: u64, _count: u64) -> Result<(Arc<[(Entry<String>, Arc<dyn File>)]>, usize), FloreumError> {
        Ok((
            self.content.clone(),
            offset.try_into().map_err(|_| FloreumError::HostUsize)?,
        ))
    }
    fn find(&self, name: &str, options: OpenOptions) -> Result<Arc<dyn File>, FloreumError> {
        if let Some((_, file)) = self.content.iter().find(|(entry, _)| entry.name == name) {
            if options.create & options.create_new {
                Err(FloreumError::DoesExist)
            } else {
                Ok(file.clone())
            }
        } else if options.create {
            Err(FloreumError::PermissionDenied)
        } else {
            Err(FloreumError::DoesNotExist)
        }
    }
}