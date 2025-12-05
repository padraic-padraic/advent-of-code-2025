use itertools::Itertools;

use super::PuzzleParts;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::{FromStr};


#[derive(Debug)]
struct IdRange{
    start: u64,
    end: u64
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ParseRangeError;


impl FromStr for IdRange {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((start, end)) = s.split('-').next_tuple() {
            match (start.parse::<u64>(), end.parse::<u64>()) {
                (Ok(s), Ok(e)) => Ok(IdRange{start: s, end: e}),
                _ => Err(ParseRangeError)
            }
        } else {
            Err(ParseRangeError)
        }
    }
}

impl IdRange {
    fn sum_invalid_ids(&self) -> u64 {
        (self.start..self.end).fold(0, |sum, number| {
            let s = number.to_string();
            let digits = s.chars().count();
            if digits % 2 == 0 {
                let fold_point = digits / 2;
                let s1: String = s.chars().take(fold_point).collect();
                let s2: String = s.chars().skip(fold_point).collect();
                if s1 == s2 {
                    return sum + number
                }
            }
            sum
        })
    }
}


fn parse_line(line: &str) -> Result<Vec<IdRange>, ParseRangeError> {
    let ranges = line.trim().split(',').map(|part| part.parse::<IdRange>()).collect::<Result<_, _>>()?;
    Ok(ranges)
}

fn part1(ranges: &Vec<IdRange>) -> Result<u64, String> {
    let sum: u64 = 0;
    let sum = ranges.iter().fold(sum, |sum, range| {
        sum + range.sum_invalid_ids()
    });
    println!("Id Sum is {}", sum);
    Ok(sum)
}
#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn day_two_part_one() {
        let ranges = parse_line(EXAMPLE_INPUT).unwrap();
        assert!(part1(&ranges).is_ok_and(|sum| sum == 1227775554));
    }
}


pub fn solution(input: &std::path::Path, parts: PuzzleParts) -> Result<(), String> {
    let f = File::open(input);
    match f {
        Err(error) => Err(format!("Got error {} reading {}", error, input.display())),
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut s = String::new();
            let read_result = reader.read_to_string(&mut s);
            if let Err(e) = read_result {
                return Err(format!("Caught error {} reading from {:#?}", e, input))
            }
            if let Ok(ranges) = parse_line(s.as_str()) {
                if parts == PuzzleParts::PartOne || parts == PuzzleParts::All {
                    part1(&ranges)?;
                }
                // if parts == PuzzleParts::PartTwo || parts == PuzzleParts::All {
                //     part2(&dial_moves)?;
                // }
            } else {
                return Err("Error parsing the puzzle input".to_string())
            }

            Ok(())
        }
    }
}
