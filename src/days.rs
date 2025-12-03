pub mod utils;
pub mod day01;

#[derive(PartialEq, Debug)]
pub enum PuzzleParts {
    All,
    PartOne,
    PartTwo
}


#[derive(Debug)]
pub struct AoCArgs{
    pub day: u8,
    pub part: Option<u8>,
    pub input: Option<std::path::PathBuf>
}

pub fn dispatch_to_day(args: &AoCArgs) -> Result<(), String> {
    if args.day > 12 || args.day == 0 {
        return Err(String::from("Day must be in the range 1-12"));
    }
    let part_selection = match args.part {
        None => Some(PuzzleParts::All),
        Some(1u8) => Some(PuzzleParts::PartOne),
        Some(2u8) => Some(PuzzleParts::PartTwo),
        _ => None
    };
    match utils::validate_puzzle_file(&args.input, args.day) {
        Ok(input_path) => {
            if let Some(parts) = part_selection {
                match args.day {
                    1 => day01::solution(input_path.as_path(), parts),
                    _ => Err(format!("Day {} not yet implemented.", args.day)),
                }
            } else {
                Err(String::from("If specified, part must be one or two!"))
            }
        }
        Err(path_error) => {
            Err(path_error)
        }
    }
}
