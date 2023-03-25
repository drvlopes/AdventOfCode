use std::fs;

fn main() {
    let crabs = give_vec();
    let mut lowest = 999999999;

    for (i, _) in crabs.iter().enumerate() {
        let mut soma = 0;
        crabs.iter().for_each(|x| {
            soma += ((x - i as i32).abs() * ((x - i as i32).abs() + 1)) / 2;
        });
        if soma < lowest {
            lowest = soma;
        }
    }

    println!("{:?}", lowest);
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
