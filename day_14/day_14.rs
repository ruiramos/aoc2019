// cargo-deps: num="0.2.0"

extern crate num;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

type Map = HashMap<String, (u64, Vec<(u64, String)>)>;

fn main() {
    let result = get_ore(read_test_data());
    assert_eq!(result, 31);

    let result = get_ore(read_test_data2());
    assert_eq!(result, 165);

    let result = get_ore(read_test_data3());
    assert_eq!(result, 13312);

    let result = get_ore(read_test_data4());
    assert_eq!(result, 180697);

    let result = get_ore(read_test_data5());
    assert_eq!(result, 2210736);

    let result = get_ore(read_data());
    println!("{}", result);

    let trillion: u64 = 1_000_000_000_000;

    let result = get_fuel_with_ore(trillion, read_data());
    println!("{}", result);
}

fn parse_reactions(data: String) -> Map {
    let mut map: Map = HashMap::new();

    for (i, reaction) in data.split('\n').enumerate() {
        let mut iter = reaction.split(" => ");
        let reagents: Vec<(u64, String)> = iter
            .next()
            .unwrap()
            .split(", ")
            .map(parse_ingredient)
            .collect();

        let (ammount, product) = parse_ingredient(iter.next().unwrap());

        // C => (1 [7A, 1B])
        map.insert(product, (ammount, reagents));
    }
    map
}

fn get_ore(data: String) -> u64 {
    let mut map = parse_reactions(data);
    let mut waste: HashMap<String, u64> = HashMap::new();
    produce(1, "FUEL", &mut map, &mut waste)
}

fn get_fuel_with_ore(ore: u64, data: String) -> usize {
    let mut produce_fuel = |n| {
        let mut map = parse_reactions(data.to_string());
        let mut waste: HashMap<String, u64> = HashMap::new();
        produce(n, "FUEL", &mut map, &mut waste)
    };

    let mut min: u64 = 100;
    let mut max: u64 = 100_000_000;

    loop {
        if max - min <= 1 {
            return min as usize;
        }

        let mid: u64 = (max + min) / 2;

        if produce_fuel(mid) > ore {
            max = mid;
        } else {
            min = mid;
        };
    }
}

fn produce(ammount: u64, element: &str, map: &mut Map, waste: &mut HashMap<String, u64>) -> u64 {
    if element == "ORE" {
        return ammount as u64;
    }

    let (product_ammount, ingredients) = map.get(element).unwrap().clone();
    let num_reactions = (ammount as f64 / product_ammount as f64).ceil() as u64;
    *waste.entry(element.to_string()).or_insert(0) +=
        (num_reactions * product_ammount as u64) - ammount;

    let mut sum = 0;

    for (mult, ingredient) in ingredients {
        let waste_ammount = *waste.get(&ingredient).unwrap_or(&0);
        let needed = num_reactions * mult as u64;

        if needed > waste_ammount {
            waste.insert(ingredient.to_string(), 0);
            sum += produce(needed - waste_ammount, &ingredient, map, waste);
        } else {
            waste.insert(ingredient.to_string(), waste_ammount - needed);
        }
    }

    sum
}

fn parse_ingredient(s: &str) -> (u64, String) {
    let mut iter = s.split(" ");
    (
        iter.next().unwrap().parse::<u64>().unwrap(),
        iter.next().unwrap().to_string(),
    )
}

fn read_data() -> String {
    let mut f = File::open("data.txt").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    String::from(buffer.trim())
}

fn read_test_data() -> String {
    String::from(
        "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL",
    )
}

fn read_test_data2() -> String {
    String::from(
        "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL",
    )
}

fn read_test_data3() -> String {
    String::from(
        "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
    )
}

fn read_test_data4() -> String {
    String::from(
        "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF",
    )
}

fn read_test_data5() -> String {
    String::from(
        "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX",
    )
}
