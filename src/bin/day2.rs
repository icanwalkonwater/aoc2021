use aoc2021::answer;

const INPUT: &str = include_str!("../../inputs/day2.txt");

enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn parse() -> Vec<Instruction> {
    INPUT
        .trim()
        .split("\n")
        .map(|raw| {
            if raw.starts_with("forward") {
                Instruction::Forward(raw[8..].parse().unwrap())
            } else if raw.starts_with("down") {
                Instruction::Down(raw[5..].parse().unwrap())
            } else if raw.starts_with("up") {
                Instruction::Up(raw[3..].parse().unwrap())
            } else {
                panic!("Aled");
            }
        })
        .collect()
}

fn part1(instructions: &[Instruction]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Forward(n) => horizontal += n,
            Instruction::Down(n) => depth += n,
            Instruction::Up(n) => depth -= n,
        }
    }

    horizontal * depth
}

fn part2(instructions: &[Instruction]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Forward(n) => {
                horizontal += n;
                depth += aim * n;
            }
            Instruction::Down(n) => aim += n,
            Instruction::Up(n) => aim -= n,
        }
    }

    horizontal * depth
}

fn main() {
    let instructions = parse();

    answer(2, &part1(&instructions), &part2(&instructions));
}
