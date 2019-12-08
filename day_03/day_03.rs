use std::fs::File;
use std::io::Read;

fn main() {
    let inputs = vec![
        ("R8,U5,L5,D3", "U7,R6,D4,L4", 6, 30),
        (
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
            159,
            610,
        ),
        (
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            135,
            410,
        ),
    ];

    for input in inputs {
        let w1 = Wire::new(input.0);
        let w2 = Wire::new(input.1);
        let inter = w1.intercept(&w2);

        let (min_dist, point) = get_min_with_distance_fn(&inter, &|point| dist_to_origin(point));

        assert_eq!(min_dist, input.2);
        println!("{} ok", min_dist);

        let (min_sum_steps, point) = get_min_with_distance_fn(&inter, &|point| {
            let steps = w1.get_steps_to(point) + w2.get_steps_to(point);
            steps
        });

        assert_eq!(min_sum_steps, input.3);
        println!("{} {:?} ok", min_sum_steps, point);
    }

    // tests done

    let mut f = File::open("data.txt").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    let inputs: Vec<&str> = buffer.trim().split('\n').collect();

    let w1 = Wire::new(inputs[0]);
    let w2 = Wire::new(inputs[1]);
    let inter = w1.intercept(&w2);

    let (min_dist, point) = get_min_with_distance_fn(&inter, &|point| dist_to_origin(point));

    println!("01. {}", min_dist);

    let (min_sum_steps, point) = get_min_with_distance_fn(&inter, &|point| {
        let steps = w1.get_steps_to(point) + w2.get_steps_to(point);
        steps
    });

    println!("02. {} for {:?}", min_sum_steps, point);
}

fn get_min_with_distance_fn(
    inter: &Vec<(i32, i32)>,
    dist_fn: &dyn Fn((i32, i32)) -> u32,
) -> (u32, (i32, i32)) {
    let distances: Vec<u32> = inter.iter().cloned().map(|pos| dist_fn(pos)).collect();

    let mut min = distances[0];
    let mut point = Some(inter[0]);

    for (i, distance) in distances.iter().enumerate() {
        if *distance < min {
            min = *distance;
            point = Some(inter[i]);
        }
    }

    (min, point.expect("Point not found?"))
}

fn dist_to_origin(p: (i32, i32)) -> u32 {
    (p.0.abs() + p.1.abs()) as u32
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    R,
    U,
    L,
    D,
}

impl std::ops::Add<(i32, i32)> for Direction {
    type Output = (i32, i32);

    fn add(self, other: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::R => (other.0 + 1, other.1),
            Direction::U => (other.0, other.1 - 1),
            Direction::L => (other.0 - 1, other.1),
            Direction::D => (other.0, other.1 + 1),
        }
    }
}

type Move = (Direction, u32);

struct Wire {
    moves: Vec<Move>,
    positions: Vec<(i32, i32)>,
}

impl Wire {
    pub fn new(moves: &str) -> Wire {
        let mut pos = (0, 0);
        let mut positions: Vec<(i32, i32)> = vec![];
        let moves: Vec<Move> = moves
            .split(',')
            .map(|d| {
                (
                    match d.chars().nth(0).unwrap() {
                        'R' => Direction::R,
                        'U' => Direction::U,
                        'L' => Direction::L,
                        'D' => Direction::D,
                        _ => panic!("what"),
                    },
                    d[1..].parse::<u32>().unwrap(),
                )
            })
            .collect();

        for (d, m) in &moves {
            for i in 0..*m {
                let new_pos = *d + pos;
                positions.push(new_pos);
                pos = new_pos;
            }
        }

        Wire { moves, positions }
    }

    pub fn intercept(&self, other: &Wire) -> Vec<(i32, i32)> {
        self.positions
            .iter()
            .cloned()
            .filter(|pos| other.positions.contains(pos))
            .collect()
    }

    pub fn get_steps_to(&self, pos: (i32, i32)) -> u32 {
        let res = (self.positions.iter().position(|el| *el == pos).unwrap() as u32) + 1;
        res
    }
}
