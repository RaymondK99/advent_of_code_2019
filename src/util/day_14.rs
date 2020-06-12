use crate::util::Part;
use std::collections::HashMap;


pub fn solve(input:String, part:Part) -> String {
    let result = match part {
        Part::Part1 => part1(input.as_str()),
        Part::Part2 => part2(input.as_str())
    };

    format!("{}",result)
}

fn part1(input:&str) -> u64 {
    count_or_for_one_fuel(parse_input(input))
}

fn part2(input:&str) -> u64 {
    2
}

#[derive(Debug,Clone)]
struct Component {
    qty:u64,
    material:String,
}

#[derive(Debug,Clone)]
struct Reaction {
    output:Component,
    input:Vec<Component>,
}

fn parse_reaction(line:&str) -> Reaction {
    let parts:Vec<&str> = line.split("=>").map(|s| s.trim()).collect();
    let output_pair:Vec<&str> = parts[1].split(' ').map(|s| s.trim()).collect();
    let output : Component = Component{qty:output_pair[0].parse().unwrap(), material:output_pair[1].to_string()};

    let material:Vec<&str> = parts[0].split(",").map(|s| s.trim()).collect();
    let materials : Vec<Component> = material.iter().map(|&s| {
        let l : Vec<&str>= s.split(' ').collect();
        Component{material:String::from(l[1]), qty:l[0].parse().unwrap()} }
    ).collect();

    Reaction{output:output, input:materials}
}

fn parse_input(input:&str) -> Vec<Reaction> {
    let reactions:Vec<Reaction> = input.lines().map( |s| parse_reaction(s)).collect();
    reactions
}

fn int_div_round_up(denominator:u64, divisor:u64) -> u64 {
    if denominator % divisor > 0 {
        return (denominator / divisor) + 1;
    }
    denominator / divisor
}

fn get_required_multiple(req_qty:u64, multiple:u64) -> u64 {
    if multiple == 1 {
        return req_qty;
    } else if multiple < req_qty {
        // Ex. required is 9 but multiple of 7
        let rest = req_qty % multiple;
        if rest > 0 {
            return 1+(req_qty / multiple);
        } else {
            return req_qty / multiple;
        }
    } else {
        // Ex. Required is 5 but multiple is 9
        1
    }
}

fn count_or_for_one_fuel(reactions:Vec<Reaction>) -> u64 {
    count_ore_per_fuel(&reactions, &mut HashMap::new())
}


fn count_or_for_n_fuel(reactions:Vec<Reaction>, total_ore:u64) -> u64 {
    let mut surplus_map:HashMap<String, u64> =  HashMap::new();
    let mut ore_acc = 0;
    let mut fuel_acc:u64 = 0;
    while ore_acc < total_ore {
        ore_acc += count_ore_per_fuel(&reactions,  &mut surplus_map);
        fuel_acc += 1;
        if surplus_map.is_empty() {
            println!("=> at fuel {} and ore {}, surplus is empty.", fuel_acc, ore_acc);
            break;
        }
    }

    let fuel_mult = fuel_acc;
    let ore_mult = ore_acc;

    let mult = total_ore / ore_mult;
    ore_acc = mult * ore_mult;
    fuel_acc = mult * fuel_mult;

    while ore_acc < total_ore {
        ore_acc += count_ore_per_fuel(&reactions,  &mut surplus_map);
        if ore_acc <= total_ore {
            fuel_acc += 1;
        }
    }

    fuel_acc
}

fn count_ore_per_fuel(reactions:&Vec<Reaction>, surplus_map:&mut HashMap<String, u64>) -> u64 {
    //  Build map of materials
    let mut material_map:HashMap<String, u64> = HashMap::new();
    let mut reaction_map:HashMap<String, &Reaction> = HashMap::new();
    for reaction in reactions.iter() {
        reaction_map.insert(reaction.output.material.clone(), reaction);
    }

    // Get fuel
    let mut stack:Vec<Component> = vec![];
    let mut ore_stack:std::collections::HashSet<String> = std::collections::HashSet::new();

    stack.push( Component{material:String::from("FUEL"), qty:1});

    while !stack.is_empty() {
        // pop material
        let material = stack.pop().unwrap();

        // Get reaction
        let &reaction = reaction_map.get(&material.material).unwrap();

        let mut bank_qty = surplus_map.remove(&material.material).or(Some(0)).unwrap();
        let mut lowest_mult = 0;

        if bank_qty > 0 {
            //println!(" => Found {} rest qty for {}", bank_qty, material.material);
        }

        // Determine how may times we need to run this reaction
        while bank_qty < material.qty {
            bank_qty += reaction.output.qty;
            lowest_mult += 1;
        }

        // Rest qty?
        let rest_qty = bank_qty - material.qty;

        // Update surplus map
        if rest_qty > 0 {
            surplus_map.insert(material.material.clone(), rest_qty);
            //println!(" => Added {} rest qty for {}", rest_qty, material.material);
        }

        // Could make it on rest material
        if lowest_mult == 0 {
            continue;
        }

        // Push new materials required
        for comp in reaction.input.iter() {
            let total_qty = lowest_mult * comp.qty;

            if !comp.material.as_str().eq("ORE") {
                //println!("Reaction {:?}", reaction);
                //println!("To produce {} {} I need {} {}", material.qty, material.material, total_qty ,comp.material);
                stack.push(Component { qty: total_qty, material: comp.material.clone() });

                // Add to requirement map
                if material_map.contains_key(&comp.material) {
                    let cnt = material_map.get_mut(&comp.material).unwrap();
                    *cnt += total_qty;
                    //println!("   Updated need for {} to {}", comp.material, cnt);
                } else {
                    material_map.insert(comp.material.clone(), total_qty);
                    //println!("   Need for {} to {}", comp.material, total_qty);
                }
            } else {
                // Ore transition
                ore_stack.insert(material.material.clone());
            }
        }
    }

    // Transition from Ore to Materials
    let mut ore_qty = 0;
    for material_str in ore_stack {
        let material_qty = material_map.get(&material_str).unwrap();

        //println!("I need to produce {} of {}", material_qty, material_str);
        let &reaction = reaction_map.get(&material_str).unwrap();
        //println!("Reaction: {:?}", reaction);

        let mult = get_required_multiple(*material_qty, reaction.output.qty);
        ore_qty += mult * reaction.input.first().unwrap().qty;

        //println!("Required ore: {}", mult * reaction.input.first().unwrap().qty);
    }

    ore_qty
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {
        println!("Test");
        let input = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
        let input = "3 A, 4 B => 1 AB";
        let reaction = parse_reaction(input);
        println!("{:?}",reaction);

    }

    #[test]
    fn test2() {
        println!("Test");
        let input = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
        let reactions = parse_input(input);
        println!("{:?}",reactions);
    }

    #[test]
    fn test3() {
        println!("Test");
        let input = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
        let res = count_or_for_one_fuel(parse_input(input));
        println!("{:?}",res);
        assert_eq!(165, res);
    }


    #[test]
    fn test4() {
        println!("Test");
        let input = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
        let res = count_or_for_one_fuel(parse_input(input));
        println!("{:?}",res);
        assert_eq!(31, res);
    }

    #[test]
    fn test5() {
        println!("Test");
        let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let res = count_or_for_one_fuel(parse_input(input));
        println!("{:?}",res);
        assert_eq!(13312, res);
    }

    #[test]
    fn test_part2_test1() {
        println!("Test");
        let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let total_ore = 1000_000_000_000;
        let produced_fuel = count_or_for_n_fuel(parse_input(input), total_ore);
        println!("{} ORE gives {} fuel", total_ore, produced_fuel);

    }

    #[test]
    fn test6() {
        println!("Test");
        let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
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
176 ORE => 6 VJHF";
        let res = count_or_for_one_fuel(parse_input(input));
        println!("{:?}",res);
        assert_eq!(180697, res);
    }

    #[test]
    fn test_part2_test2() {
        println!("Test");
        let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
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
176 ORE => 6 VJHF";
        let total_ore = 1000_000_000_000;
        let produced_fuel = count_or_for_n_fuel(parse_input(input), total_ore);
        println!("{} ORE gives {} fuel", total_ore, produced_fuel);

    }

    #[test]
    fn test7() {
        println!("Test");
        let input = "171 ORE => 8 CNZTR
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
5 BHXH, 4 VRPVC => 5 LTCX";
        let res = count_or_for_one_fuel(parse_input(input));
        println!("{:?}",res);
        assert_eq!(2210736, res);
    }

    //#[test]
    fn part2_test3() {
        println!("Test");
        let input = "171 ORE => 8 CNZTR
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
5 BHXH, 4 VRPVC => 5 LTCX";
        let total_ore = 1000_000_000_000;
        let produced_fuel = count_or_for_n_fuel(parse_input(input), total_ore);
        println!("{} ORE gives {} fuel", total_ore, produced_fuel);

    }


    #[test]
    fn test_part1() {
        println!("Test");
        let input = "12 JSMPL, 1 RFSHT => 8 NLTCF
6 LTSZQ, 22 KLSMX, 12 CWLGT => 2 MZXFC
4 WMVD, 3 PLBT, 1 ZKDMR => 5 CWLGT
5 SDTGC => 2 LSFKV
189 ORE => 3 TNTDN
20 CZKW => 4 BGNFD
5 XFMH => 7 SFRQ
7 NLTCF => 1 KLSMX
1 NLTCF => 4 HTDFH
2 RFPT, 5 JFXPH => 5 KRCQ
178 ORE => 7 XGLBX
1 NHQH => 3 NDMT
4 BNVTZ, 13 KXFJ, 14 QRBK, 56 SJSLP, 18 SPFP, 9 WMVD, 12 JFXPH, 1 MHXF => 1 FUEL
1 XQRX, 2 DPRVM, 1 HTDFH, 24 NLTCF, 8 SPBXP, 20 TSRNS, 2 VJDBK, 1 PXKL => 7 SPFP
6 WMVD => 3 SPBXP
1 XGLBX => 8 QXLMV
1 PLBT => 5 ZKDMR
25 VJDBK, 5 MZXFC, 3 BDGCJ => 9 BNVTZ
2 TNTDN, 1 SZNCS => 2 LMXBH
3 TNTDN => 6 RVRD
4 RFPT => 6 VHMQ
7 QXLMV, 1 LMXBH, 4 CSZP => 8 XFMH
5 SZNCS => 5 JSMPL
5 MHXF, 5 LTSZQ => 4 RFPT
5 XQMBJ, 1 BGNFD, 5 TQPGR => 3 NHQH
10 CHWS => 2 BDGCJ
19 DPRVM, 13 NHQH, 7 CZKW => 6 FWMXM
1 KLSMX, 1 PLBT, 5 XFMH => 3 SDTGC
20 LMXBH => 9 RFSHT
3 XGLBX => 1 TNPVZ
3 FBWF => 7 WMVD
1 QXLMV, 1 LMXBH => 3 ZMNV
5 JSMPL, 12 SFRQ => 8 CZKW
2 TNPVZ => 9 MHXF
2 MNVX, 1 RBMLP, 6 LSFKV => 9 VJDBK
26 SZNCS, 1 XGLBX => 6 CSZP
6 FBWF, 2 SPBXP, 4 BDGCJ => 2 TQPGR
5 LSFKV, 5 DPRVM => 9 QNFC
33 BDGCJ, 3 CWLGT => 4 XQRX
2 TQPGR, 22 LSFKV, 2 RFPT, 1 BDGCJ, 1 ZKDMR, 7 TSRNS, 6 DPRVM, 11 KRCQ => 2 QRBK
13 XQRX, 3 FWMXM, 2 CWLGT, 1 XQMBJ, 3 BGNFD, 6 HTDFH, 10 TSRNS => 5 KXFJ
1 ZKDMR => 9 CHWS
14 MNVX, 5 XFMH => 7 LTSZQ
2 NDMT, 2 QNFC, 11 ZMNV => 6 PXKL
7 SFRQ => 5 MNVX
2 WMPKD, 1 QXLMV => 9 SJSLP
14 JFXPH => 3 XQMBJ
14 SFRQ => 7 FBWF
1 WMPKD, 30 GBQGR, 4 SPBXP => 9 DPRVM
129 ORE => 4 SZNCS
5 JSMPL => 8 JFXPH
9 JFXPH, 2 VHMQ => 5 RBMLP
6 JSMPL => 7 GBQGR
25 SFRQ, 19 HRMT => 5 WMPKD
3 ZMNV => 9 PLBT
7 ZMNV, 9 RVRD, 8 SFRQ => 7 HRMT
8 RBMLP => 6 TSRNS";
        let res = count_or_for_one_fuel(parse_input(input));
        println!("{:?}",res);
        assert_eq!(319014, res);
    }
}