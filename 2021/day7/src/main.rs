use std::fs;

fn main() {
    let crabs = give_vec();

    let lowest = std::i32::MAX;

    println!("Part 1-> {:?}", part1(crabs.clone(), lowest));
    println!("Part 2-> {:?}", part2(crabs, lowest));
}

fn give_vec() -> Vec<i32> {
    let contents = fs::read_to_string("files/input.txt").expect("Failed to read input file");
    let crabs: Vec<i32> = contents
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    return crabs;
}

fn part1(crabs: Vec<i32>, mut lowest: i32) -> i32 {
    for i in 0..crabs.len() {
        let mut soma = 0;
        for x in &crabs {
            let index = i as i32;
            if x - index != 0 {
                soma += (x - index).abs();
            }
        }
        if soma < lowest {
            lowest = soma;
        }
    }
    lowest
}

fn part2(crabs: Vec<i32>, mut lowest: i32) -> i32 {
    for i in 0..crabs.len() {
        let mut soma = 0;
        for x in &crabs {
            let index = i as i32;
            if x - index != 0 {
                soma += ((x - index).abs() * ((x - index).abs() + 1)) / 2;
            }
        }
        if soma < lowest {
            lowest = soma;
        }
    }
    lowest
}
