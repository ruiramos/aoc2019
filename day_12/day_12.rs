// cargo-deps: num = "0.2"
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut moons = create_moons(read_data());

    let mut xs: Vec<Vec<isize>> = vec![vec![], vec![], vec![], vec![]];
    let mut ys: Vec<Vec<isize>> = vec![vec![], vec![], vec![], vec![]];
    let mut zs: Vec<Vec<isize>> = vec![vec![], vec![], vec![], vec![]];

    for i in 0..1_000_000 {
        for (i, moon) in moons.iter().enumerate() {
            let (x, y, z) = (moon.position[0], moon.position[1], moon.position[2]);
            xs[i].push(x);
            ys[i].push(y);
            zs[i].push(z);
        }

        update_moons(&mut moons);

        if i == 999 {
            println!("01. {}", moons.iter().fold(0, |acc, m| acc + m.get_enery()));
        }
    }

    let x_period = xs.iter().map(|e| calc_period(e.to_vec())).max().unwrap();
    let y_period = ys.iter().map(|e| calc_period(e.to_vec())).max().unwrap();
    let z_period = zs.iter().map(|e| calc_period(e.to_vec())).max().unwrap();

    println!(
        "02. {:?} {:?} {:?} {}",
        x_period,
        y_period,
        z_period,
        num::integer::lcm(x_period, num::integer::lcm(y_period, z_period))
    );
}

fn calc_period(c: Vec<isize>) -> usize {
    let first = c[0];
    let mut min_index = 1;
    loop {
        let repeat_idx = c
            .iter()
            .skip(min_index)
            .position(|e| *e == first)
            .expect("cant find element")
            + min_index;
        for i in 0..repeat_idx {
            if c[i] != c[repeat_idx + i] {
                min_index = repeat_idx + 1;
                break;
            }
            if i == repeat_idx - 1 {
                return repeat_idx;
            }
        }
    }
}

fn read_data() -> String {
    let mut f = File::open("data.txt").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    String::from(buffer.trim())
}

fn create_moons(s: String) -> Vec<Moon> {
    let split = s.split('\n');
    let mut moons: Vec<Moon> = vec![];

    for m in split {
        moons.push(Moon::parse(m.trim()));
    }

    moons
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Moon {
    position: [isize; 3],
    velocity: [isize; 3],
}

impl Moon {
    pub fn new() -> Moon {
        Moon {
            position: [0; 3],
            velocity: [0; 3],
        }
    }
    pub fn parse(s: &str) -> Moon {
        let mut position = [0, 0, 0];

        for (i, coord) in s[1..s.len() - 1].split(',').enumerate() {
            let parts: Vec<&str> = coord.split('=').collect();
            position[i] = parts[1]
                .parse::<isize>()
                .expect("Error parsing moon coords");
        }

        Moon {
            position,
            velocity: [0, 0, 0],
        }
    }

    pub fn apply_velocity(&mut self) {
        for (i, v) in self.velocity.iter().enumerate() {
            self.position[i] += v;
        }
    }

    pub fn get_enery(&self) -> usize {
        self.position
            .iter()
            .map(|p| p.abs() as usize)
            .sum::<usize>()
            * self
                .velocity
                .iter()
                .map(|p| p.abs() as usize)
                .sum::<usize>()
    }
}

pub fn apply_gravity(m1: Moon, m2: Moon) -> (Moon, Moon) {
    let mut m1c = m1.clone();
    let mut m2c = m2.clone();

    for (i, p) in m1c.position.iter_mut().enumerate() {
        if *p > m2c.position[i] {
            m1c.velocity[i] -= 1;
            m2c.velocity[i] += 1;
        } else if *p < m2c.position[i] {
            m1c.velocity[i] += 1;
            m2c.velocity[i] -= 1;
        }
    }

    (m1c, m2c)
}

fn update_moons(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        for j in i + 1..moons.len() {
            let (m1, m2) = apply_gravity(moons[i], moons[j]);
            moons[i] = m1;
            moons[j] = m2;
        }
    }

    for m in moons.iter_mut() {
        m.apply_velocity();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn read_test_data() -> String {
        String::from(
            "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>",
        )
    }

    fn read_test_data2() -> String {
        String::from(
            "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>",
        )
    }

    #[test]
    fn creates_a_moon() {
        let m = Moon::parse("<x=-1, y=0, z=2>");
        assert_eq!(m.position, [-1, 0, 2]);
        assert_eq!(m.velocity, [0, 0, 0]);
    }

    #[test]
    fn creates_moons_vec() {
        let moons = create_moons(read_test_data());
        assert_eq!(moons.len(), 4);
        assert_eq!(moons[1].position, [2, -10, -7]);
    }

    #[test]
    fn iterates() {
        let mut moons = create_moons(read_test_data());
        update_moons(&mut moons);
        assert_eq!(moons[1].position, [3, -7, -4]);
        assert_eq!(moons[1].velocity, [1, 3, 3]);
    }

    #[test]
    fn calc_energy() {
        let mut moons = create_moons(read_test_data());
        for i in 0..10 {
            update_moons(&mut moons);
        }
        assert_eq!(moons.iter().fold(0, |acc, m| acc + m.get_enery()), 179);
    }

    #[test]
    fn calc_energy_2() {
        let mut moons = create_moons(read_test_data2());
        for i in 0..100 {
            update_moons(&mut moons);
        }
        assert_eq!(moons.iter().fold(0, |acc, m| acc + m.get_enery()), 1940);
    }

    #[test]
    fn simulate() {
        let mut universes = HashSet::new();
        let mut moons = create_moons(read_test_data());
        let mut i = 0;
        loop {
            update_moons(&mut moons);
            let new_moons = moons.clone();
            if universes.get(&new_moons).is_some() {
                break;
            }
            universes.insert(new_moons);
            i += 1;
        }
        assert_eq!(i, 2772);
    }

    #[test]
    fn test_calc_period() {
        let mut xs: Vec<Vec<isize>> = vec![vec![], vec![], vec![], vec![]];
        let mut ys: Vec<Vec<isize>> = vec![vec![], vec![], vec![], vec![]];
        let mut zs: Vec<Vec<isize>> = vec![vec![], vec![], vec![], vec![]];
        let mut moons = create_moons(read_test_data());

        for i in 0..100 {
            for (i, moon) in moons.iter().enumerate() {
                let (x, y, z) = (moon.position[0], moon.position[1], moon.position[2]);
                xs[i].push(x);
                ys[i].push(y);
                zs[i].push(z);
            }

            update_moons(&mut moons);
        }

        let x_period = xs
            .iter()
            .map(|e| calc_period(e.to_vec()))
            .collect::<Vec<usize>>();
        let y_period = ys
            .iter()
            .map(|e| calc_period(e.to_vec()))
            .collect::<Vec<usize>>();
        let z_period = zs
            .iter()
            .map(|e| calc_period(e.to_vec()))
            .collect::<Vec<usize>>();

        println!("periods {:?} {:?} {:?}", x_period, y_period, z_period);

        assert_eq!(*x_period.iter().max().unwrap(), 18);
        assert_eq!(*y_period.iter().max().unwrap(), 28);
        assert_eq!(*z_period.iter().max().unwrap(), 44);
    }

}
