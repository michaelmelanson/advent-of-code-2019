use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq)]
pub struct Reaction {
    precursors: Vec<(String, usize)>,
    product: (String, usize)
}

#[aoc_generator(day14)]
pub fn parse_reactions(input: &str) -> Vec<Reaction> {
    // let input = "9 ORE => 2 A
    // 8 ORE => 3 B
    // 7 ORE => 5 C
    // 3 A, 4 B => 1 AB
    // 5 B, 7 C => 1 BC
    // 4 C, 1 A => 1 CA
    // 2 AB, 3 BC, 4 CA => 1 FUEL";

    // let input = "157 ORE => 5 NZVS
    // 165 ORE => 6 DCFZ
    // 44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
    // 12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
    // 179 ORE => 7 PSHF
    // 177 ORE => 5 HKGWZ
    // 7 DCFZ, 7 PSHF => 2 XJWVT
    // 165 ORE => 2 GPVTF
    // 3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

    // let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
    // 17 NVRVD, 3 JNWZP => 8 VPVL
    // 53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
    // 22 VJHF, 37 MNCFX => 5 FWMGM
    // 139 ORE => 4 NVRVD
    // 144 ORE => 7 JNWZP
    // 5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
    // 5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
    // 145 ORE => 6 MNCFX
    // 1 NVRVD => 8 CXFTF
    // 1 VJHF, 6 MNCFX => 4 RFSQX
    // 176 ORE => 6 VJHF";

    // let input = "171 ORE => 8 CNZTR
    // 7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
    // 114 ORE => 4 BHXH
    // 14 VRPVC => 6 BMBT
    // 6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
    // 6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
    // 15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
    // 13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
    // 5 BMBT => 4 WPTQ
    // 189 ORE => 9 KTJDG
    // 1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
    // 12 VRPVC, 27 CNZTR => 2 XDBXC
    // 15 KTJDG, 12 BHXH => 5 XCVML
    // 3 BHXH, 2 VRPVC => 7 MZWV
    // 121 ORE => 7 VRPVC
    // 7 XCVML => 6 RJRHP
    // 5 BHXH, 4 VRPVC => 5 LTCX";

    let mut reactions = Vec::new();

    for line in input.lines() {
        let parts = line.split("=>").collect::<Vec<_>>();
        let precursor_part = parts[0];
        let product_part = parts[1];

        let mut precursors = Vec::new();
        for precursor_str in precursor_part.split(",") {
            let precursor_str = precursor_str.trim();
            let parts = precursor_str.split(" ").collect::<Vec<_>>();

            let count: isize = parts[0].parse().unwrap();
            let chemical: String = parts[1].to_string();
            let precursor = (chemical, count as usize);

            precursors.push(precursor);
        }

        let parts = product_part.trim().split(" ").collect::<Vec<_>>();
        let count: usize = parts[0].trim().parse().unwrap();
        let chemical: String = parts[1].trim().to_string();
        let product = (chemical, count);

        reactions.push(Reaction {
            precursors, product
        });
    }

    reactions
}

fn sort_reactions(reactions: &Vec<Reaction>) -> Vec<Reaction> {
    let mut ordered_reactions: Vec<Reaction> = Vec::new();
    let mut known_products = HashSet::new();
    known_products.insert("ORE".to_string());

    while ordered_reactions.len() < reactions.len() {
        'reaction: for reaction in reactions.iter() {
            if ordered_reactions.contains(&reaction) { 
                continue; 
            }

            for precursor in reaction.precursors.iter() {
                if !known_products.contains(&precursor.0) { 
                    continue 'reaction; 
                }
            }

            known_products.insert(reaction.product.0.clone());
            ordered_reactions.push(reaction.clone());
        }
    }

    ordered_reactions
}

#[aoc(day14, part1)]
pub fn ore_per_one_fuel(reactions: &Vec<Reaction>) -> usize {
    let ordered_reactions = sort_reactions(reactions);

    ore_per_fuel(1, &ordered_reactions)
}

#[test]
pub fn test_part1_samples() {
    let reactions = parse_reactions("9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL");

    assert_eq!(ore_per_one_fuel(&reactions), 165);

    let reactions = parse_reactions("157 ORE => 5 NZVS
        165 ORE => 6 DCFZ
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
        179 ORE => 7 PSHF
        177 ORE => 5 HKGWZ
        7 DCFZ, 7 PSHF => 2 XJWVT
        165 ORE => 2 GPVTF
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT");

    assert_eq!(ore_per_one_fuel(&reactions), 13312);

    let reactions = parse_reactions("2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
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
        176 ORE => 6 VJHF");

    assert_eq!(ore_per_one_fuel(&reactions), 180697);
    
    let reactions = parse_reactions("171 ORE => 8 CNZTR
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
        5 BHXH, 4 VRPVC => 5 LTCX");

    assert_eq!(ore_per_one_fuel(&reactions), 2210736);
    
}

fn ore_per_fuel(fuel: usize, ordered_reactions: &Vec<Reaction>) -> usize {
    let mut chemicals = HashMap::new();
    chemicals.insert("FUEL".to_string(), fuel);

    for reaction in ordered_reactions.iter().rev() {
        let multiplier = div_ceil(*chemicals.get(&reaction.product.0).unwrap_or(&0), reaction.product.1);
        chemicals.remove(&reaction.product.0);

        for precursor in reaction.precursors.iter() {
            *chemicals.entry(precursor.0.clone()).or_default() += multiplier * precursor.1;
        }
    }

    *chemicals.get(&"ORE".to_string()).unwrap()
}

#[aoc(day14, part2)]
pub fn fuel_per_trillion_ore(reactions: &Vec<Reaction>) -> usize {
    let ordered_reactions = sort_reactions(reactions);

    let mut low = 100000;
    let mut high = 100000000;

    loop {
        if high - low <= 1 { return low; }
        let mid = (low + high) / 2;
        let fuel = ore_per_fuel(mid, &ordered_reactions);
        println!("fuel({}) = {}", mid, fuel);
        if fuel < 1000000000000 {
            low = mid;
        } else if fuel > 1000000000000 {
            high = mid;
        }
    }
}

#[test]
pub fn test_part2_samples() {
    let reactions = parse_reactions("157 ORE => 5 NZVS
        165 ORE => 6 DCFZ
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
        179 ORE => 7 PSHF
        177 ORE => 5 HKGWZ
        7 DCFZ, 7 PSHF => 2 XJWVT
        165 ORE => 2 GPVTF
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT");
    assert_eq!(fuel_per_trillion_ore(&reactions), 82892753);

    let reactions = parse_reactions("2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
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
        176 ORE => 6 VJHF");

    assert_eq!(fuel_per_trillion_ore(&reactions), 5586022);

    let reactions = parse_reactions("171 ORE => 8 CNZTR
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
        5 BHXH, 4 VRPVC => 5 LTCX");

    assert_eq!(fuel_per_trillion_ore(&reactions), 460664);
}

fn div_ceil(x: usize, y: usize) -> usize {
    if x % y == 0 {
        x / y
    } else {
        (x / y) + 1
    }
}