use tag::{TagType, VerifyTag};
use core::{mem, str, slice};
use util;


#[repr(packed)]
pub struct BootCommandLineTag {
    tag_type: u32,
    size: u32,
    string: u8, // the first byte of the string
}

impl BootCommandLineTag {
    /// Get the boot loader name.
    pub fn string(&self) -> &str {
        let length = self.size as usize - mem::size_of::<BootCommandLineTag>();
        unsafe {
            let byte_slice = slice::from_raw_parts((&self.string) as *const u8, length);

            str::from_utf8_unchecked(byte_slice)
        }

    }

    pub fn size(&self) -> usize {
        self.size as usize
    }
}

impl VerifyTag for BootCommandLineTag {
    /// Validate the input `BootLoaderNameTag`.
    fn is_valid(&self) -> bool {
        (self.tag_type == TagType::BootCommandLine as u32) && 
        util::validate_cstring(self.string())
    }
}
