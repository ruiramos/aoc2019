use std::fs::File;
use std::io::Read;

fn main() {
    let data = read_data();

    let mut n_zeros = u32::max_value() as usize;
    let mut answer = 0;

    for layer in split_at(&data, 25 * 6) {
        let this_n_zeros = number_of('0', &layer);
        if this_n_zeros < n_zeros {
            answer = number_of('1', &layer) * number_of('2', &layer);
            n_zeros = this_n_zeros;
        }
    }

    println!("{}", answer);

    let (data, w, h) = (read_data(), 25, 6);
    let layers = split_at(&data, w * h);
    let image = combine_layers(layers);

    for split in split_at(&image, w) {
        println!(
            "{}",
            split
                .chars()
                .map(|c| {
                    if c == '0' {
                        ' '
                    } else {
                        'x'
                    }
                })
                .collect::<String>()
        );
    }
}

fn combine_layers(layers: Vec<&str>) -> String {
    let mut combined = String::new();
    let len = layers[0].len();

    for i in 0..len {
        combined.push(merge_layers(&layers, i));
    }

    combined
}

fn merge_layers(layers: &Vec<&str>, pos: usize) -> char {
    for layer in layers {
        let c = layer.chars().nth(pos).unwrap();
        if c != '2' {
            return c;
        }
    }
    '2'
}

fn split_at(data: &String, len: usize) -> Vec<&str> {
    data.as_bytes()
        .chunks(len)
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
}

fn number_of(c: char, s: &str) -> usize {
    s.chars().filter(|el| *el == c).count()
}

fn read_data() -> String {
    let mut f = File::open("data.txt").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    String::from(buffer.trim())
}

fn read_test_data() -> (String, usize, usize) {
    (String::from("0222112222120000"), 2, 2)
}
