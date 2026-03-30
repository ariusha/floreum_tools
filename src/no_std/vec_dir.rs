use crate::{DirDescriptor, File, FileDescriptor};
use alloc::{boxed::Box, string::String, sync::Arc};
use floreum_parser::{Entry, FloreumError};
use lock_api::{RawRwLock, RwLock};
#[derive(Clone)]
pub struct VecDir<
    Create: Fn() -> Result<Arc<dyn FileDescriptor>, FloreumError> + 'static,
    CreateDir: Fn() -> Result<Arc<dyn FileDescriptor>, FloreumError> + 'static,
    Rw: RawRwLock + 'static,
> {
    content: Arc<RwLock<Rw, Arc<[(Entry<String>, Arc<dyn File>)]>>>,
    create: Create,
    create_dir: CreateDir,
}
impl<
    Create: Fn() -> Result<Arc<dyn FileDescriptor>, FloreumError> + 'static,
    CreateDir: Fn() -> Result<Arc<dyn FileDescriptor>, FloreumError> + 'static,
    Rw: RawRwLock + 'static,
> File for VecDir<Create, CreateDir, Rw>
{
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
        _write: bool,
        _append: bool,
        _truncate: bool,
    ) -> Result<Arc<dyn DirDescriptor>, FloreumError> {
        Ok(self.clone())
    }
}
impl<
    Create: Fn() -> Result<Arc<dyn FileDescriptor>, FloreumError>,
    CreateDir: Fn() -> Result<Arc<dyn FileDescriptor>, FloreumError>,
    Rw: RawRwLock + 'static,
> DirDescriptor for VecDir<Create, CreateDir, Rw>
{
    fn read(&self, offset: u64, _count: u64) -> Result<(Arc<[(Entry<String>, Arc<dyn File>)]>, usize), FloreumError> {
        Ok((self.content.read().clone(), offset.try_into().map_err(|_| FloreumError::HostUsize)?))
    }
    fn find(&self, name: &str, options: floreum_parser::OpenOptions) -> Result<Arc<dyn File>, FloreumError> {
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
