use std::{collections::HashSet, fs, io};

#[derive(Debug, PartialEq)]
struct Line {
    point1: Coord,
    point2: Coord,
}
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Coord {
    pointx: i32,
    pointy: i32,
}

fn main() {
    let vec = give_vec();
    let mut vents: HashSet<Coord> = HashSet::new();
    let mut overlapping = HashSet::new();
    let part2;

    loop {
        println!("[1, 2]");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().parse::<u16>().unwrap() {
            1 => {
                part2 = false;
                break;
            }
            2 => {
                part2 = true;
                break;
            }
            _ => println!("Invalid input"),
        }
    }

    for line in vec.iter() {
        let temp = give_line(line);
        for point in temp {
            if part2 {
                if !vents.insert(point) {
                    overlapping.insert(point);
                }
            } else {
                if !diagonal(line) {
                    if !vents.insert(point) {
                        overlapping.insert(point);
                    }
                }
            }
        }
    }

    println!("Result -> {:?}", overlapping.len())
}

fn diagonal(line: &Line) -> bool {
    if (line.point1.pointx == line.point2.pointx) || (line.point1.pointy == line.point2.pointy) {
        return false;
    }
    return true;
}

fn give_vec() -> Vec<Line> {
    let mut vec: Vec<Line> = vec![];
    let mut point: Coord = Coord {
        pointx: -1,
        pointy: -1,
    };

    let lines: Vec<String> = fs::read_to_string("files/input.txt")
        .expect("Unable to read file")
        .lines()
        .map(|s| s.trim().to_string())
        .collect();

    for line in lines {
        let row: Vec<String> = line.split("->").map(|f| f.trim().to_string()).collect();

        for x in row {
            let cord: Vec<i32> = x.split(',').map(|s| s.parse().unwrap()).collect();

            if point.pointx == -1 {
                point = Coord {
                    pointx: *cord.first().unwrap(),
                    pointy: *cord.last().unwrap(),
                }
            } else {
                vec.push(Line {
                    point1: point,
                    point2: Coord {
                        pointx: *cord.first().unwrap(),
                        pointy: *cord.last().unwrap(),
                    },
                });
                point = Coord {
                    pointx: -1,
                    pointy: -1,
                };
            }
        }
    }

    return vec;
}

fn give_line(line: &Line) -> HashSet<Coord> {
    let mut coord = HashSet::new();

    let (firstx, secondx) = (line.point1.pointx, line.point2.pointx);
    let (firsty, secondy) = (line.point1.pointy, line.point2.pointy);

    let step_x = if firstx <= secondx { 1 } else { -1 };
    let step_y = if firsty <= secondy { 1 } else { -1 };

    let (mut last_x, mut last_y) = (firstx, firsty);

    let mut x = firstx;
    while if step_x == 1 {
        x <= secondx
    } else {
        x >= secondx
    } {
        let mut y = firsty;
        while if step_y == 1 {
            y <= secondy
        } else {
            y >= secondy
        } {
            if diagonal(line) {
                if x == last_x && y == last_y {
                    coord.insert(Coord {
                        pointx: x,
                        pointy: y,
                    });
                    last_x = x + step_x;
                    last_y = y + step_y;
                }
            } else {
                coord.insert(Coord {
                    pointx: x,
                    pointy: y,
                });
            }
            y += step_y;
        }
        x += step_x;
    }

    coord
}
