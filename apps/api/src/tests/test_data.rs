pub const NORMALIZATION_TEST_INPUT: [(&str, &str, &str); 4] = [
    ("contains spaces", "  Test Text     ", "Test Text"),
    ("contains newlines", "\nTest Text\n", "Test Text"),
    ("contains tabs", "\tTest Text\t", "Test Text"),
    ("combination", "\t   Test Text   \n", "Test Text"),
];

pub const CONTAINS_WHITESPACE_TEST_INPUT: [(&str, &str); 3] = [
    ("contains newline", "Test\nText"),
    ("contains tab", "Test\tText"),
    ("combindation", "Test\n\tText"),
];
