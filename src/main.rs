use std::env;
use pdp8_emu;

fn main() {
    // Parse a command line option to specify a memory file to load, then send the memory contents
    // to the lib to load themselves (that way tests are more easilly written)
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let input = std::fs::read_to_string(&args[1]).expect("Unable to read file");
        if let Err(e) = pdp8_emu::run(input) {
            // I dunno. Something amazing, I guess?
        }
    }
}
