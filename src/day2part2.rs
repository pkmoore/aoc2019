use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct IntcodeComputer {
    mem: Vec<u64>,
    cur_index: usize
}

impl IntcodeComputer {
    fn run(&mut self) {
        loop {
            let op = self.mem[self.cur_index];
            match op {
                1 => self.handle_add(),
                2 => self.handle_multiply(),
                99 => break,
                _ => self.handle_error(),
            }
            self.cur_index += 4;
        }
        self.handle_terminate();
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

    fn handle_terminate(&self) {
        println!("Day 2 Part 1 position 0: {}", self.mem[0]);
    }

    fn handle_error(&self) {
        println!(
            "Error: Got bad opcode {} at position {}",
            self.mem[self.cur_index], self.cur_index
        );
        println!("{:?}", self.mem);
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day2.txt")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader.read_line(&mut data)?;
    // Blindly delete trailing new line before parsing
    data.pop();
    let mut comp = IntcodeComputer {
        mem: data
            .split(",")
            .map(|m| m.parse::<u64>().unwrap())
            .collect::<Vec<u64>>(),
        cur_index: 0
    };
    // "Fix" the program
    comp.mem[1] = 12;
    comp.mem[2] = 2;
    comp.run();
    Ok(())
}
