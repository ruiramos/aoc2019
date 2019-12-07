use day_07::Circuit;
use permutohedron::Heap;
use std::fs::File;
use std::io::Read;

fn main() {
    run_first();
    run_second();
}

fn run_first() {
    let test_data = get_test_data();

    for test in test_data {
        let mut circuit = Circuit::new(&test.i, test.p.to_vec());
        let result = circuit.execute();
        assert_eq!(result, test.r);
        println!("{} ok", result);
    }

    // cool

    let data = get_program();
    let mut max = 0;
    let mut phase: Vec<usize> = (0..=4).collect();

    for perm in Heap::new(&mut phase) {
        let mut circuit = Circuit::new(&data, perm);
        let result = circuit.execute();
        max = isize::max(result, max);
    }

    println!("max: {}", max);
}

fn run_second() {
    let test_data = get_test_data_2();

    for test in test_data {
        let mut circuit = Circuit::new(&test.i, test.p.to_vec());
        let result = circuit.exec_loop();
        assert_eq!(result, test.r);
        println!("{} ok", result);
    }

    let data = get_program();
    let mut max = 0;
    let mut phase: Vec<usize> = (5..=9).collect();

    for perm in Heap::new(&mut phase) {
        let mut circuit = Circuit::new(&data, perm);
        let result = circuit.exec_loop();
        max = isize::max(result, max);
    }

    println!("max: {}", max);
}

struct Test {
    i: String,
    r: isize,
    p: [usize; 5],
}

fn get_test_data() -> Vec<Test> {
    vec![Test{
        i: "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".to_string(),
        r: 43210,
        p: [4,3,2,1,0],
    },
    Test {
        i: "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0".to_string(),
        r: 54321,
        p: [0,1,2,3,4]
    },
    Test {
        i: "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0".to_string(),
        r: 65210,
        p: [1,0,4,3,2]
    }]
}

fn get_test_data_2() -> Vec<Test> {
    vec![Test{
        i: "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5".to_string(), 
        r: 139629729,
        p: [9,8,7,6,5],
    },
    Test {
        i: "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10".to_string(),
        r: 18216,
        p: [9,7,8,5,6]
    },
    ]
}

fn get_program() -> String {
    let mut f = File::open("./data.txt").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    String::from(buffer.trim())
}
