use std::cmp;
use std::convert::TryInto;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};

fn main() {
    /*
    let test_inputs = vec![
        "3,3,1108,-1,8,3,4,3,99",
        "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9",
        "3,3,1105,-1,9,1101,0,0,12,4,12,99,1",
        "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"
    ];

    for test in test_inputs {
        let mut computer = Computer::new(test);
        computer.execute();
    }
    */

    let mut f = File::open("data.txt").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    let mut computer = Computer::new(buffer.trim());
    computer.execute();
}

struct Computer {
    inst: Vec<isize>,
}

impl Computer {
    pub fn new(input: &str) -> Computer {
        Computer {
            inst: input.split(',').map(|s| s.parse().unwrap()).collect(),
        }
    }

    pub fn execute(&mut self) {
        let mut i = 0;
        while i < self.inst.len() {
            let op = self.inst[i];
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
                    return;
                }
                1 | 2 | 7 | 8 => {
                    let (result, dest) = self.execute_operation(opcode, i, params);
                    self.inst[dest] = result;
                    i += 4;
                }
                3 => {
                    let dest = self.inst[i + 1] as usize;
                    let input = get_input();
                    self.inst[dest] = input;
                    i += 2;
                }
                4 => {
                    let result =
                        self.get_value_with_mode(i + 1, params.pop().unwrap_or(0)) as usize;
                    println!("{}", result);
                    i += 2;
                }
                5 | 6 => {
                    i = self.jump(opcode, i, params);
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

        f.write_str(&state);
        Ok(())
    }
}

fn get_input() -> isize {
    print!("> ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let line1 = stdin.lock().lines().next().unwrap().unwrap();
    line1.parse().expect("Expected a number")
}
