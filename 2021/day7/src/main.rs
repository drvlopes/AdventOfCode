use std::fs;

fn main() {
    let crabs = give_vec();

    println!("Part 1-> {:?}", part1(crabs.clone()));
    println!("Part 2-> {:?}", part2(crabs));
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

fn part1(mut crabs: Vec<i32>) -> i32 {
    crabs.sort();
    let mid = crabs.len() / 2;
    let mut soma = 0;

    for x in &crabs {
        let index = crabs[mid] as i32;
        if x - index != 0 {
            soma += (x - index).abs();
        }
    }

    soma
}

fn part2(crabs: Vec<i32>) -> i32 {
    let alignment = crabs.iter().sum::<i32>() as f32 / (crabs.len() as f32 - 0.5);
    let mut soma: i32 = 0;

    for x in &crabs {
        let index = alignment as i32;
        if x - index != 0 {
            soma += ((x - index).abs() * ((x - index).abs() + 1)) / 2;
        }
    }

    soma
}
