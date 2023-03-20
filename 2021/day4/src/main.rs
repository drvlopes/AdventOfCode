use std::{fs::{self}};

fn main() {
    let (a, b) = part1_2();

    println!("part1 -> {}", a);
    println!("part2 -> {}", b)

}

fn give_vec()-> (Vec<i32>, Vec<Vec<Vec<i32>>>){
    let lines: Vec<String> = fs::read_to_string("files/input.txt")
    .expect("Unable to read file")
    .lines()
    .map(|s| s.trim().to_string())
    .collect();
    let mut array: Vec<i32> = vec![]; 
    let mut matrix: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut i = 0;
        for line in lines {
            if array.len() < 1 {
                array = line.split(',').map(|s| s.parse().expect("parse error")).collect();
            }
            else {
                let row: Vec<i32> = line
                    .split_ascii_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                if row.len() > 1 {
                    if i == 0 {
                        matrix.push(vec![row]);
                    }
                    else if i < 5{
                        let j = matrix.len();
                        matrix[j - 1].push(row);
                    }
                    i+=1;
                }
                else{
                    i = 0;
                }
            }
        }
        return (array, matrix);
    
}

fn check_win(bingo_boards: Vec<Vec<Vec<i32>>>, victory_counter: Vec<i32>) -> i32{
    let mut board_counter = 0;

    for board in bingo_boards.clone(){
        if !victory_counter.contains(&board_counter){
            for num in 0..5{
                if (board[num][0] == -1) && (board[num][1] == -1) && (board[num][2] == -1) && (board[num][3] == -1) && (board[num][4] == -1){
                    return board_counter;
                }
                else if (board[0][num] == -1) && (board[1][num] == -1) && (board[2][num] == -1) && (board[3][num] == -1) && (board[4][num] == -1){
                    return board_counter;
                }
            }
        }
        board_counter += 1;
    }

    return -1;
}

fn part1_2() -> (i32, i32){
    let (bingo_nums, bingo_boards) = give_vec();
    let mut board_counter;
    let mut row_counter;
    let mut num_counter;
    let mut row_col;
    let mut bingo_boards_copy = bingo_boards.clone();
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    let mut victory_counter: Vec<i32> = vec![];

    for single_num in bingo_nums{
        board_counter = 0;
        for single_board in bingo_boards.clone(){
            row_counter = 0;
            for board_row in single_board {
                num_counter = 0;
                for row_num in board_row{
                    if row_num == single_num {
                        bingo_boards_copy[board_counter][row_counter][num_counter] = -1;
                        row_col = check_win(bingo_boards_copy.clone(), victory_counter.clone());
                        if row_col != -1{
                            if part1 == 0{
                                part1 = calc_points(bingo_boards_copy[row_col as usize].clone(), single_num);
                            }
                            if !victory_counter.contains(&(board_counter as i32)){
                                victory_counter.push(board_counter as i32);
                                if victory_counter.len() == bingo_boards.len(){
                                    part2 = calc_points(bingo_boards_copy[*victory_counter.last().unwrap() as usize].clone(), single_num);
                                }
                            }
                        }
                    }
                    num_counter += 1;
                }
                row_counter += 1;
            }
            board_counter += 1;
        }
    }

    return (part1, part2);
}

fn calc_points(board: Vec<Vec<i32>>, last_bingo_num: i32) -> i32{

    let mut soma = 0;

    for row in board{
        for num in row {
            if num != -1 {
                soma += num;
            }
        }
    }

    return soma * last_bingo_num;
}