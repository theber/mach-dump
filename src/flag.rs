#![allow(unused)]

use std::fmt;

/// Contains which flags are set
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Flag(pub u32);

const MH_NOUNDEFS: Flag = Flag(0x1);
const MH_INCRLINK: Flag = Flag(0x2);
const MH_DYLDLINK: Flag = Flag(0x4);
const MH_BINDATLOAD: Flag = Flag(0x8);
const MH_PREBOUND: Flag = Flag(0x10);
const MH_SPLIT_SEGS: Flag = Flag(0x20);
const MH_LAZY_INIT: Flag = Flag(0x40);
const MH_TWOLEVEL: Flag = Flag(0x80);
const MH_FORCE_FLA: Flag = Flag(0x100);
const MH_NOMULTIDEFS: Flag = Flag(0x200);
const MH_NOFIXPREBINDIN: Flag = Flag(0x400);
const MH_PREBINDABLE: Flag = Flag(0x800);
const MH_ALLMODSBOUND: Flag = Flag(0x1000);
const MH_SUBSECTIONS_VIA_SYMBOLS: Flag = Flag(0x2000);
const MH_CANONICAL: Flag = Flag(0x4000);
const MH_WEAK_DEFINES: Flag = Flag(0x8000);
const MH_BINDS_TO_WEAK: Flag = Flag(0x10000);
const MH_ALLOW_STACK_EXECUTION: Flag = Flag(0x20000);
const MH_ROOT_SAFE: Flag = Flag(0x40000);
const MH_SETUID_SAFE: Flag = Flag(0x80000);
const MH_NO_REEXPORTED_DYLIBS: Flag = Flag(0x100000);
const MH_PIE: Flag = Flag(0x200000);
const MH_DEAD_STRIPPABLE_DYLIB: Flag = Flag(0x400000);
const MH_HAS_TLV_DESCRIPTORS: Flag = Flag(0x800000);
const MH_NO_HEAP_EXECUTION: Flag = Flag(0x1000000);
const MH_APP_EXTENSION_SAFE: Flag = Flag(0x02000000);
const MH_NLIST_OUTOFSYNC_WITH_DYLDINFO: Flag = Flag(0x04000000);
const MH_SIM_SUPPORT: Flag = Flag(0x08000000);
const MH_DYLIB_IN_CACHE: Flag = Flag(0x80000000);

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let flags: [Option<&str>; 32] = [
            Some("MH_NOUNDEFS: the object file has no undefined references"),
            Some("MH_INCRLINK: the object file is the output of an incremental link against a base \n\tfile and can't be link edited again"),
            Some("MH_DYLDLINK: the object file is input for the dynamic linker and can't be \n\tstaticly link edited again"),
            Some("MH_BINDATLOAD: the object file's undefined references are bound by the dynamic \n\tlinker when loaded"),
            Some("MH_PREBOUND: the file has its dynamic undefined references prebound"),
            Some("MH_SPLIT_SEGS: the file has its read-only and read-write segments split"),
            Some("MH_LAZY_INIT: the shared library init routine is to be run lazily via catching \n\tmemory faults to its writeable segments (obsolete)"),
            Some("MH_TWOLEVEL: the image is using two-level name space bindings"),
            Some("MH_FORCE_FLAT: the executable is forcing all images to use flat name space bindings"),
            Some("MH_NOMULTIDEFS: this umbrella guarantees no multiple defintions of symbols in \n\tits sub-images so the two-level namespace hints can always be used"),
            Some("MH_NOFIXPREBINDIN: do not have dyld notify the prebinding agent about this executable"),
            Some("MH_PREBINDABLE: the binary is not prebound but can have its prebinding redone. \n\tonly used when MH_PREBOUND is not set"),
            Some("MH_ALLMODSBOUND: indicates that this binary binds to all two-level namespace \n\tmodules of its dependent libraries. Only used when MH_PREBINDABLE and MH_TWOLEVEL are both set"),
            Some("MH_SUBSECTIONS_VIA_SYMBOLS: safe to divide up the sections into sub-sections via \n\tsymbols for dead code stripping"),
            Some("MH_CANONCIAL: the binary has been canonicalized via the unprebind operation"),
            Some("MH_WEAK_DEFINES: the final linked image contains external weak symbols"),
            Some("MH_BINDS_TO_WEAK: the final linked image uses weak symbols"),
            Some("MH_ALLOW_STACK_EXECUTION: When this bit is set, all stacks in the task will be \n\tgiven stack execution privilege. Only used in MH_EXECUTE filetypes"),
            Some("MH_ROOT_SAFE: When this bit is set, the binary declares it is safe for use in \n\tprocesses with uid zero"),
            Some("MH_SETUID_SAFE: When this bit is set, the binary declares it is safe for use in \n\tprocesses when issetugid() is true"),
            Some("MH_NO_REEXPORTED_DYLIBS: When this bit is set on a dylib, the static linker does \n\tnot need to examine dependent dylibs to see if any are re-exported"),
            Some("MH_PIE: When this bit is set, the OS will load the main executable at a random \n\taddress. Only used in MH_EXECUTE filetypes"),
            Some("MH_DEAD_STRIPPABLE_DYLIB: Only for use on dylibs. When linking against a dylib \n\tthat has this bit set, the static linker will automatically not create a LC_LOAD_DYLIB load command to the dylib if no symbols are being referenced from the dylib"),
            Some("MH_HAS_TLV_DESCRIPTORS: Contains a section of type S_THREAD_LOCAL_VARIABLES"),
            Some("MH_NO_HEAP_EXECUTION: When this bit is set, the OS will run the main executable \n\twith a non-executable heap even on platforms (e.g. i386) that don't require it. Only used in MH_EXECUTE filetypes"),
            Some("MH_APP_EXTENSION_SAFE: The code was linked for use in an application extension"),
            Some("MH_NLIST_OUTOFSYNC_WITH_DYLDINFO: The external symbols listed in the nlist symbol \n\ttable do not include all the symbols listed in the dyld info"),
            Some("MH_SIM_SUPPORT: Allow LC_MIN_VERSION_MACOS and LC_BUILD_VERSION load commands with \n\tthe platforms macOS, iOSMac, iOSSimulator, tvOSSimulator and watchOSSimulator"),
            Some("UNKNOWN"),
            Some("UNKNOWN"),
            Some("UNKNOWN"),
            Some("MH_DYLIB_IN_CACHE: Only for use on dylibs. When this bit is set, the dylib is part \n\tof the dyld shared cache, rather than loose in the filesystem"),
        ];

        for x in 0..u32::BITS {
            if (self.0 >> x) & 1 == 1 {
                writeln!(f, "\t{}", flags[x as usize].unwrap())?;
            }
        }

        write!(f, "")
    }
}
