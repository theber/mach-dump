use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::load_command::{
    ArmThreadState64, CommandType, LoadCommand, SegmentCommand64, ThreadCommand, LC_SEGMENT_64,
    LC_THREAD,
};
use crate::mach_header::MachHeader;
use crate::segment::Segment;

/// Main struct which representes a core dump
#[derive(Debug)]
pub struct Macho {
    /// Mach-O Header
    pub header: MachHeader,
    /// List of Segment64 and Thread Commands
    pub load_commands: Vec<CommandType>,
    /// Memory ranges which are stored in the core dump
    pub segments: Vec<Segment>,
}

impl Macho {
    /// Loads a core dump from disk and returns the parsed
    /// struct
    pub fn load<P: AsRef<Path>>(path: P) -> Option<Self> {
        let mut f = File::open(path).expect("Could not load Mach-O");
        let mut contents: Vec<u8> = vec![];
        f.read_to_end(&mut contents).expect("Read failed");
        let header: &MachHeader = &MachHeader::new(
            contents[..std::mem::size_of::<MachHeader>()]
                .try_into()
                .expect("Could not load header"),
        );

        let mut load_commands: Vec<CommandType> = Vec::with_capacity(header.ncmds as usize);
        let mut segments: Vec<Segment> = Vec::new();

        let mut lc_offset = std::mem::size_of::<MachHeader>();

        // Iterate over each load command
        for _nlc in 0..header.ncmds as usize {
            let lc = LoadCommand::new(&contents[lc_offset..lc_offset + 8].try_into().expect("AAA"));

            // Only care for LC_THREAD and LC_SEGMENT_64 right now
            // because they are only included in the core dump
            match lc.cmd {
                LC_THREAD => {
                    let command_buf: [u8; std::mem::size_of::<ThreadCommand>()] = contents
                        [lc_offset..lc_offset + std::mem::size_of::<ThreadCommand>()]
                        .try_into()
                        .unwrap();
                    let thread_command = ThreadCommand::new(&command_buf);
                    load_commands.push(CommandType::ThreadCommand(Box::new(thread_command)));
                }
                LC_SEGMENT_64 => {
                    // Create Load Command
                    let command_buf: [u8; std::mem::size_of::<SegmentCommand64>()] = contents
                        [lc_offset..lc_offset + std::mem::size_of::<SegmentCommand64>()]
                        .try_into()
                        .unwrap();
                    let seg64_command = SegmentCommand64::new(&command_buf);
                    load_commands.push(CommandType::SegmentCommand64(Box::new(seg64_command)));

                    // Add segment
                    let seg_buf: Vec<u8> = contents[seg64_command.fileoff as usize
                        ..(seg64_command.fileoff + seg64_command.filesize) as usize]
                        .to_owned();
                    let segment = Segment::new(
                        seg64_command.vmaddr as usize,
                        seg64_command.vmsize as usize,
                        seg64_command.maxprot as u8,
                        seg_buf,
                    );
                    segments.push(segment);
                }
                _ => println!("unsupported lc {:?}", lc.cmd),
            }
            lc_offset += lc.cmdsize as usize;
        }

        Some(Self {
            header: *header,
            load_commands,
            segments,
        })
    }

    /// Return a Vec of all thread states containing the
    /// general purpose registers in the core dump.
    pub fn get_threads(&self) -> Vec<ArmThreadState64> {
        let mut thread_states: Vec<ArmThreadState64> = Vec::new();
        for lc in &self.load_commands {
            if let CommandType::ThreadCommand(tc) = lc {
                thread_states.push(tc.state)
            }
        }
        thread_states
    }
}
