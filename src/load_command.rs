use std::convert::TryInto;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LoadCommandType(pub u32);

const LC_REQ_DYLD: u32 = 0x80000000;

pub const LC_SEGMENT: LoadCommandType = LoadCommandType(0x1);
pub const LC_SYMTAB: LoadCommandType = LoadCommandType(0x2);
pub const LC_SYMSEG: LoadCommandType = LoadCommandType(0x3);
pub const LC_THREAD: LoadCommandType = LoadCommandType(0x4);
pub const LC_UNIXTHREAD: LoadCommandType = LoadCommandType(0x5);
pub const LC_LOADFVMLIB: LoadCommandType = LoadCommandType(0x6);
pub const LC_IDFVMLIB: LoadCommandType = LoadCommandType(0x7);
pub const LC_IDENT: LoadCommandType = LoadCommandType(0x8);
pub const LC_FVMFILE: LoadCommandType = LoadCommandType(0x9);
pub const LC_PREPAGE: LoadCommandType = LoadCommandType(0xa);
pub const LC_DYSYMTAB: LoadCommandType = LoadCommandType(0xb);
pub const LC_LOAD_DYLIB: LoadCommandType = LoadCommandType(0xc);
pub const LC_ID_DYLIB: LoadCommandType = LoadCommandType(0xd);
pub const LC_LOAD_DYLINKER: LoadCommandType = LoadCommandType(0xe);
pub const LC_ID_DYLINKER: LoadCommandType = LoadCommandType(0xf);
pub const LC_PREBOUND_DYLIB: LoadCommandType = LoadCommandType(0x10);
pub const LC_ROUTINES: LoadCommandType = LoadCommandType(0x11);
pub const LC_SUB_FRAMEWORK: LoadCommandType = LoadCommandType(0x12);
pub const LC_SUB_UMBRELLA: LoadCommandType = LoadCommandType(0x13);
pub const LC_SUB_CLIENT: LoadCommandType = LoadCommandType(0x14);
pub const LC_SUB_LIBRARY: LoadCommandType = LoadCommandType(0x15);
pub const LC_TWOLEVEL_HINTS: LoadCommandType = LoadCommandType(0x16);
pub const LC_PREBIND_CKSUM: LoadCommandType = LoadCommandType(0x17);
pub const LC_LOAD_WEAK_DYLIB: LoadCommandType = LoadCommandType(0x18 | LC_REQ_DYLD);
pub const LC_SEGMENT_64: LoadCommandType = LoadCommandType(0x19);
pub const LC_ROUTINES_64: LoadCommandType = LoadCommandType(0x1a);
pub const LC_UUID: LoadCommandType = LoadCommandType(0x1b);
pub const LC_RPATH: LoadCommandType = LoadCommandType(0x1c | LC_REQ_DYLD);
pub const LC_CODE_SIGNATURE: LoadCommandType = LoadCommandType(0x1d);
pub const LC_SEGMENT_SPLIT_INFO: LoadCommandType = LoadCommandType(0x1e);
pub const LC_REEXPORT_DYLIB: LoadCommandType = LoadCommandType(0x1f | LC_REQ_DYLD);
pub const LC_LAZY_LOAD_DYLIB: LoadCommandType = LoadCommandType(0x20);
pub const LC_ENCRYPTION_INFO: LoadCommandType = LoadCommandType(0x21);
pub const LC_DYLD_INFO: LoadCommandType = LoadCommandType(0x22);
pub const LC_DYLD_INFO_ONLY: LoadCommandType = LoadCommandType(0x22 | LC_REQ_DYLD);
pub const LC_LOAD_UPWARD_DYLIB: LoadCommandType = LoadCommandType(0x23 | LC_REQ_DYLD);
pub const LC_VERSION_MIN_MACOSX: LoadCommandType = LoadCommandType(0x24);
pub const LC_VERSION_MIN_IPHONEOS: LoadCommandType = LoadCommandType(0x25);
pub const LC_FUNCTION_STARTS: LoadCommandType = LoadCommandType(0x26);
pub const LC_DYLD_ENVIRONMENT: LoadCommandType = LoadCommandType(0x27);
pub const LC_MAIN: LoadCommandType = LoadCommandType(0x28 | LC_REQ_DYLD);
pub const LC_DATA_IN_CODE: LoadCommandType = LoadCommandType(0x29);
pub const LC_SOURCE_VERSION: LoadCommandType = LoadCommandType(0x2A);
pub const LC_DYLIB_CODE_SIGN_DRS: LoadCommandType = LoadCommandType(0x2B);
pub const LC_ENCRYPTION_INFO_64: LoadCommandType = LoadCommandType(0x2C);
pub const LC_LINKER_OPTION: LoadCommandType = LoadCommandType(0x2D);
pub const LC_LINKER_OPTIMIZATION_HINT: LoadCommandType = LoadCommandType(0x2E);
pub const LC_VERSION_MIN_TVOS: LoadCommandType = LoadCommandType(0x2F);
pub const LC_VERSION_MIN_WATCHOS: LoadCommandType = LoadCommandType(0x30);
pub const LC_NOTE: LoadCommandType = LoadCommandType(0x31);
pub const LC_BUILD_VERSION: LoadCommandType = LoadCommandType(0x32);
pub const LC_DYLD_EXPORTS_TRIE: LoadCommandType = LoadCommandType(0x33 | LC_REQ_DYLD);
pub const LC_DYLD_CHAINED_FIXUPS: LoadCommandType = LoadCommandType(0x34 | LC_REQ_DYLD);
pub const LC_FILESET_ENTRY: LoadCommandType = LoadCommandType(0x35 | LC_REQ_DYLD);

impl fmt::Display for LoadCommandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let filetype = match *self {
            LC_SEGMENT => "LC_SEGMENT",
            LC_SYMTAB => "LC_SYMTAB",
            LC_SYMSEG => "LC_SYMSEG",
            LC_THREAD => "LC_THREAD",
            LC_UNIXTHREAD => "LC_UNIXTHREAD",
            LC_LOADFVMLIB => "LC_LOADFVMLIB",
            LC_IDFVMLIB => "LC_IDFVMLIB",
            LC_IDENT => "LC_IDENT",
            LC_FVMFILE => "LC_FVMFILE",
            LC_PREPAGE => "LC_PREPAGE",
            LC_DYSYMTAB => "LC_DYSYMTAB",
            LC_LOAD_DYLIB => "LC_LOAD_DYLIB",
            LC_ID_DYLIB => "LC_ID_DYLIB",
            LC_LOAD_DYLINKER => "LC_LOAD_DYLINKER",
            LC_ID_DYLINKER => "LC_ID_DYLINKER",
            LC_PREBOUND_DYLIB => "LC_PREBOUND_DYLIB",
            LC_ROUTINES => "LC_ROUTINES",
            LC_SUB_FRAMEWORK => "LC_SUB_FRAMEWORK",
            LC_SUB_UMBRELLA => "LC_SUB_UMBRELLA",
            LC_SUB_CLIENT => "LC_SUB_CLIENT",
            LC_SUB_LIBRARY => "LC_SUB_LIBRARY",
            LC_TWOLEVEL_HINTS => "LC_TWOLEVEL_HINTS",
            LC_PREBIND_CKSUM => "LC_PREBIND_CKSUM",
            LC_LOAD_WEAK_DYLIB => "LC_LOAD_WEAK_DYLIB",
            LC_SEGMENT_64 => "LC_SEGMENT_64",
            LC_ROUTINES_64 => "LC_ROUTINES_64",
            LC_UUID => "LC_UUID",
            LC_RPATH => "LC_RPATH",
            LC_CODE_SIGNATURE => "LC_CODE_SIGNATURE",
            LC_SEGMENT_SPLIT_INFO => "LC_SEGMENT_SPLIT_INFO",
            LC_REEXPORT_DYLIB => "LC_REEXPORT_DYLIB",
            LC_LAZY_LOAD_DYLIB => "LC_LAZY_LOAD_DYLIB",
            LC_ENCRYPTION_INFO => "LC_ENCRYPTION_INFO",
            LC_DYLD_INFO => "LC_DYLD_INFO",
            LC_DYLD_INFO_ONLY => "LC_DYLD_INFO_ONLY",
            LC_LOAD_UPWARD_DYLIB => "LC_LOAD_UPWARD_DYLIB",
            LC_VERSION_MIN_MACOSX => "LC_VERSION_MIN_MACOSX",
            LC_VERSION_MIN_IPHONEOS => "LC_VERSION_MIN_IPHONEOS",
            LC_FUNCTION_STARTS => "LC_FUNCTION_STARTS",
            LC_DYLD_ENVIRONMENT => "LC_DYLD_ENVIRONMENT",
            LC_MAIN => "LC_MAIN",
            LC_DATA_IN_CODE => "LC_DATA_IN_CODE",
            LC_SOURCE_VERSION => "LC_SOURCE_VERSION",
            LC_DYLIB_CODE_SIGN_DRS => "LC_DYLIB_CODE_SIGN_DRS",
            LC_ENCRYPTION_INFO_64 => "LC_ENCRYPTION_INFO_64",
            LC_LINKER_OPTION => "LC_LINKER_OPTION",
            LC_LINKER_OPTIMIZATION_HINT => "LC_LINKER_OPTIMIZATION_HINT",
            LC_VERSION_MIN_TVOS => "LC_VERSION_MIN_TVOS",
            LC_VERSION_MIN_WATCHOS => "LC_VERSION_MIN_WATCHOS",
            LC_NOTE => "LC_NOTE",
            LC_BUILD_VERSION => "LC_BUILD_VERSION",
            LC_DYLD_EXPORTS_TRIE => "LC_DYLD_EXPORTS_TRIE",
            LC_DYLD_CHAINED_FIXUPS => "LC_DYLD_CHAINED_FIXUPS",
            LC_FILESET_ENTRY => "LC_FILESET_ENTRY",
            _ => "unknown",
        };

        write!(f, "{}", filetype)
    }
}

/// LoadCommand are stored right after the Mach-O Header in a 
/// core dump. It only contains the LoadCommandType and its size
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct LoadCommand {
    pub cmd: LoadCommandType,
    pub cmdsize: u32,
}

impl fmt::Display for LoadCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:30} | Size: {}", self.cmd, self.cmdsize)
    }
}

impl LoadCommand {
    pub fn new(raw_lc: &[u8; std::mem::size_of::<LoadCommand>()]) -> Self {
        Self {
            cmd: LoadCommandType(u32::from_le_bytes(raw_lc[..4].try_into().expect("A"))),
            cmdsize: u32::from_le_bytes(raw_lc[4..].try_into().expect("B")),
        }
    }
}

/// Core dumps use the SegmentCommand64 command to store 
/// memory content. The segment name is always empty. 
/// Maximum and initial permissions are always the same.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct SegmentCommand64 {
    /// Is always SegmentCommand64
    cmd: LoadCommandType,
    /// Total size of command in bytes
    cmdsize: u32,
    /// Never set in core dumps
    segname: [u8; 16],
    /// Virtual address where content should be mapped
    pub vmaddr: u64,
    /// Amount of memory that should be allocated
    pub vmsize: u64,
    /// Offset to where memory contents are stored in the core dump
    pub fileoff: u64,
    /// Amount of bytes that should be read from core dump
    pub filesize: u64,
    /// Permissions of segment (R/W/E)
    pub maxprot: i32,
    /// Alsways same as maxprot
    initprot: i32,
    /// Always set to 0 
    nsects: u32,
    /// Flags
    flags: u32,
}

impl SegmentCommand64 {
    pub fn new(raw_sc64: &[u8; std::mem::size_of::<SegmentCommand64>()]) -> Self {
        Self {
            cmd: LC_SEGMENT_64,
            cmdsize: std::mem::size_of::<SegmentCommand64>() as u32,
            segname: raw_sc64[8..24].try_into().unwrap(),
            vmaddr: u64::from_le_bytes(raw_sc64[24..32].try_into().unwrap()),
            vmsize: u64::from_le_bytes(raw_sc64[32..40].try_into().unwrap()),
            fileoff: u64::from_le_bytes(raw_sc64[40..48].try_into().unwrap()),
            filesize: u64::from_le_bytes(raw_sc64[48..56].try_into().unwrap()),
            maxprot: i32::from_le_bytes(raw_sc64[56..60].try_into().unwrap()),
            initprot: i32::from_le_bytes(raw_sc64[60..64].try_into().unwrap()),
            nsects: u32::from_le_bytes(raw_sc64[64..68].try_into().unwrap()),
            flags: u32::from_le_bytes(raw_sc64[68..72].try_into().unwrap()),
        }
    }
}

impl fmt::Display for SegmentCommand64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name:\t{}\n\
        vmaddr:   0x{:08x}\n\
        vmsize:   0x{:08x}\n\
        fileoff:  0x{:08x}\n\
        filesize: 0x{:08x}\n\
        maxprot:  0x{:08x}\n\
        initprot: 0x{:08x}\n\
        nsects:   0x{:08x}\n\
        flags:    0x{:08x}\n\
        ",
            std::str::from_utf8(&self.segname).unwrap_or("Invalid Name"),
            self.vmaddr,
            self.vmsize,
            self.fileoff,
            self.filesize,
            self.maxprot,
            self.initprot,
            self.nsects,
            self.flags,
        )
    }
}

/// ArmThreadState64 contains all general purpose registers,
/// the frame pointer, link register, stack pointer,
/// program counter, and the current program status register
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ArmThreadState64 {
    /// General Purpose Registers
    pub x: [u64; 29],
    /// Frame Pointer
    pub fp: u64,
    /// Link Register
    pub lr: u64,
    /// Stack Pointer
    pub sp: u64,
    /// Program Counter
    pub pc: u64,
    /// Current Program Status Register
    pub cpsr: u32,
    /// Padding
    pub pad: u32,
}

impl fmt::Display for ArmThreadState64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "X0:  0x{:016x}   X1:  0x{:016x}   X2:  0x{:016x}   X3:  0x{:016x}\n\
            X4:  0x{:016x}   X5:  0x{:016x}   X6:  0x{:016x}   X7:  0x{:016x}\n\
            X8:  0x{:016x}   X9:  0x{:016x}   X10: 0x{:016x}   X11: 0x{:016x}\n\
            X12: 0x{:016x}   X13: 0x{:016x}   X14: 0x{:016x}   X15: 0x{:016x}\n\
            X16: 0x{:016x}   X17: 0x{:016x}   X18: 0x{:016x}   X19: 0x{:016x}\n\
            X20: 0x{:016x}   X21: 0x{:016x}   X22: 0x{:016x}   X23: 0x{:016x}\n\
            X24: 0x{:016x}   X25: 0x{:016x}   X26: 0x{:016x}   X27: 0x{:016x}\n\
            X28: 0x{:016x}   FP:  0x{:016x}   LR:  0x{:016x}   SP:  0x{:016x}\n\
            PC:  0x{:016x}   CPSR: 0x{:016x}\n",
            self.x[0], 
            self.x[1], 
            self.x[2], 
            self.x[3], 
            self.x[4], 
            self.x[5], 
            self.x[6], 
            self.x[7], 
            self.x[8], 
            self.x[9], 
            self.x[10], 
            self.x[11], 
            self.x[12], 
            self.x[13], 
            self.x[14], 
            self.x[15], 
            self.x[16], 
            self.x[17], 
            self.x[18], 
            self.x[19], 
            self.x[20], 
            self.x[21], 
            self.x[22], 
            self.x[23], 
            self.x[24], 
            self.x[25], 
            self.x[26], 
            self.x[27], 
            self.x[28], 
            self.fp, 
            self.lr, 
            self.sp, 
            self.pc, 
            self.cpsr, 
        )
    }
}

/// ThreadCommand contains general purpose register state 
/// for each thread
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ThreadCommand {
    /// Always ThreadCommand
    cmd: LoadCommandType,
    /// Size of this command
    cmdsize: u32,
    /// Flavor
    flavor: u32,
    /// Amount of u32s in ArmThreadState64
    count: u32,
    /// Contains General Purpose Register Information
    pub state: ArmThreadState64,
}

impl ThreadCommand {
    pub fn new(raw_tc: &[u8; 16 + std::mem::size_of::<ArmThreadState64>()]) -> Self {
        let mut regs: [u64; 29] = [0; 29];
        for i in 0..regs.len() {
            regs[i] = u64::from_le_bytes(raw_tc[16 + i * 8..16 + (i + 1) * 8].try_into().unwrap());
        }

        let reg_offset = 16 + 29 * 8;

        let arm_thread_state64 = ArmThreadState64 {
            x: regs,
            fp: u64::from_le_bytes(raw_tc[reg_offset..reg_offset + 8].try_into().unwrap()),
            lr: u64::from_le_bytes(raw_tc[reg_offset + 8..reg_offset + 16].try_into().unwrap()),
            sp: u64::from_le_bytes(raw_tc[reg_offset + 16..reg_offset + 24].try_into().unwrap()),
            pc: u64::from_le_bytes(raw_tc[reg_offset + 24..reg_offset + 32].try_into().unwrap()),
            cpsr: u32::from_le_bytes(raw_tc[reg_offset + 32..reg_offset + 36].try_into().unwrap()),
            pad: u32::from_le_bytes(raw_tc[reg_offset + 36..reg_offset + 40].try_into().unwrap()),
        };

        Self {
            cmd: LC_THREAD,
            cmdsize: std::mem::size_of::<ThreadCommand>() as u32,
            flavor: u32::from_le_bytes(raw_tc[8..12].try_into().unwrap()),
            count: u32::from_le_bytes(raw_tc[12..16].try_into().unwrap()),
            state: arm_thread_state64,
        }
    }
}

/// Enum for storing boxed Commands
#[derive(Debug)]
pub enum CommandType {
    SegmentCommand64(Box<SegmentCommand64>),
    ThreadCommand(Box<ThreadCommand>),
}
