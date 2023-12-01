pub fn read_file(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Cannot read file {}", file_path))
}
