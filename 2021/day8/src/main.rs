use std::fs;

fn main() {
    let vec = give_vec();

    println!("Part1->{}", part1(vec.clone()));
    println!("Part2->{}", part2(vec));
}

fn give_vec() -> Vec<Vec<Vec<String>>> {
    let input: String = fs::read_to_string("files/input.txt").expect("error");
    let mut vec: Vec<Vec<Vec<String>>> = Vec::new();

    for line in input.lines() {
        let mut vectors: Vec<Vec<String>> = Vec::new();
        for part in line.split(" | ") {
            let vec_string: Vec<String> = part.split_whitespace().map(|s| s.to_string()).collect();
            vectors.push(vec_string);
        }
        vec.push(vectors);
    }

    vec
}

fn part1(vec: Vec<Vec<Vec<String>>>) -> i32 {
    let mut count = 0;

    vec.iter().for_each(|f| {
        f[1].iter().for_each(|s| match s.len() {
            2 => count += 1,
            3 => count += 1,
            4 => count += 1,
            7 => count += 1,
            _ => print!(""),
        })
    });

    count
}

fn part2(vec: Vec<Vec<Vec<String>>>) -> i32 {
    let mut count = 0;

    for nums in vec.iter() {
        count += converter(nums[0].clone(), nums[1].clone());
    }

    count
}

fn similar(checker: &str, num: &String) -> bool {
    let mut count = 0;

    for c in checker.chars() {
        if num.contains(c) {
            count += 1;
        }
    }
    if (count == 2 && (num.len() == 6 || checker.len() == 2))
        || (count == 3 && num.len() == 5)
        || (count == 4)
    {
        true
    } else {
        false
    }
}

fn converter(signal: Vec<String>, nums: Vec<String>) -> i32 {
    let mut one = "";
    let mut four = "";

    for s in signal.iter() {
        match s.len() {
            2 => one = s,
            4 => four = s,
            _ => print!(""),
        }
    }

    let mut count = "".to_string();
    for s in nums.iter() {
        match s.len() {
            2 => count += "1",
            3 => count += "7",
            4 => count += "4",
            5 => {
                if similar(one, s) {
                    count += "3"
                } else if similar(four, s) {
                    count += "5"
                } else {
                    count += "2"
                }
            }
            6 => {
                if similar(four, s) {
                    count += "9"
                } else if similar(one, s) {
                    count += "0"
                } else {
                    count += "6"
                }
            }
            7 => count += "8",
            _ => print!(""),
        }
    }
    count.parse::<i32>().unwrap()
}
