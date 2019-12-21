use std::collections::HashMap;
use std::io;

pub struct IntcodeComputer {
    pub mem: Vec<i64>,
    cur_index: usize,
    full_op: i64,
    cur_op: i64,
    cur_param_modes: Vec<usize>,
    cur_param_count: usize,
    op_lengths: HashMap<i64, usize>,
}

impl IntcodeComputer {
    pub fn new() -> IntcodeComputer {
        IntcodeComputer {
            mem: Vec::new(),
            cur_index: 0,
            full_op: 0,
            cur_op: 0,
            cur_param_modes: vec![],
            cur_param_count: 0,
            op_lengths: [
                (1i64, 4usize),
                (2i64, 4usize),
                (3i64, 2usize),
                (4i64, 2usize),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }

    pub fn run(&mut self) -> Result<i64, &str> {
        loop {
            self.full_op = self.mem[self.cur_index];
            if self.full_op == 99 {
                return self.handle_terminate();
            }
            if self.full_op > 99 {
                self.cur_op = self.op_from_full_op();
                self.cur_param_count = self.param_count_for_op();
                self.cur_param_modes = self.param_modes_from_full_op();
            } else {
                self.cur_op = self.full_op;
                self.cur_param_count = self.param_count_for_op();
                self.cur_param_modes = vec![0; self.cur_param_count];
            }
            match self.cur_op {
                1 => self.handle_add(),
                2 => self.handle_multiply(),
                3 => self.handle_int_input(),
                4 => self.handle_int_output(),
                _ => return self.handle_error(),
            }
            self.cur_index += self.cur_param_count;
        }
    }

    fn param_count_for_op(&mut self) -> usize {
        *self.op_lengths.get(&self.cur_op).unwrap() as usize
    }

    fn param_modes_from_full_op(&mut self) -> Vec<usize> {
        let mut modes: Vec<usize> = self
            .full_op
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as usize)
            .collect();
        modes.pop();
        modes.pop();
        modes.reverse();
        modes.insert(0, 0);
        while modes.len() < self.cur_param_count {
            modes.push(0);
        }
        modes
    }

    fn op_from_full_op(&mut self) -> i64 {
        self.full_op
            .to_string()
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap() as i64
    }

    fn get_value_accounting_for_mode(&mut self, param_no: usize) -> i64 {
        assert!(param_no < self.cur_param_count);
        match self.cur_param_modes.get(param_no).unwrap() {
            0 => self.mem[self.mem[self.cur_index + param_no] as usize],
            1 => self.mem[self.cur_index + param_no],
            _ => panic!("Got param mode other than 0 or 1"),
        }
    }

    fn handle_add(&mut self) {
        let val1 = self.get_value_accounting_for_mode(1);
        let val2 = self.get_value_accounting_for_mode(2);
        let dest = self.mem[self.cur_index + 3] as usize;
        self.mem[dest] = val1 + val2;
    }

    fn handle_multiply(&mut self) {
        let val1 = self.get_value_accounting_for_mode(1);
        let val2 = self.get_value_accounting_for_mode(2);
        let dest = self.mem[self.cur_index + 3] as usize;
        self.mem[dest] = val1 * val2;
    }

    fn handle_terminate(&self) -> Result<i64, &str> {
        Ok(self.mem[0])
    }

    fn handle_int_input(&mut self) {
        let mut input = String::new();
        let dest = self.mem[self.cur_index + 1] as usize;
        println!("Input: ");
        io::stdin().read_line(&mut input);
        input.pop();
        self.mem[dest] = input.parse::<i64>().unwrap();
    }

    fn handle_int_output(&mut self) {
        let val = self.get_value_accounting_for_mode(1);
        println!("Output: {}", val);
    }

    fn handle_error(&self) -> Result<i64, &str> {
        Err("Bad Opcode")
    }
}
