use std::{cmp::Ordering, fs, collections::{HashMap, HashSet}};

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Could not read the input file");
    let two_inputs: Vec<&str> = data.split("\n\n").collect();

    let mut depends_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    let _= two_inputs[0]
        .lines()
        .for_each(|line| {
            let values: Vec<i32> = line.split('|').map(|val| val.parse().expect("Could not parse the input")).collect();
            depends_map.entry(values[1]).or_insert_with(HashSet::new).insert(values[0]);
        });

    let answer: i32 = two_inputs[1]
        .lines()
        .map(|line| {
            let mut values: Vec<i32> = line.split(',').map(|val| val.parse().expect("Could not parse the input")).collect();
            let mut not_allowed: HashSet<i32> = HashSet::new();

            let ret = values
                .iter()
                .all(|&x| {
                    if(not_allowed.contains(&x)) {
                        return false;
                    } else {
                        if let Some(vec) = depends_map.get(&x) {
                            vec.iter().for_each(|val| {
                                not_allowed.insert(*val);
                            });
                        }
                        return true;
                    }
                });

            if ret {0} else {
                values.sort_by(|a, b| {
                    if let Some(found_key) = depends_map.get(&a) {
                        if found_key.contains(&b) {Ordering::Greater} else {Ordering::Less}
                    } else {
                        Ordering::Less
                    }
                });

                values[values.len()/2]
            }
        })
        .sum();

    println!("{}", answer);
}

