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
            println!("Part1-> {}", fishes_count.iter().sum::<i64>())
        }
    }

    println!("Part2-> {}", fishes_count.iter().sum::<i64>())
}

fn give_vec() -> Vec<i64> {
    let contents = fs::read_to_string("files/input.txt").expect("Failed to read input file");
    let fishes: Vec<usize> = contents
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut fishes_count: Vec<i64> = vec![0; 9];

    fishes.iter().for_each(|n| fishes_count[*n as usize] += 1);

    return fishes_count;
}
