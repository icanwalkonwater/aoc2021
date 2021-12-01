#![feature(array_windows)]

use aoc2021::answer;

const INPUT: &str = include_str!("../../inputs/day1.txt");

fn part1(measurements: &[i32]) -> u32 {
    let mut depth_increase_count = 0;
    for [a, b] in measurements.array_windows() {
        if b > a {
            depth_increase_count += 1;
        }
    }

    depth_increase_count
}

fn part2(measurements: &[i32]) -> u32 {
    let mut depth_increase_count_sliding = 0;
    for [a, b, c, d] in measurements.array_windows() {
        let sum_a = a + b + c;
        let sum_b = b + c + d;
        if sum_b > sum_a {
            depth_increase_count_sliding += 1;
        }
    }

    depth_increase_count_sliding
}

fn main() {
    let measurements = INPUT
        .trim()
        .split("\n")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    answer(1, &part1(&measurements), &part2(&measurements));
}
