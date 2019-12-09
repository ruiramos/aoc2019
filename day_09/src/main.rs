use day_09::{Computer, InputMode};
use std::fs::File;
use std::io::Read;

fn main() {
    run_first();
}

fn run_tests() {
    let test_data = get_test_data();

    for test in test_data {
        let mut c1 = Computer::new(&test.i, InputMode::Stdin, vec![]);
        c1.execute();
    }
}

fn run_first() {
    let data = get_program();
    let mut c = Computer::new(&data, InputMode::Stdin, vec![]);
    c.execute();
}

fn run_second() {}

struct Test {
    i: String,
}

fn get_test_data() -> Vec<Test> {
    vec![
        Test {
            i: "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".to_string(),
        },
        Test {
            i: "1102,34915192,34915192,7,4,7,99,0".to_string(),
        },
        Test {
            i: "104,1125899906842624,99".to_string(),
        },
        Test {
            i: "109,-1,203,2,4,1,99".to_string(),
        },
    ]
}

fn get_program() -> String {
    let mut f = File::open("./data.txt").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    String::from(buffer.trim())
}
