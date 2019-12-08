use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

type Planets = HashMap<String, Vec<String>>;

fn main() {
    // test
    let test_data = read_test_data();
    let mut planets = Planets::new();

    for split in test_data.split('\n') {
        add_orbit(&split, &mut planets);
    }

    let sum = get_distance_from(&planets, "COM", 0);

    println!("{}", sum);

    // data
    let data = read_data();
    let mut planets = Planets::new();

    for split in data.split('\n') {
        add_orbit(&split, &mut planets);
    }

    let sum = get_distance_from(&planets, "COM", 0);

    println!("{}", sum);

    // test 2
    let test_data = read_test_data_2();
    let mut planets = Planets::new();

    for split in test_data.split('\n') {
        add_orbit(&split, &mut planets);
    }

    let mut planets = reverse(planets);

    let dist = get_distance_between(&planets, "YOU", "SAN", 0, vec![]);
    println!("{}", dist);

    // data 2
    let test_data = read_data();
    let mut planets = Planets::new();

    for split in test_data.split('\n') {
        add_orbit(&split, &mut planets);
    }

    let mut planets = reverse(planets);

    let dist = get_distance_between(&planets, "YOU", "SAN", 0, vec![]);
    println!("{}", dist);
}

fn get_distance_from(planets: &Planets, p: &str, distance: usize) -> usize {
    if let Some(orbits) = planets.get(p) {
        let children: usize = orbits
            .iter()
            .map(|p| get_distance_from(&planets, p, distance + 1))
            .sum();

        distance + children
    } else {
        distance
    }
}

fn get_distance_between(
    planets: &Planets,
    p1: &str,
    p2: &str,
    dist: usize,
    mut travelled: Vec<String>,
) -> usize {
    travelled.push(p1.to_string());
    if p1 == p2 {
        dist - 2
    } else {
        if let Some(orbits) = planets.get(p1) {
            let dists = orbits
                .iter()
                .filter(|o| !travelled.contains(o))
                .map(|p| get_distance_between(planets, p, p2, dist + 1, travelled.clone()))
                .collect::<Vec<usize>>();
            *dists.iter().min().unwrap_or(&(u32::max_value() as usize))
        } else {
            u32::max_value() as usize
        }
    }
}

fn add_orbit(orbit: &str, planets: &mut Planets) {
    let mut orbit_split = orbit.split(')');
    let p1 = orbit_split.next().unwrap().trim().to_string();
    let p2 = orbit_split.next().unwrap().trim().to_string();
    planets.entry(p1).or_insert(vec![]).push(p2);
}

fn reverse(planets: Planets) -> Planets {
    let mut reversed = Planets::new();

    for (key, orbits) in planets.iter() {
        for p in orbits {
            reversed
                .entry(p.to_string())
                .or_insert(planets.get(p).unwrap_or(&Vec::new()).to_vec())
                .push(key.to_string());
        }
    }

    reversed
}

//

fn read_test_data() -> String {
    String::from(
        "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L",
    )
}

fn read_test_data_2() -> String {
    String::from(
        "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN",
    )
}

fn read_data() -> String {
    let mut f = File::open("data.txt").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    String::from(buffer.trim())
}
