use std::fs;

fn main() {
    let vec = give_vec();

    part2(vec.clone(), part1(vec));
}

fn give_vec() -> Vec<Vec<u16>> {
    let input: String = fs::read_to_string("files/input.txt").expect("error");
    let mut vec: Vec<Vec<u16>> = Vec::new();

    for line in input.lines() {
        let mut vectors: Vec<u16> = Vec::new();
        for n in line.chars() {
            vectors.push(n as u16 - '0' as u16)
        }
        vec.push(vectors);
    }

    vec
}

fn part1(vec: Vec<Vec<u16>>) -> Vec<(usize, usize)> {
    let len_y = vec.len() - 1;
    let len_x = vec[0].len() - 1;
    let mut lowest: Vec<(usize, usize)> = vec![];
    let mut part1: Vec<u16> = vec![];
    for (i, y) in vec.iter().enumerate() {
        for (j, x) in y.iter().enumerate() {
            let (mut top, mut bottom, mut left, mut right) = (true, true, true, true);
            if i > 0 {
                top = x < &vec[i - 1][j]
            }
            if j > 0 {
                left = x < &vec[i][j - 1]
            }
            if i < len_y {
                bottom = x < &vec[i + 1][j]
            }
            if j < len_x {
                right = x < &vec[i][j + 1]
            }
            if top && bottom && left && right {
                part1.push(*x);
                lowest.push((i, j))
            }
        }
    }
    let sum: u16 = part1.iter().map(|f| f + 1).sum();
    println!("part1-> {}", sum);
    lowest
}

fn part2(mut vec: Vec<Vec<u16>>, pos: Vec<(usize, usize)>) {
    let mut map: Vec<(usize, usize)> = vec![];
    let mut count: Vec<u16> = vec![];
    let len_y = vec.len() - 1;
    let len_x = vec[0].len() - 1;

    for (i, (y, x)) in pos.iter().enumerate() {
        count.push(0);
        map.push((*y, *x));
        loop {
            let (mut top, mut bottom, mut left, mut right) = (false, false, false, false);
            let len = map.len();

            if map.len() == 0 {
                break;
            }
            let (y, x) = map.get(len - 1).expect("error").clone();

            if y > 0 {
                top = vec[y - 1][x] != 9;
                if top {
                    map.push((y - 1, x));
                }
            }
            if x > 0 {
                left = vec[y][x - 1] != 9;
                if left {
                    map.push((y, x - 1));
                }
            }
            if y < len_y {
                bottom = vec[y + 1][x] != 9;
                if bottom {
                    map.push((y + 1, x));
                }
            }
            if x < len_x {
                right = vec[y][x + 1] != 9;
                if right {
                    map.push((y, x + 1));
                }
            }
            if vec[y][x] != 9 {
                vec[y][x] = 9;
                count[i] += 1;
            }

            if !top && !bottom && !left && !right {
                map.remove(len - 1);
            }
        }
    }
    count.sort_by(|a, b| b.cmp(a));
    let mut conta: u32 = 1;
    for i in 0..3 {
        conta *= count[i] as u32;
    }
    println!("part2-> {:?}", conta)
}
