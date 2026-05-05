pub fn first_token(line: &str) -> Option<&str> {
    line.split_whitespace().next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_first_word() {
        assert_eq!(first_token("hello world"), Some("hello"));
    }

    #[test]
    fn skips_leading_whitespace() {
        assert_eq!(first_token("   leading spaces"), Some("leading"));
    }

    #[test]
    fn handles_single_token() {
        assert_eq!(first_token("solo"), Some("solo"));
    }

    #[test]
    fn handles_empty() {
        assert_eq!(first_token(""), None);
    }

    #[test]
    fn handles_whitespace_only() {
        assert_eq!(first_token("   \t  "), None);
    }
}
