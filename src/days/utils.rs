pub fn validate_puzzle_file(input_args: &Option<std::path::PathBuf>, day: u8) ->  Result<std::path::PathBuf, String> {
    let mut input_path = match input_args {
        None => {
            if let Ok(work_dir) = std::env::current_dir() {
                work_dir.join("inputs")
            } else {
                return Err(String::from("Unable to resolve working directory, check your permissions."))
            }
        }
        Some(path) => path.to_path_buf()
    };
    if input_path.is_dir() {
        input_path = input_path.join(format!("day{:02}.txt", day));
    }
    if input_path.exists() {
        Ok(input_path)
    }
    else {
        Err(format!("Puzzle input {} not found", input_path.display()))
    }
}
