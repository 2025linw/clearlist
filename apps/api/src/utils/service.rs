use std::{collections::HashSet, hash::Hash};

const VALID_MULTILINE_CHARS: [char; 3] = [' ', '\n', '\t'];

pub fn is_valid_single_line_string(s: &str) -> bool {
    !s.chars().any(|c| c.is_whitespace() && c != ' ')
}

pub fn is_valid_multiline_string(s: &str) -> bool {
    !s.chars()
        .any(|c| c.is_whitespace() && !VALID_MULTILINE_CHARS.contains(&c))
}

pub fn dedupe_array<T>(e: Vec<T>) -> Vec<T>
where
    T: Eq + Hash,
{
    e.into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>()
}
