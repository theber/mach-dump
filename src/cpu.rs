use std::fmt;

/// Processor Type
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CpuType(pub u32);

pub const MH_MAGIC_64: u32 = 0xfeedfacf;
pub const MH_CIGAM_64: u32 = 0xcffaedfe;

const CPU_ARCH_ABI64: u32 = 0x01000000; // 64 bit ABI

const CPU_TYPE_ANY: CpuType = CpuType(u32::MAX);
const CPU_TYPE_VAX: CpuType = CpuType(1);
const CPU_TYPE_MC680X0: CpuType = CpuType(6);
const CPU_TYPE_X86: CpuType = CpuType(7);
const CPU_TYPE_X86_64: CpuType = CpuType(CPU_TYPE_X86.0 | CPU_ARCH_ABI64);
const CPU_TYPE_MC98000: CpuType = CpuType(10);
const CPU_TYPE_HPPA: CpuType = CpuType(11);
const CPU_TYPE_ARM: CpuType = CpuType(12);
const CPU_TYPE_ARM64: CpuType = CpuType(CPU_TYPE_ARM.0 | CPU_ARCH_ABI64);
const CPU_TYPE_MC88000: CpuType = CpuType(13);
const CPU_TYPE_SPARC: CpuType = CpuType(14);
const CPU_TYPE_I860: CpuType = CpuType(15);
const CPU_TYPE_POWERPC: CpuType = CpuType(18);
const CPU_TYPE_POWERPC64: CpuType = CpuType(CPU_TYPE_POWERPC.0 | CPU_ARCH_ABI64);

impl fmt::Display for CpuType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cputype = match *self {
            CPU_TYPE_ANY => "ANY",
            CPU_TYPE_VAX => "VAX",
            CPU_TYPE_MC680X0 => "MC680X0",
            CPU_TYPE_X86 => "X86",
            CPU_TYPE_X86_64 => "X86_64",
            CPU_TYPE_MC98000 => "MC98000",
            CPU_TYPE_HPPA => "HPPA",
            CPU_TYPE_ARM => "ARM",
            CPU_TYPE_ARM64 => "ARM64",
            CPU_TYPE_MC88000 => "MC88000",
            CPU_TYPE_SPARC => "SPARC",
            CPU_TYPE_I860 => "I860",
            CPU_TYPE_POWERPC => "POWERPC",
            CPU_TYPE_POWERPC64 => "POWERPC64",
            _ => "unknown",
        };
        write!(f, "{}", cputype)
    }
}

/// Processor Subtype
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CpuSubType(pub u32);

const CPU_SUBTYPE_ARM64_ALL: CpuSubType = CpuSubType(0);
const CPU_SUBTYPE_ARM64_V8: CpuSubType = CpuSubType(1);
const CPU_SUBTYPE_ARM64E: CpuSubType = CpuSubType(2);
const CPU_SUBTYPE_X86_ALL: CpuSubType = CpuSubType(3);
const CPU_SUBTYPE_X86_ARCH1: CpuSubType = CpuSubType(4);
const CPU_SUBTYPE_X86_64_H: CpuSubType = CpuSubType(8);

impl fmt::Display for CpuSubType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let subtype = match *self {
            CPU_SUBTYPE_ARM64_ALL => "ARM64",
            CPU_SUBTYPE_ARM64_V8 => "ARM64v8",
            CPU_SUBTYPE_ARM64E => "ARM64e",
            CPU_SUBTYPE_X86_ALL => "X86",
            CPU_SUBTYPE_X86_ARCH1 => "X86_ARCH1",
            CPU_SUBTYPE_X86_64_H => "X86_64 Haswell",
            _ => "unknown",
        };
        write!(f, "{}", subtype)
    }
}
