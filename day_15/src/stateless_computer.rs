use crate::ComputerState;
use std::collections::HashMap;

pub struct StatelessComputer {}

pub type Program = HashMap<usize, isize>;
type ComputerOutput<'a> = (Option<isize>, ComputerState, &'a mut Program, usize, isize);

impl StatelessComputer {
    pub fn execute(
        program: &mut Program,
        mut i_count: usize,
        mut r_count: isize,
        input: Option<isize>,
    ) -> ComputerOutput {
        let op = program.get(&i_count).expect("Error unwrapping op");
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
            99 => (None, ComputerState::Terminated, program, i_count, r_count),
            1 | 2 | 7 | 8 => {
                let (result, dest) = StatelessComputer::execute_operation(
                    &program,
                    opcode,
                    i_count,
                    r_count,
                    &mut params,
                );
                program.insert(dest, result);
                i_count += 4;
                (None, ComputerState::Running, program, i_count, r_count)
            }
            3 => {
                let dest = StatelessComputer::resolve_destination(
                    &program,
                    i_count + 1,
                    r_count,
                    params.pop(),
                );
                let input = input.expect("None unwraping input");
                program.insert(dest, input);
                i_count += 2;
                (None, ComputerState::Running, program, i_count, r_count)
            }
            4 => {
                let result = StatelessComputer::get_value_with_mode(
                    &program,
                    i_count + 1,
                    r_count,
                    params.pop(),
                );
                i_count += 2;
                (
                    Some(result),
                    ComputerState::Running,
                    program,
                    i_count,
                    r_count,
                )
            }
            5 | 6 => {
                i_count = StatelessComputer::jump(&program, opcode, i_count, r_count, &mut params);
                (None, ComputerState::Running, program, i_count, r_count)
            }
            9 => {
                let arg1 = StatelessComputer::get_value_with_mode(
                    &program,
                    i_count + 1,
                    r_count,
                    params.pop(),
                );
                i_count += 2;
                r_count += arg1;
                (None, ComputerState::Running, program, i_count, r_count)
            }
            _ => panic!("what the opcode? {}", op),
        }
    }

    fn execute_operation(
        program: &Program,
        opcode: isize,
        i_count: usize,
        r_count: isize,
        params: &mut Vec<usize>,
    ) -> (isize, usize) {
        let arg1 =
            StatelessComputer::get_value_with_mode(program, i_count + 1, r_count, params.pop());
        let arg2 =
            StatelessComputer::get_value_with_mode(program, i_count + 2, r_count, params.pop());
        let dest =
            StatelessComputer::resolve_destination(program, i_count + 3, r_count, params.pop());

        let result = match opcode {
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

    fn resolve_destination(
        program: &Program,
        i_count: usize,
        r_count: isize,
        mode: Option<usize>,
    ) -> usize {
        match mode {
            Some(2) => (program[&i_count] as isize + r_count) as usize,
            _ => program[&i_count] as usize,
        }
    }

    fn get_value_with_mode(
        program: &Program,
        i_count: usize,
        r_count: isize,
        mode: Option<usize>,
    ) -> isize {
        let i_op = program.get(&i_count).unwrap_or(&0);
        let result = match mode {
            None => program.get(&(*i_op as usize)),
            Some(n) if n == 0 => program.get(&(*i_op as usize)),
            Some(n) if n == 1 => Some(i_op),
            Some(n) if n == 2 => program.get(&((r_count + i_op) as usize)),
            _ => panic!("invalid mode"),
        };

        *result.unwrap_or(&0)
    }

    fn jump(
        program: &Program,
        opcode: isize,
        i_count: usize,
        r_count: isize,
        params: &mut Vec<usize>,
    ) -> usize {
        let arg1 =
            StatelessComputer::get_value_with_mode(&program, i_count + 1, r_count, params.pop());
        let arg2 =
            StatelessComputer::get_value_with_mode(&program, i_count + 2, r_count, params.pop());

        match opcode {
            5 if arg1 != 0 => arg2 as usize,
            6 if arg1 == 0 => arg2 as usize,
            _ => i_count + 3,
        }
    }
}
