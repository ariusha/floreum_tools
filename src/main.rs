use serde::{Deserialize, Serialize};
fn main() {
    println!("Hello, world!");
}
#[repr(C, align(4096))]
pub struct Page([u8; 4096]);
pub struct Pages(Vec<Page>);
pub trait Entry {
    fn read(&self, length: u64) -> Vec<Page>;
    fn write(&self, content: Vec<Page>) -> u64;
}
pub struct Dynamic<Content: Serialize + for<'de> Deserialize<'de>, Read: Fn(u64) -> Content, Write: Fn(Content) -> u64> {
    read: Read,
    write: Write,
}
impl<Content: Serialize + for<'de> Deserialize<'de>, Read: Fn() -> Content, Write: Fn(Content)> Entry for Dynamic<Content, Read, Write> {
    fn read(&self, length: u64) -> Vec<Page> {
        (self.read)(length)
    }

    fn write(&self, content: Vec<Page>) -> u64 {
        (self.write(content)
    }
}