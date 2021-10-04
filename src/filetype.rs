use std::fmt;

/// Identifies file type of Mach-O, should be 
/// `MH_CORE` for core dumps
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FileType(pub u32);

const MH_OBJECT: FileType = FileType(1);
const MH_EXECUTE: FileType = FileType(2);
const MH_FVMLIB: FileType = FileType(3);
const MH_CORE: FileType = FileType(4);
const MH_PRELOAD: FileType = FileType(5);
const MH_DYLIB: FileType = FileType(6);
const MH_DYLINKER: FileType = FileType(7);
const MH_BUNDLE: FileType = FileType(8);
const MH_DYLIB_STUB: FileType = FileType(9);
const MH_DSYM: FileType = FileType(10);
const MH_KEXT_BUNDLE: FileType = FileType(11);
const MH_FILESET: FileType = FileType(12);

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let filetype = match *self {
            MH_OBJECT => "MH_OBJECT (relocatable object file)",
            MH_EXECUTE => "MH_EXECUTE (demand paged executable file)",
            MH_FVMLIB => "MH_FVMLIB (fixed VM shared library file)",
            MH_CORE => "MH_CORE (core file)",
            MH_PRELOAD => "MH_PRELOAD (preloaded executable file)",
            MH_DYLIB => "MH_DYLIB (dynamically bound shared library)",
            MH_DYLINKER => "MH_DYLINKER (dynamic link editor)",
            MH_BUNDLE => "MH_BUNDLE (dynamically bound bundle file)",
            MH_DYLIB_STUB => {
                "MH_DYLIB_STUB (shared library stub for static\
            linking only, no section contents)"
            }
            MH_DSYM => "MH_DSYM (companion file with only debug sections)",
            MH_KEXT_BUNDLE => "MH_KEXT_BUNDLE (x86_64 kexts)",
            MH_FILESET => "MH_FILESET (set of mach-o's)",
            _ => "unknown",
        };

        write!(f, "{}", filetype)
    }
}
