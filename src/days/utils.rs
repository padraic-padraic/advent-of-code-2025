use std::Result;

pub fn validate_puzzle_file(input_args: Option<std::path::PathBuf>, day: u8) ->  Result<std::path::Path, String> {
    let input_path = match input_args {
        None => std::env::current_dir(),
        Some(path) => Ok(path)
    };
    if let Ok(path) = input_path {
        if path.is_dir() {
            path = path.join(format!("day{:02}.txt"));
        }
        if path.exists() {
            Ok(path.as_path())
        }
        else {
            Err(format!("Puzzle input {} not found", path))
        }
    } else {
        Err("Unable to resolve working directory, check your permissions.")
    }
}

