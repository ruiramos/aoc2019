use std::fmt::Display;

fn main() {
    let test_inputs = vec![
        vec![
            "1,9,10,3,2,3,11,0,99,30,40,50",
            "3500,9,10,70,2,3,11,0,99,30,40,50",
        ],
        vec!["1,0,0,0,99", "2,0,0,0,99"],
        vec!["2,3,0,3,99", "2,3,0,6,99"],
        vec!["2,4,4,5,99,0", "2,4,4,5,99,9801"],
        vec!["1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99"],
    ];

    for test in test_inputs {
        if let &[input, expected] = &*test {
            let mut computer = Computer::new(input);
            computer.execute();
            assert_eq!(computer.to_string(), expected);
        }
    }

    // all tests ok!
    let mut computer = Computer::new(&get_input(12, 2));
    computer.execute();

    println!("{}", split_to_comma(computer.to_string()));

    let target = 19690720;
    let mut values = (None, None);

    for i in 0..99 {
        for j in 0..99 {
            let mut computer = Computer::new(&get_input(i, j));
            computer.execute();
            if computer.get_first_value().parse::<usize>().unwrap() == target {
                values = (Some(i), Some(j));
            }
        }
    }

    println!("{:?}", values);
}

struct Computer {
    inst: Vec<usize>,
}

impl Computer {
    pub fn new(input: &str) -> Computer {
        Computer {
            inst: input.split(',').map(|s| s.parse().unwrap()).collect(),
        }
    }

    pub fn execute(&mut self) {
        let mut i = 0;
        while i < self.inst.len() && self.inst[i] != 99 {
            let op = self.inst[i];

            match op {
                99 => {
                    return;
                }
                1 | 2 => {
                    let arg1 = self.inst[self.inst[i + 1]];
                    let arg2 = self.inst[self.inst[i + 2]];
                    let dest = self.inst[i + 3];

                    let result = self.execute_op(op, Some(arg1), Some(arg2));

                    self.inst[dest] = result;

                    i += 4;
                }
                _ => panic!("What the opcode?!"),
            };
        }
    }

    pub fn execute_op(&mut self, op: usize, arg1: Option<usize>, arg2: Option<usize>) -> usize {
        match op {
            1 => arg1.unwrap() + arg2.unwrap(),
            2 => arg1.unwrap() * arg2.unwrap(),
            _ => 0,
        }
    }

    pub fn get_first_value(&self) -> String {
        split_to_comma(self.to_string())
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

fn get_input(p1: usize, p2: usize) -> String {
    format!("1,{},{},3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,10,19,1,6,19,23,1,10,23,27,2,27,13,31,1,31,6,35,2,6,35,39,1,39,5,43,1,6,43,47,2,6,47,51,1,51,5,55,2,55,9,59,1,6,59,63,1,9,63,67,1,67,10,71,2,9,71,75,1,6,75,79,1,5,79,83,2,83,10,87,1,87,5,91,1,91,9,95,1,6,95,99,2,99,10,103,1,103,5,107,2,107,6,111,1,111,5,115,1,9,115,119,2,119,10,123,1,6,123,127,2,13,127,131,1,131,6,135,1,135,10,139,1,13,139,143,1,143,13,147,1,5,147,151,1,151,2,155,1,155,5,0,99,2,0,14,0", p1, p2)
}

fn split_to_comma(input: String) -> String {
    input[0..input.find(',').unwrap_or(input.len())].to_string()
}
