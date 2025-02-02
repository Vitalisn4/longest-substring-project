// Function to find the longest substring without repeating characters
pub fn longest_unique_substring(s: &str) -> String {
    let mut map = std::collections::HashMap::new();
    let (mut start, mut max_len, mut max_substr) = (0, 0, String::new());

    for (i, c) in s.char_indices() {
        if let Some(&last_index) = map.get(&c) {
            // Move the start pointer if we see a duplicate character
            start = std::cmp::max(start, last_index + 1);
        }

        map.insert(c, i);

        let len = i - start + 1;
        if len > max_len {
            max_len = len;
            max_substr = s[start..=i].to_string();
        }
    }

    max_substr
}
