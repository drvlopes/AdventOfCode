use std::fs;

fn main() {
    let mut fishes_count: Vec<i64> = give_vec();

    for i in 1..=256 {
        let temp_num = fishes_count[0];
        for x in 0..=7 {
            fishes_count.swap(x, x + 1);
        }
        fishes_count[6] += temp_num;
        if i == 80 {
            let total: i64 = fishes_count.iter().sum();
            println!("Part1-> {}", total)
        }
    }

    let total: i64 = fishes_count.iter().sum();
    println!("Part2-> {}", total)
}

fn give_vec() -> Vec<i64> {
    let contents = fs::read_to_string("files/input.txt").expect("Failed to read input file");
    let fishes: Vec<usize> = contents
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut fishes_count: Vec<i64> = vec![0; 9];

    for fish in fishes.iter() {
        fishes_count[*fish] += 1;
    }

    return fishes_count;
}
