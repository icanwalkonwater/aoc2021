#![feature(drain_filter)]

use aoc2021::answer;
use std::str::FromStr;

const INPUT: &str = include_str!("../../inputs/day4.txt");

#[derive(Debug, Clone)]
struct BingoBoard {
    numbers: [i32; 5 * 5],
    marked: [bool; 5 * 5],
}

impl FromStr for BingoBoard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .trim()
            .split("\n")
            .map(|line| {
                let cells = line
                    .split_whitespace()
                    .map(|cell| cell.trim().parse().unwrap())
                    .collect::<Vec<_>>();
                assert_eq!(cells.len(), 5);
                cells
            })
            .enumerate()
            .fold([0; 5 * 5], |mut acc, (i, line)| {
                for y in 0..5 {
                    acc[i * 5 + y] = line[y];
                }
                acc
            });

        Ok(BingoBoard {
            numbers,
            marked: Default::default(),
        })
    }
}

impl BingoBoard {
    fn mark(&mut self, value: i32) {
        self.numbers
            .iter()
            .enumerate()
            .filter(|(_, &val)| val == value)
            .for_each(|(i, _)| self.marked[i] = true);
    }

    fn check(&self) -> bool {
        // Check for complete lines
        let line_complete = (0..5)
            .into_iter()
            .map(|line| self.marked.iter().skip(line * 5).take(5).all(|m| *m))
            .any(|m| m);

        // Don't need to check columns
        if line_complete {
            return true;
        }

        let col_complete = (0..5)
            .into_iter()
            .map(|col| self.marked.iter().skip(col).step_by(5).all(|m| *m))
            .any(|m| m);

        col_complete
    }

    fn score(&self) -> i32 {
        self.numbers
            .iter()
            .zip(self.marked.iter())
            .filter(|(_, &marked)| !marked)
            .map(|(&v, _)| v)
            .sum()
    }
}

fn part1(seq: &[i32], boards: &[BingoBoard]) -> i32 {
    let mut boards = boards.to_vec();

    for &num in seq {
        for board in &mut boards {
            board.mark(num);
        }

        for board in &boards {
            if board.check() {
                return board.score() * num;
            }
        }
    }

    panic!("Nobody won !");
}

fn part2(seq: &[i32], boards: &[BingoBoard]) -> i32 {
    let mut boards = boards.to_vec();

    for &num in seq {
        for board in &mut boards {
            board.mark(num);
        }

        // It is the last winner
        if boards.len() == 1 && boards[0].check() {
            return boards[0].score() * num;
        }

        // Remove the finished boards
        boards.drain_filter(|b| b.check());
        assert!(!boards.is_empty());
    }

    panic!("Wtf happened");
}

fn main() {
    let mut input_iter = INPUT.trim().split("\n");
    let seq = input_iter
        .next()
        .unwrap()
        .split(",")
        .map(|num| num.trim().parse().unwrap())
        .collect::<Vec<i32>>();

    let mut boards = Vec::new();

    // The next in the condition should be an empty line anyway
    while let Some(_) = input_iter.next() {
        let mut raw_board = String::new();
        // 5 lines of bingo board
        raw_board.push_str(input_iter.next().unwrap());
        raw_board.push('\n');
        raw_board.push_str(input_iter.next().unwrap());
        raw_board.push('\n');
        raw_board.push_str(input_iter.next().unwrap());
        raw_board.push('\n');
        raw_board.push_str(input_iter.next().unwrap());
        raw_board.push('\n');
        raw_board.push_str(input_iter.next().unwrap());

        boards.push(raw_board.parse::<BingoBoard>().unwrap());
    }

    answer(4, &part1(&seq, &boards), &part2(&seq, &boards));
}
