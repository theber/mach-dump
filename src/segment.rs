use std::fmt;

#[derive(Debug)]
pub struct Segment {
    /// Memory address of this segment
    pub vmaddr: usize,
    /// Memory size of this segment
    pub vmsize: usize,
    /// Permissions of segment
    pub perms: u8,
    /// Memory content that should be mapped
    pub content: Vec<u8>,
}

impl Segment {
    pub fn new(vmaddr: usize, vmsize: usize, perms: u8, content: Vec<u8>) -> Self {
        Self {
            vmaddr,
            vmsize,
            perms,
            content,
        }
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "vmaddr: 0x{:08x}\n\
            vmsize:  0x{:08x}\n\
            perms:   0x{:08x}\n\
            ",
            self.vmaddr,
            self.vmsize,
            self.perms,
        )
    }
}
