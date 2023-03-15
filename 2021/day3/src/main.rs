use std::fs;

fn main() {
    let vec = give_vec();
    part1(&vec);
    part2(&vec);
}

fn give_vec() -> Vec<String>{
    return fs::read_to_string("files/input.txt")
    .expect("Unable to read file")
    .lines()
    .map(|s| s.trim().to_string())
    .collect();
}

fn part1(vec: &Vec<String>){
    let (gr, er) = give_usage(vec.to_vec());

    println!("Part1: {:?}", isize::from_str_radix(&gr, 2).unwrap() * isize::from_str_radix(&er, 2).unwrap());
}

fn part2(vec: &Vec<String>){
    let mut new_vec = vec.clone();
    let mut i = 0;

    while new_vec.len() > 1 {
        new_vec = idk(new_vec, i, true);
        i += 1;
    }

    let a = isize::from_str_radix(&new_vec[0], 2).unwrap();

    new_vec = vec.clone();
    i = 0;
    while new_vec.len() > 1 {
        new_vec = idk(new_vec, i, false);
        i += 1;
    }

    println!("Part2: {:?}", a * isize::from_str_radix(&new_vec[0], 2).unwrap())

}

fn idk(vec: Vec<String>, i: i32, first: bool) -> Vec<String>{
    let mut bit: i32 = 0;
    let mut new_vec: Vec<String> = Vec::new();

    vec.iter().for_each(|f| {
        if f.chars().nth(i.try_into().unwrap()).unwrap() == '1' {
            bit += 1;
        }
        else {
            bit -= 1;
        }
    });

    let bit = if (bit >= 0 && first) || (bit < 0 && !first){ '1' } else { '0' };

    vec.iter().for_each(|f| {
        if f.chars().nth(i.try_into().unwrap()).unwrap() == bit {
            new_vec.push(f.to_string())
        }
    });

    return new_vec;
}

fn give_usage(vec: Vec<String>) -> (String, String){
    let mut bit = [0; 12];
    let mut gr = "".to_string();
    let mut er = "".to_string();

    for x in vec {
        for (i, c) in x.chars().enumerate() {
            if c == '1'{
                bit[i] += 1;
            }
            else{
                bit[i] -= 1;
            }
        }
    }

    for i in 0..12{
        if bit[i] > 0 {
            gr.push_str(&"1".to_string());
            er.push_str(&"0".to_string())
        }
        else {
            gr.push_str(&"0".to_string());
            er.push_str(&"1".to_string())
        }
    }
    return (gr, er);

}
