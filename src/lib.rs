pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    if pattern.is_empty() {
        eprintln!("Pattern must not be empty");
        return;
    }
    for line in content.lines() {
        if line.contains(pattern) {
            match writeln!(writer, "{}", line) {
                Ok(_) => {}
                Err(e) => eprintln!("Error writing to writer: {}", e),
            }
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
