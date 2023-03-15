use std::fs;
use std::io::{self};

fn main() {
    loop{
        println!("[1, 2]");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        
        let vec = give_vec();

        match guess.trim().parse::<u16>().unwrap() {
            1 => {part1(vec); break},
            2 => {part2(vec); break},
            _ => println!("Invalid input"),
        }
    }
}

fn part1(vec: Vec<u16>){
    let mut num: u16 = 0;
    let mut counter: u16 = 0;
    
    for x in vec.iter()  {
        if x > &num {
            counter+=1;
        }
        num = *x;
    }

    println!("{}", counter)
}

fn part2(vec: Vec<u16>){
    let mut num: u16 = 0;
    let mut counter:u16 = 0;

    for x in vec.windows(3){
        let sum: u16 = x.iter().sum();
        if sum > num{
            counter += 1;
        }
        num = sum;
    }

    println!("{}", counter)
}

fn give_vec() -> Vec<u16>{
    return fs::read_to_string("files/input.txt")
        .expect("Unable to read file")
        .lines()
        .map(|s| s.trim().parse::<u16>().unwrap())
        .collect();
}