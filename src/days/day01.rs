use super::PuzzleParts;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug)]
struct DialState {
    pub zero_count: i32,
    pub position: i32,
}

impl Default for DialState {
    fn default() -> Self {
        DialState { zero_count: 0, position: 50 }
    }
}

fn parse_lines<T: Iterator<Item = String>>(lines: T) -> Vec<i32> {
    lines
        .filter_map(|line| {
            let line = line.trim();
            let number: String = line.chars().skip(1).collect();
            match (line.chars().nth(0), number.parse::<i32>()) {
                (Some('L'), Ok(num)) => Some(-1 * (num % 100)),
                (Some('R'), Ok(num)) => Some(num % 100),
                _ => None,
            }
        })
        .collect()
}

fn part1(dial_moves: &Vec<i32>) -> Result<i32, String> {
    let state = dial_moves
        .iter()
        .fold(DialState::default(), |dial_state, dial_move| {
            // println!("State {:#?}, move {}", dial_state, dial_move);
            let new_position = (dial_state.position + (100 + dial_move)) % 100;
            DialState {
                zero_count: if new_position == 0 {
                    dial_state.zero_count + 1
                } else {
                    dial_state.zero_count
                },
                position: new_position,
            }
        });
    println!("The password is {}", state.zero_count);
    Ok(state.zero_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82";

    #[test]
    fn day_one_part_one(){
        let moves = parse_lines(EXAMPLE_INPUT.lines().map(|l| String::from(l)));
        let password = part1(&moves);
        assert!(password.is_ok_and(|pw| pw == 3));
    }
}

pub fn solution(input: &std::path::Path, parts: PuzzleParts) -> Result<(), String> {
    let f = File::open(input);
    match f {
        Err(error) => Err(format!("Got error {} reading {}", error, input.display())),
        Ok(file) => {
            let reader = BufReader::new(file);
            let dial_moves = parse_lines(reader.lines().map_while(Result::ok));
            if parts == PuzzleParts::PartOne || parts == PuzzleParts::All {
                part1(&dial_moves)?;
            }
            Ok(())
        }
    }
}
