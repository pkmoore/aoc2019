use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct IntcodeComputer {
    mem: Vec<i64>,
    cur_index: usize,
}

impl IntcodeComputer {
    fn run(&mut self) -> Result<i64, &str> {
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
        let address_1 = self.mem[self.cur_index + 1];
        assert!(address_1 >= 0);
        let val1 = self.mem[address_1 as usize];
        let address_2 = self.mem[self.cur_index + 2];
        assert!(address_2 >= 0);
        let val2 = self.mem[address_2 as usize];
        let dest = self.mem[self.cur_index + 3];
        self.mem[dest as usize] = val1 * val2;
    }

    fn handle_multiply(&mut self) {
        let address_1 = self.mem[self.cur_index + 1];
        assert!(address_1 >= 0);
        let val1 = self.mem[address_1 as usize];
        let address_2 = self.mem[self.cur_index + 2];
        assert!(address_2 >= 0);
        let val2 = self.mem[address_2 as usize];
        let dest = self.mem[self.cur_index + 3];
        self.mem[dest as usize] = val1 * val2;
    }

    fn handle_terminate(&self) -> Result<i64, &str> {
        Ok(self.mem[0])
    }

    fn handle_error(&self) -> Result<i64, &str> {
        Err("Bad Opcode")
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day5.txt")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader.read_line(&mut data)?;
    // Blindly delete trailing new line before parsing
    data.pop();

    let initial_mem = data
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut comp = IntcodeComputer {
        mem: initial_mem.clone(),
        cur_index: 0,
    };
    Ok(())
}
