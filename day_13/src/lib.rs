use std::collections::HashMap;
use std::fmt::Display;
use std::io::{self, BufRead};

pub mod vis;

pub struct Circuit {
    amps: Vec<Computer>,
}

impl Circuit {
    pub fn new(input: &str, phases: Vec<u32>) -> Circuit {
        let mut amps = Vec::new();

        for i in 0..phases.len() {
            amps.push(Computer::new(
                input,
                InputMode::Args,
                vec![phases[i] as isize],
            ));
        }

        Circuit { amps }
    }

    pub fn execute(&mut self) -> isize {
        self.amps.iter_mut().fold(0, |output, amp| {
            amp.send_input(output);
            amp.execute();
            amp.output.expect("No output from amp")
        })
    }

    pub fn exec_loop(&mut self) -> isize {
        let mut output = 0;
        let last_index = self.amps.len() - 1;
        loop {
            for (i, amp) in self.amps.iter_mut().enumerate() {
                amp.send_input(output);
                amp.execute();
                output = amp.output.expect("No output from amp");

                if i == last_index && amp.state == ComputerState::Terminated {
                    return output;
                }
            }
        }
    }
}

pub enum InputMode {
    Stdin,
    Args,
}

#[derive(PartialEq)]
pub enum ComputerState {
    Running,
    Terminated,
}

pub struct Computer {
    inst: HashMap<usize, isize>,
    input_mode: InputMode,
    inputs: Vec<isize>,
    output: Option<isize>,
    state: ComputerState,
    i_count: usize,
    r_pointer: isize,
}

impl Computer {
    pub fn new(program: &str, input_mode: InputMode, inputs: Vec<isize>) -> Computer {
        Computer {
            inst: program
                .split(',')
                .enumerate()
                .fold(HashMap::new(), |mut map, (i, s)| {
                    map.insert(i, s.parse().unwrap());
                    map
                }),
            input_mode,
            state: ComputerState::Running,
            i_count: 0,
            r_pointer: 0,
            inputs,
            output: None,
        }
    }

    pub fn execute(&mut self) {
        while self.i_count < self.inst.len() {
            let op = self.inst.get(&self.i_count).expect("Error unwrapping op");
            let opcode: isize = op % 100;

            let mut params: Vec<usize> = op
                .to_string()
                .chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect();

            // pops the opcode out
            params.pop();
            params.pop();

            match opcode {
                99 => {
                    self.state = ComputerState::Terminated;
                    return;
                }
                1 | 2 | 7 | 8 => {
                    let (result, dest) = self.execute_operation(opcode, self.i_count, params);
                    self.inst.insert(dest, result);
                    self.i_count += 4;
                }
                3 => {
                    let dest = self.resolve_destination(self.i_count + 1, params.pop());
                    let input = self.get_input();
                    self.inst.insert(dest, input);
                    self.i_count += 2;
                }
                4 => {
                    let result = self.get_value_with_mode(self.i_count + 1, params.pop());
                    //println!("{}", result);
                    self.i_count += 2;
                    self.output = Some(result);
                    return;
                }
                5 | 6 => {
                    self.i_count = self.jump(opcode, self.i_count, params);
                }
                9 => {
                    let arg1 = self.get_value_with_mode(self.i_count + 1, params.pop());
                    self.r_pointer += arg1;
                    self.i_count += 2;
                }
                _ => panic!("what the opcode? {}", op),
            };
        }
    }

    fn execute_operation(&self, op: isize, i: usize, mut params: Vec<usize>) -> (isize, usize) {
        let arg1 = self.get_value_with_mode(i + 1, params.pop());
        let arg2 = self.get_value_with_mode(i + 2, params.pop());
        let dest = self.resolve_destination(i + 3, params.pop());

        let result = match op {
            1 => arg1 + arg2,
            2 => arg1 * arg2,
            7 => {
                if arg1 < arg2 {
                    1
                } else {
                    0
                }
            }
            8 => {
                if arg1 == arg2 {
                    1
                } else {
                    0
                }
            }
            _ => panic!("Wrong opcode given to execute_operation"),
        };

        (result, dest)
    }

    fn jump(&self, op: isize, i: usize, mut params: Vec<usize>) -> usize {
        let arg1 = self.get_value_with_mode(i + 1, params.pop());
        let arg2 = self.get_value_with_mode(i + 2, params.pop());

        match op {
            5 if arg1 != 0 => arg2 as usize,
            6 if arg1 == 0 => arg2 as usize,
            _ => i + 3,
        }
    }

    fn get_value_with_mode(&self, i: usize, mode: Option<usize>) -> isize {
        let i_op = self.inst.get(&i).unwrap_or(&0);
        let result = match mode {
            None => self.inst.get(&(*i_op as usize)),
            Some(n) if n == 0 => self.inst.get(&(*i_op as usize)),
            Some(n) if n == 1 => Some(i_op),
            Some(n) if n == 2 => self.inst.get(&((self.r_pointer + i_op) as usize)),
            _ => panic!("invalid mode"),
        };

        *result.unwrap_or(&0)
    }

    fn resolve_destination(&self, i: usize, mode: Option<usize>) -> usize {
        match mode {
            Some(2) => (self.inst[&i] as isize + self.r_pointer) as usize,
            _ => self.inst[&i] as usize,
        }
    }

    pub fn send_input(&mut self, input: isize) {
        self.inputs.insert(0, input);
    }

    pub fn set_input(&mut self, input: isize) {
        self.inputs = vec![input];
    }

    fn get_input(&mut self) -> isize {
        match self.input_mode {
            InputMode::Stdin => {
                let stdin = io::stdin();
                let line1 = stdin.lock().lines().next().unwrap().unwrap();
                line1.parse().expect("Expected a number")
            }
            InputMode::Args => self.inputs.pop().expect("Poped empty input array"),
        }
    }

    pub fn is_terminated(&self) -> bool {
        self.state == ComputerState::Terminated
    }

    pub fn get_output(&self) -> Option<isize> {
        self.output
    }
}

impl Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut state = String::new();
        state.push_str("\n>>\n");
        for (i, el) in self.inst.iter().enumerate() {
            state.push_str(&format!(">{}: {}\n", el.0, el.1));
        }

        f.write_str(&state).unwrap();
        Ok(())
    }
}
