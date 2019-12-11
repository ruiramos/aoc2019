use std::fs::File;
use std::io::Read;

const TRESHOLD: f64 = 0.00001;

fn main() {
    let map = parse_map(read_data()).0;
    let (val, pos) = get_max_visible(&map);
    println!("01. {}, at {:?}", val, map[pos]);
    assert_eq!(val, 284);

    let base = Asteroid {
        x: map[pos].x,
        y: map[pos].y,
    };

    let removed = vaporize_order(&map, base);

    for i in vec![0, 1, 2, 9, 19, 49, 99, 198, 199, 200, 298] {
        println!("{}: {:?}", (i + 1), map[*removed.get(i).unwrap() as usize]);
    }

    let target = &map[*removed.get(199).unwrap() as usize];
    assert_eq!(target.x, 4);
    assert_eq!(target.y, 4);
}

fn read_data() -> String {
    let mut f = File::open("data.txt").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    String::from(buffer.trim())
}

fn dist(x: usize, y: usize) {
    ()
}

#[derive(Debug)]
struct Asteroid {
    x: usize,
    y: usize,
}

impl Asteroid {
    pub fn new(x: usize, y: usize) -> Asteroid {
        Asteroid { x, y }
    }

    pub fn dist(&self, other: &Asteroid) -> f64 {
        let dx = other.x as isize - self.x as isize;
        let dy = other.y as isize - self.y as isize;
        ((dx.pow(2) + dy.pow(2)) as f64).sqrt()
    }

    pub fn acos(&self, other: &Asteroid) -> f64 {
        let dist = self.dist(other);
        let dx = other.x as isize - self.x as isize;
        let acos = (dx as f64 / dist).acos();
        acos
    }

    pub fn ang(&self, other: &Asteroid) -> f64 {
        let acos = self.acos(&other);
        if other.y <= self.y {
            // first or second quadrant
            acos
        } else {
            // third or fourth quadrant
            (2. * std::f64::consts::PI) - acos
        }
    }
}

fn parse_map(s: String) -> (Vec<Asteroid>, Option<Asteroid>) {
    let mut map = vec![];
    let mut base: Option<Asteroid> = None;
    let w = s.chars().position(|c| c == '\n').unwrap() + 1;

    for (i, c) in s.chars().enumerate() {
        match c {
            '#' => {
                map.push(Asteroid::new(i % w, i / w));
            }
            'X' => {
                // for test data
                base = Some(Asteroid { x: i % w, y: i / w });
            }
            _ => (),
        }
    }

    (map, base)
}

fn get_max_visible(map: &Vec<Asteroid>) -> (usize, usize) {
    let vec: Vec<usize> = map
        .iter()
        .map(|b| {
            let mut angs: Vec<f64> = vec![];

            map.iter().fold(0, |acc, a| {
                if a.x == b.x && a.y == b.y {
                    return acc;
                }

                let acos = b.acos(&a);
                let ang = b.ang(&a);

                if angs
                    .iter()
                    .filter(|other| (ang - *other).abs() < TRESHOLD)
                    .count()
                    == 0
                {
                    angs.push(ang);
                    acc + 1
                } else {
                    acc
                }
            })
        })
        .collect();

    let val = vec.iter().max().unwrap();
    let pos = vec.iter().position(|e| e == val).unwrap();
    (*val, pos)
}

fn vaporize_order(map: &Vec<Asteroid>, base: Asteroid) -> Vec<usize> {
    let mut angs: Vec<(f64, f64, usize, f64)> = map
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let original_ang = base.ang(a);

            // translate the referential
            let mut ang = std::f64::consts::PI / 2. - original_ang;
            if original_ang <= std::f64::consts::PI / 2. {
                ang = -3. * std::f64::consts::PI / 2. - original_ang;
            }

            (ang, base.dist(a), i, base.acos(a))
        })
        .collect();

    // sort by angle and then by distance
    angs.sort_by(|a, b| {
        if ((a.0 - b.0).abs() > TRESHOLD) {
            a.0.partial_cmp(&b.0).unwrap()
        } else {
            a.1.partial_cmp(&b.1).unwrap()
        }
    });

    // hacky way of removing yourself (dist = 0)
    angs.remove(0);

    let mut i = 0;
    let mut prev_ang: Option<f64> = None;
    let mut removal_idx: Vec<usize> = vec![];

    loop {
        let val = angs[i];

        match prev_ang {
            Some(v) if (v - val.0).abs() < TRESHOLD => {
                if angs.iter().filter(|a| a.0 != v).count() == 0 {
                    // remove anyway if we only have this angle left
                    removal_idx.push(val.2);
                    prev_ang = Some(val.0);
                    angs.remove(i);
                } else {
                    // move until we find a different angle
                    i += 1;
                }
            }
            _ => {
                removal_idx.push(val.2);
                prev_ang = Some(val.0);
                angs.remove(i);
            }
        }

        if angs.len() > 0 {
            i = i % angs.len();
        } else {
            break;
        }
    }

    removal_idx
}

#[cfg(test)]
mod test {
    use super::*;

    fn read_test_data() -> String {
        String::from(
            ".#..#
.....
#####
....#
...##",
        )
    }

    fn read_test_data2() -> String {
        String::from(
            ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.####X#####...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
        )
    }

    #[test]
    fn test_get_same_acos_for_aligned_asteriods() {
        let b = Asteroid::new(0, 0);
        let a1 = Asteroid::new(3, 1);
        let a2 = Asteroid::new(6, 2);
        let a3 = Asteroid::new(9, 3);
        assert_eq!(a1.acos(&b), a2.acos(&b));
        assert_eq!(a2.acos(&b), a3.acos(&b));
    }

    #[test]
    fn test_get_same_ang_for_aligned_asteriods() {
        let b = Asteroid::new(0, 0);
        let a1 = Asteroid::new(3, 1);
        let a2 = Asteroid::new(6, 2);
        let a3 = Asteroid::new(9, 3);
        assert_eq!(a1.ang(&b), a2.ang(&b));
        assert_eq!(a2.ang(&b), a3.ang(&b));
    }

    #[test]
    fn test_parse_map() {
        let map = parse_map(read_test_data()).0;
        assert_eq!(map.len(), 10);
    }

    #[test]
    fn test_map_asteroid_to_angules() {
        let map = parse_map(read_test_data()).0;
        let ans = get_max_visible(map);
        assert_eq!(ans, 8);
    }

}
