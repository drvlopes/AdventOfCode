use std::{io, fs, ops::{Index}};

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

fn give_vec() -> Vec<String>{
    return fs::read_to_string("files/input.txt")
    .expect("Unable to read file")
    .lines()
    .map(|s| s.trim().to_string())
    .collect();
}

fn part1(vec: Vec<String>){
    let mut hp: i32 = 0;
    let mut d: i32 = 0;
    let mut num:i32;
    
    for x in vec {
        let i: Vec<&str> = x.split(" ").collect();
        num = i.index(1).parse::<i32>().unwrap();

        if x.contains("up"){
            d-= num;
        }
        else if x.contains("down") {
            d += num;
        }
        else {
            hp += num;
        }
    }

    println!("{}", d * hp)

}

fn part2(vec: Vec<String>){
    let mut hp: i32 = 0;
    let mut d: i32 = 0;
    let mut aim: i32 = 0;
    let mut num: i32;
    
    for x in vec {
        let i: Vec<&str> = x.split(" ").collect();
        num = i.index(1).parse::<i32>().unwrap();

        if x.contains("up"){
            aim -= num;
        }
        else if x.contains("down") {
            aim += num;
        }
        else {
            hp += num;
            d += num * aim;
        }
    }

    println!("{}", d * hp)

}