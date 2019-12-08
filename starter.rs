use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("code here");
}

fn read_data() -> String {
    let mut f = File::open("data.txt").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    String::from(buffer.trim())
}

#[cfg(test)]
mod test {
    fn read_test_data() -> String {
        String::from("hello,world")
    }

    #[test]
    fn tests_run() {
        assert_eq!(read_test_data().split(',').count(), 2);
    }
}
