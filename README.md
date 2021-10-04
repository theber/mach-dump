# mach-dump

mach-dump can parse Mach-O core dumps taken with lldb from macOS and iOS 
devices. It has no external dependencies.

# Example

```rust
use std::path::Path;
use mach_dump::macho::Macho;
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("Usage: {} <mach-o>", args[0]);
    }

    let macho = Macho::load(Path::new(&args[1])).unwrap();
    println!("{}", macho.header);
    for (i, lc) in macho.load_commands.into_iter().enumerate() {
        println!("LC {:02}: {:?}", i, lc);
    }
    for (i, seg) in macho.segments.into_iter().enumerate() {
        println!("Segment {:02}:\n{}", i, seg);
    }
}
```
