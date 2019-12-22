use std::io;

pub struct IntcodeComputer {
    pub mem: Vec<i64>,
    cur_index: usize,
}

pub struct Operation {
    pub full_op: i64,
    pub op: i64,
    pub param_count: usize,
    pub param_modes: Vec<usize>,
}

impl Operation {
    pub fn new(operation: i64) -> Result<Operation, &'static str> {
        Ok(Operation {
            full_op: operation,
            op: Operation::op_from_full_op(operation)?,
            param_count: Operation::param_count_from_full_op(operation)?,
            param_modes: Operation::param_modes_from_full_op(operation)?,
        })
    }

    fn param_modes_from_full_op(operation: i64) -> Result<Vec<usize>, &'static str> {
        // This works because FromIterator is implemented for Result
        // This means we can create an iterator of Results and collapse them
        // into either a vector of Ok's or the first Err encountered
        let transforms: Result<Vec<usize>, &'static str> = operation
            .to_string()
            .chars()
            .map(|x| match x.to_digit(10) {
                Some(v) => Ok(v as usize),
                None => Err("Problem generating param modes"),
            })
            .collect();
        // Pull the vec<usize> out of Result
        let mut modes = match transforms {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        modes.pop();
        modes.pop();
        modes.reverse();
        modes.insert(0, 0);
        let param_count = Operation::param_count_from_full_op(operation)?;
        while modes.len() < param_count {
            modes.push(0);
        }
        Ok(modes)
    }

    fn param_count_from_full_op(operation: i64) -> Result<usize, &'static str> {
        match Operation::op_from_full_op(operation) {
            Ok(v) => Operation::operation_length(v),
            Err(e) => Err(e),
        }
    }

    fn op_from_full_op(operation: i64) -> Result<i64, &'static str> {
        if operation < 10 || operation == 99 {
            return Ok(operation);
        } else {
            match (match operation.to_string().chars().last() {
                Some(v) => v,
                None => return Err("op_from_full_op: Invalid operation"),
            })
            .to_digit(10)
            {
                Some(v) => Ok(v as i64),
                None => return Err("op_from_full_op: Invalid operation"),
            }
        }
    }

    pub fn operation_length(op: i64) -> Result<usize, &'static str> {
        match op {
            1 => Ok(4),
            2 => Ok(4),
            3 => Ok(2),
            4 => Ok(2),
            99 => Ok(0),
            _ => Err("Invalid Operation"),
        }
    }
}

impl IntcodeComputer {
    pub fn new() -> IntcodeComputer {
        IntcodeComputer {
            mem: Vec::new(),
            cur_index: 0,
        }
    }

    pub fn run(&mut self) -> Result<i64, &str> {
        loop {
            let operation = Operation::new(self.mem[self.cur_index])?;
            if operation.op == 99 {
                return self.handle_terminate();
            }
            match operation.op {
                1 => self.handle_add(&operation)?,
                2 => self.handle_multiply(&operation)?,
                3 => self.handle_int_input(&operation)?,
                4 => self.handle_int_output(&operation)?,
                _ => return self.handle_error(),
            }
            self.cur_index += operation.param_count;
        }
    }

    fn get_value_accounting_for_mode(
        &mut self,
        param_no: usize,
        operation: &Operation,
    ) -> Result<i64, &'static str> {
        match operation.param_modes.get(param_no) {
            Some(v) => match v {
                0 => Ok(self.mem[self.mem[self.cur_index + param_no] as usize]),
                1 => Ok(self.mem[self.cur_index + param_no]),
                _ => Err("Got param mode other than 0 or 1"),
            },
            None => Err("Failed to get param mode from vector"),
        }
    }

    fn handle_add(&mut self, operation: &Operation) -> Result<(), &'static str> {
        let val1 = self.get_value_accounting_for_mode(1, operation)?;
        let val2 = self.get_value_accounting_for_mode(2, operation)?;
        let dest = self.mem[self.cur_index + 3] as usize;
        self.mem[dest] = val1 + val2;
        Ok(())
    }

    fn handle_multiply(&mut self, operation: &Operation) -> Result<(), &'static str> {
        let val1 = self.get_value_accounting_for_mode(1, operation)?;
        let val2 = self.get_value_accounting_for_mode(2, operation)?;
        let dest = self.mem[self.cur_index + 3] as usize;
        self.mem[dest] = val1 * val2;
        Ok(())
    }

    fn handle_terminate(&self) -> Result<i64, &str> {
        Ok(self.mem[0])
    }

    fn handle_int_input(&mut self, _operation: &Operation) -> Result<(), &'static str> {
        let mut input = String::new();
        let dest = self.mem[self.cur_index + 1] as usize;
        println!("Input: ");
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => return Err("Something went bad during I/O")
        }
        input.pop();
        self.mem[dest] = match input.parse::<i64>() {
            Ok(v) => v,
            Err(_) => return Err("Failed to parse input into an int")
        };
        Ok(())
    }

    fn handle_int_output(&mut self, operation: &Operation) -> Result<(), &'static str> {
        let val = self.get_value_accounting_for_mode(1, operation)?;
        println!("Output: {}", val);
        Ok(())
    }

    fn handle_error(&self) -> Result<i64, &str> {
        Err("Bad Opcode")
    }
}
