// rule:
// 1. 縦9列のどの列にも1~9の数字がひとつずつ入る
// 2. 横9列のどの列にも1~9の数字がひとつずつ入る
// 3. 正方形内で区切られた3*3のブロック(全部で9つ)のどのブロックにも1~9の数字がひとつずつ入る

use std::io::{stdin, BufRead, Error};

fn main() {
    let board = vec![
        vec![0, 2, 0, 0, 7, 5, 0, 1, 0],
        vec![1, 0, 0, 0, 0, 4, 5, 0, 8],
        vec![0, 5, 6, 8, 0, 0, 2, 0, 0],
        vec![8, 1, 0, 0, 0, 0, 7, 0, 0],
        vec![9, 0, 0, 0, 0, 0, 0, 0, 3],
        vec![0, 0, 2, 0, 0, 0, 0, 4, 5],
        vec![0, 0, 9, 0, 0, 1, 4, 3, 0],
        vec![3, 0, 1, 7, 0, 0, 0, 0, 9],
        vec![0, 7, 0, 3, 9, 0, 0, 2, 0],
    ];

    // solver
    let board2 = read_board();

    for row in board {
        for num in row {
            if num == 0 {
                print!("  ");
            } else {
                print!("{} ", num);
            }
        }
        println!();
    }

    println!("{:?}", board2);
}

fn read_board() -> Vec<Vec<i32>> {
    let stdin = stdin();
    let reader = stdin.lock();

    let mut board = Vec::new();
    let mut num = 0;

    for line in reader.lines() {
        num += 1;
        let line = match line {
            Ok(string) => {
                if string == "" {
                    break;
                }
                string
            }
            Err(_) => { panic!("can't read stdin") }
        };
        let row = line.split(",")
            .collect::<Vec<_>>()
            .into_iter()
            .map(|str| str.parse().unwrap())
            .collect::<Vec<i32>>();
        board.push(row);
        if num == 9 {
            break;
        }
    }

    board
}
