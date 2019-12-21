use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("01. {}", &run_phases(&read_data(), 100)[0..8]);
    println!("02. {}", &get_message(&read_data()));
}

fn run_phases(input: &str, n: usize) -> String {
    let mut str_input = input.to_string();

    for i in 0..n {
        let len = str_input.len();
        let chars_vec = str_input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as isize)
            .collect::<Vec<isize>>();
        str_input = chars_vec
            .iter()
            .enumerate()
            .map(|(i, chars)| {
                let idxs = get_pattern_idx(i, len);
                idxs.iter()
                    .fold(0, |acc, (i, positive)| {
                        acc + (chars_vec[*i] * (if *positive { 1 } else { -1 }))
                    })
                    .to_string()
                    .chars()
                    .last()
                    .unwrap()
            })
            .collect::<String>();
    }

    str_input
}

fn get_message(n: &str) -> String {
    let location = &n[0..7].to_string().parse::<usize>().unwrap();
    let mut input = String::new();
    for _ in 0..10_000 {
        input.push_str(n);
    }

    let mut vec_input = input
        .chars()
        .skip(*location)
        .map(|e| e.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    vec_input.reverse();

    for i in 0..100 {
        let mut sum = 0;
        for v in (0..vec_input.len()) {
            sum += vec_input[v];
            vec_input[v] = sum % 10;
        }
    }

    vec_input.reverse();

    let result = vec_input
        .into_iter()
        .take(8)
        .map(|e| e.to_string())
        .collect::<Vec<String>>();

    result.join("")
}

fn get_pattern(n: usize, len: usize) -> Vec<isize> {
    let base = [0, 1, 0, -1];
    let mut ans = vec![];

    while ans.len() <= len {
        for b in base.iter() {
            for i in 0..=n {
                ans.push(*b as isize);
            }
        }
    }

    ans.remove(0);
    ans
}

fn get_pattern_idx(n: usize, len: usize) -> Vec<(usize, bool)> {
    let pat_reps = (len as f32 / (4 * (n + 1)) as f32).ceil() as usize;
    let pat_len = 4 * (n + 1);
    let mut result = vec![];

    for p in 0..pat_reps {
        for i in n..=(2 * n) {
            let idx1 = i + (p * pat_len);
            let idx2 = i + (p * pat_len) + pat_len / 2;
            if idx1 < len {
                result.push((idx1, true));
            }
            if idx2 < len {
                result.push((idx2, false));
            }
        }
    }

    result
}

fn read_data() -> String {
    let mut f = File::open("data.txt").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    String::from(buffer.trim())
}

#[cfg(test)]
mod test {
    use super::*;
    fn read_test_data1() -> Vec<(String, usize)> {
        vec![
            ("80871224585914546619083218645595".to_string(), 24176176),
            ("19617804207202209144916044189917".to_string(), 73745418),
            ("69317163492948606335995924319873".to_string(), 52432133),
        ]
    }

    fn read_test_data2() -> Vec<(String, String)> {
        vec![
            (
                "03036732577212944063491565474664".to_string(),
                "84462026".to_string(),
            ),
            (
                "02935109699940807407585447034323".to_string(),
                "78725270".to_string(),
            ),
            (
                "03081770884921959731165446850517".to_string(),
                "53553731".to_string(),
            ),
        ]
    }

    #[test]
    fn test_get_pattern() {
        assert_eq!(get_pattern(2, 5), vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1]);
        assert_eq!(get_pattern(2, 15).len(), 24 - 1);
    }

    #[test]
    fn tests_run_phases() {
        for test in read_test_data1() {
            assert_eq!(
                run_phases(&test.0, 100)[0..8].parse::<usize>().unwrap(),
                test.1
            );
        }
    }

    #[test]
    fn tests_get_message() {
        for test in read_test_data2() {
            println!("{}", run_phases(&test.0, 100));
            assert_eq!(get_message(&test.0), test.1);
        }
    }
}
