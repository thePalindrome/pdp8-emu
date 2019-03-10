use std::error::Error;

struct Pdp {
    ac: u16, // note that all u16s are actually 12-bit, and I'll need to wrap them
    pc: u16,
    memory: [u16; 4096]
}

impl Pdp {
    fn new() -> Self {
        Self {
            ac: 0,
            pc: 0,
            memory: [0; 4096],
        }
    }
}

fn tick(pdp: &mut Pdp) {
    let mem = pdp.memory[pdp.pc as usize];

    if mem >> 9 < 6 { // If we're operating on a memory related instruction
        let offset = {
            let mut loc = mem % 128;
            if mem & 128 != 0 { // Relative
                loc += pdp.pc >> 7;
            }
            if mem & 256 != 0 { // Indirection
                loc = pdp.memory[loc as usize];
            }
            loc
        };
        match mem >> 9 {
            0 => {
                pdp.ac = pdp.ac & pdp.memory[offset as usize];
            },
            1 => {
                pdp.ac += pdp.memory[offset as usize];
            },
            2 => {
                pdp.memory[offset as usize] += 1;
                println!("{}",offset);
                println!("{}",pdp.memory[offset as usize]);
                if pdp.memory[offset as usize] == 4096 {
                    pdp.memory[offset as usize] = 0;
                    println!("foo");
                    pdp.pc += 1;
                }
            }
            3 => {
                pdp.memory[offset as usize] = pdp.ac;
                pdp.ac = 0;
            }
            4 => {
                pdp.memory[offset as usize] = pdp.pc;
                pdp.pc = offset;
            }
            5 => {
                pdp.pc = offset;
                return;
            }
            _ => panic!("Unimplemented memory opcode!"),
        };
    }
    pdp.pc += 1;
}

pub fn run(mem_file: String) -> Result<u16, Box<dyn Error>> {
    let mut pdp = Pdp::new();
    load_memory(mem_file, &mut pdp)?;

    tick(&mut pdp);

    Ok(pdp.ac)
}

fn load_memory(mem_file: String, pdp: &mut Pdp) -> Result<(), Box<dyn Error>> {
    let mut mem_counter = 0;
    for line in mem_file.lines() {
        pdp.memory[mem_counter] = line.parse::<u16>()?;
        mem_counter += 1;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn harness(mem_file: &str, ticks: u8) -> Pdp {
        let mut pdp = Pdp::new();
        let _ = load_memory(mem_file.to_string(), &mut pdp);

        for _i in 0..ticks {
            tick(&mut pdp);
        }

        pdp
    }

    #[test]
    fn simple_memory_load() {

        let pdp = harness("123\n456\n789",0);

        assert_eq!(
            pdp.memory[0..3],
            [123,456,789]
        );
    }

    #[test]
    fn acc_add_test() {

        let pdp = harness("513\n10",1);

        assert_eq!(
            pdp.ac,
            10
        );
    }

    #[test]
    fn acc_and_test() {

        let pdp = harness("514\n3\n3\n1",2);

        assert_eq!(
            pdp.ac,
            1
        );
    }
    #[test]
    fn indirect_add() {

        let pdp = harness("769\n2\n6",1);

        assert_eq!(
            pdp.ac,
            6
        );
    }

    #[test]
    fn incremet_skip_zero() {

        let pdp = harness("1027\n516\n517\n4095\n2\n3", 2);

        assert_eq!(
            pdp.ac,
            3
        );
    }

    #[test]
    fn deposit_clear_ac() {

        let pdp = harness("514\n1636\n123", 2);

        assert_eq!(
            pdp.ac,
            0
        );
        assert_eq!(
            pdp.memory[100],
            123
        );
    }

    #[test]
    fn jump() {

        let pdp = harness("2562\n512\n515\n123", 2);

        assert_eq!(
            pdp.ac,
            123
        );
    }

    #[test]
    fn jump_subroutine() {

        let pdp = harness("2050\n2050\n0\n517\n2818\n1", 6);

        assert_eq!(
            pdp.ac,
            2
        );
    }
}
