use std::fmt::Display;
use std::io::{self, BufRead};

pub mod perms;

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
            amp.unshift_input(output);
            amp.execute();
            amp.output.expect("No output from amp")
        })
    }

    pub fn exec_loop(&mut self) -> isize {
        let mut output = 0;
        let last_index = self.amps.len() - 1;
        loop {
            for (i, amp) in self.amps.iter_mut().enumerate() {
                amp.unshift_input(output);
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
    inst: Vec<isize>,
    input_mode: InputMode,
    inputs: Vec<isize>,
    output: Option<isize>,
    state: ComputerState,
    i_count: usize,
}

impl Computer {
    pub fn new(program: &str, input_mode: InputMode, inputs: Vec<isize>) -> Computer {
        Computer {
            inst: program.split(',').map(|s| s.parse().unwrap()).collect(),
            input_mode,
            state: ComputerState::Running,
            i_count: 0,
            inputs,
            output: None,
        }
    }

    pub fn execute(&mut self) {
        while self.i_count < self.inst.len() {
            let op = self.inst[self.i_count];
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
                    self.inst[dest] = result;
                    self.i_count += 4;
                }
                3 => {
                    let dest = self.inst[self.i_count + 1] as usize;
                    self.inst[dest] = self.get_input();
                    self.i_count += 2;
                }
                4 => {
                    let result =
                        self.get_value_with_mode(self.i_count + 1, params.pop().unwrap_or(0));
                    //println!("{}", result);
                    self.i_count += 2;
                    self.output = Some(result);
                    return;
                }
                5 | 6 => {
                    self.i_count = self.jump(opcode, self.i_count, params);
                }
                _ => panic!("what the opcode? {}", op),
            };
        }
    }

    fn execute_operation(&self, op: isize, i: usize, mut params: Vec<usize>) -> (isize, usize) {
        let arg1 = self.get_value_with_mode(i + 1, params.pop().unwrap_or(0));
        let arg2 = self.get_value_with_mode(i + 2, params.pop().unwrap_or(0));
        let dest = self.inst[i + 3] as usize;

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
        let arg1 = self.get_value_with_mode(i + 1, params.pop().unwrap_or(0));
        let arg2 = self.get_value_with_mode(i + 2, params.pop().unwrap_or(0));

        match op {
            5 if arg1 != 0 => arg2 as usize,
            6 if arg1 == 0 => arg2 as usize,
            _ => i + 3,
        }
    }

    fn get_value_with_mode(&self, i: usize, mode: usize) -> isize {
        if mode == 0 {
            self.inst[self.inst[i] as usize]
        } else {
            self.inst[i]
        }
    }

    fn unshift_input(&mut self, input: isize) {
        self.inputs.insert(0, input);
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
}

impl Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut state = String::new();
        for (i, el) in self.inst.iter().enumerate() {
            if i > 0 {
                state.push(',');
            }
            state.push_str(&el.to_string());
        }

        f.write_str(&state).unwrap();
        Ok(())
    }
}
