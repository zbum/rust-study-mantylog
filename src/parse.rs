pub fn first_token(line: &str) -> Option<&str> {
    line.split_whitespace().next()
}
