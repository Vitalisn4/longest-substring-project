use longest_substring::longest_unique_substring;

#[test]
fn test_no_repeating_characters() {
    let input = "abcabcbb";
    let expected = "abc";
    assert_eq!(longest_unique_substring(input), expected);
}

#[test]
fn test_all_unique_characters() {
    let input = "abcdef";
    let expected = "abcdef";
    assert_eq!(longest_unique_substring(input), expected);
}

#[test]
fn test_empty_string() {
    let input = "";
    let expected = "";
    assert_eq!(longest_unique_substring(input), expected);
}

#[test]
fn test_single_character() {
    let input = "a";
    let expected = "a";
    assert_eq!(longest_unique_substring(input), expected);
}
