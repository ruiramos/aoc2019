fn main() {
    assert_eq!(two_adjacent(111111), true);
    assert_eq!(two_adjacent(223450), true);
    assert_eq!(two_adjacent(123789), false);

    assert_eq!(ascending_numbers(111111), true);
    assert_eq!(ascending_numbers(223450), false);
    assert_eq!(ascending_numbers(123789), true);

    assert_eq!(two_adjacent_group(112233), true);
    assert_eq!(two_adjacent_group(123444), false);
    assert_eq!(two_adjacent_group(111122), true);

    let input = "130254-678275";
    let split: Vec<u32> = input
        .split('-')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let min = split[0];
    let max = split[1];

    let mut counter = 0;
    let mut counter2 = 0;
    for i in min..max {
        if two_adjacent(i) && ascending_numbers(i) {
            counter += 1;
        }
        if two_adjacent_group(i) && ascending_numbers(i) {
            counter2 += 1;
        }
    }

    println!("1. {} \n2. {}", counter, counter2);
}

fn two_adjacent(n: u32) -> bool {
    let string = n.to_string();
    let mut prev = None;
    for (i, c) in string.chars().enumerate() {
        if prev.is_some() {
            if c == prev.unwrap() {
                return true;
            }
        }
        prev = Some(c);
    }

    return false;
}

fn two_adjacent_group(n: u32) -> bool {
    let string = n.to_string();
    let chars: Vec<char> = string.chars().collect();

    let mut i = 0;
    let mut j = 0;
    let mut counter = 0;

    while i < chars.len() {
        j = i;
        counter = 0;

        while j < chars.len() && chars[i] == chars[j] {
            counter += 1;
            j += 1;
        }

        if counter == 2 {
            return true;
        } else {
            i += counter;
        }
    }

    return false;
}

fn ascending_numbers(n: u32) -> bool {
    let string = n.to_string();
    let mut min = 0;
    for c in string.chars() {
        let n = c.to_string().parse::<u32>().unwrap();
        if n < min {
            return false;
        }
        min = n;
    }

    return true;
}
