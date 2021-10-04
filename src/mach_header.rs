use crate::cpu::{CpuSubType, CpuType, MH_CIGAM_64, MH_MAGIC_64};
use crate::filetype::FileType;
use crate::flag::Flag;
use std::convert::TryInto;
use std::fmt;

/// Mach Header containing information about target CPU
/// and architecture, number of load commands and flags of Mach-O
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct MachHeader {
    /// Identifies byte order and 32 vs 64 bit
    pub magic: u32,
    /// Target Processor Type
    pub cputype: CpuType,
    /// Target Processor Subtype
    pub cpu_subtype: CpuSubType,
    /// Filetype of Mach-O
    pub filetype: FileType,
    /// Number of commands
    pub ncmds: u32,
    /// Size of commands
    pub sizeofcmds: u32,
    /// Flags
    pub flags: Flag,
    /// Reserved
    pub reserved: u32,
}

impl MachHeader {
    pub fn new(raw_header: &[u8; std::mem::size_of::<MachHeader>()]) -> Self {
        Self {
            magic: u32::from_ne_bytes(raw_header[0..4].try_into().expect("Failed to get Magic")),
            cputype: CpuType(u32::from_ne_bytes(
                raw_header[4..8].try_into().expect("Failed to get CpuType"),
            )),
            cpu_subtype: CpuSubType(u32::from_ne_bytes(
                raw_header[8..12]
                    .try_into()
                    .expect("failed to get CPUSubType"),
            )),
            filetype: FileType(u32::from_ne_bytes(
                raw_header[12..16]
                    .try_into()
                    .expect("failed to get FileType"),
            )),
            ncmds: u32::from_ne_bytes(raw_header[16..20].try_into().expect("failed to get ncmds")),
            sizeofcmds: u32::from_ne_bytes(
                raw_header[20..24]
                    .try_into()
                    .expect("failed to get sizeofcmds"),
            ),
            flags: Flag(u32::from_ne_bytes(
                raw_header[24..28].try_into().expect("failed to get flags"),
            )),
            reserved: u32::from_ne_bytes(
                raw_header[28..32]
                    .try_into()
                    .expect("failed to get reserved"),
            ),
        }
    }
}

impl fmt::Display for MachHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let magic = match self.magic {
            MH_MAGIC_64 => "64-bit MachO (Little Endian)",
            MH_CIGAM_64 => "64-bit MachO (Big Endian)",
            _ => "unknown",
        };

        write!(
            f,
            "Magic:\t{}\n\
        Type:\t{}\n\
        CPU:\t{} ({})\n\
        Cmds:\t{}\n\
        Size:\t{}\n\
        Flags: {}\n",
            magic,
            self.filetype,
            self.cputype,
            self.cpu_subtype,
            self.ncmds,
            self.sizeofcmds,
            self.flags
        )
    }
}
