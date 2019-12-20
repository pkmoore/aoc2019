use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct IntcodeComputer {
    mem: Vec<u64>,
    cur_index: usize,
}

impl IntcodeComputer {
    fn run(&mut self) -> Result<u64, &str> {
        loop {
            let op = self.mem[self.cur_index];
            match op {
                1 => self.handle_add(),
                2 => self.handle_multiply(),
                99 => return self.handle_terminate(),
                _ => return self.handle_error(),
            }
            self.cur_index += 4;
        }
    }

    fn handle_add(&mut self) {
        let val1 = self.mem[self.mem[self.cur_index + 1] as usize];
        let val2 = self.mem[self.mem[self.cur_index + 2] as usize];
        let dest = self.mem[self.cur_index + 3] as usize;
        self.mem[dest] = val1 + val2;
    }

    fn handle_multiply(&mut self) {
        let val1 = self.mem[self.mem[self.cur_index + 1] as usize];
        let val2 = self.mem[self.mem[self.cur_index + 2] as usize];
        let dest = self.mem[self.cur_index + 3] as usize;
        self.mem[dest] = val1 * val2;
    }

    fn handle_terminate(&self) -> Result<u64, &str> {
        Ok(self.mem[0])
    }

    fn handle_error(&self) -> Result<u64, &str> {
        Err("Bad Opcode")
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day2.txt")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader.read_line(&mut data)?;
    // Blindly delete trailing new line before parsing
    data.pop();

    let initial_mem = data
        .split(",")
        .map(|m| m.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut comp = IntcodeComputer {
                mem: initial_mem.clone(),
                cur_index: 0,
            };
            comp.mem[1] = noun;
            comp.mem[2] = verb;
            match comp.run() {
                Ok(v) => {
                    if v == 19690720 {
                        println!("(noun, verb) => ({},{})", noun, verb);
                        println!("100 * noun + verb = {}", 100 * noun + verb);
                        break;
                    }
                }
                Err(e) => println!("{}", e),
            }
        }
    }
    Ok(())
}
