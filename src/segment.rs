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
