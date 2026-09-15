use chrono::NaiveDate;

use crate::types::{field::Start, query::BracketInterval};

pub const NORMALIZATION_TEST_INPUT: [(&str, &str, &str); 4] = [
    ("contains spaces", "  Test Text     ", "Test Text"),
    ("contains newlines", "\nTest Text\n", "Test Text"),
    ("contains tabs", "\tTest Text\t", "Test Text"),
    ("combination", "\t   Test Text   \n", "Test Text"),
];

pub const SINGLE_LINE_TEST_INPUT: [(&str, &str); 3] = [
    ("contains newline", "Test\nText"),
    ("contains tab", "Test\tText"),
    ("combindation", "Test\n\tText"),
];

pub const MULTILINE_TEST_INPUT: [(&str, &str); 6] = [
    ("multiline 1", "Test\x0BText"), // \v
    ("multiline 2", "Test\x0CText"), // \f
    ("multiline 3", "Test\rText"),
    ("multiline 4", "Test\u{0085}Text"), // Next Line
    ("multiline 5", "Test\u{2028}Text"), // Line separator
    ("multiline 6", "Test\u{2029}Text"), // Paragraph separator
];

pub fn create_overspecified_start_range() -> Vec<BracketInterval<Start>> {
    vec![
        BracketInterval {
            ne: Some(Start::default()),
            lt: Some(Start::default()),
            ..Default::default()
        },
        BracketInterval {
            ne: Some(Start::default()),
            lte: Some(Start::default()),
            ..Default::default()
        },
        BracketInterval {
            ne: Some(Start::default()),
            gt: Some(Start::default()),
            ..Default::default()
        },
        BracketInterval {
            ne: Some(Start::default()),
            gte: Some(Start::default()),
            ..Default::default()
        },
        BracketInterval {
            lt: Some(Start::default()),
            lte: Some(Start::default()),
            ..Default::default()
        },
        BracketInterval {
            gt: Some(Start::default()),
            gte: Some(Start::default()),
            ..Default::default()
        },
    ]
}

pub fn create_overspecified_deadline_range() -> Vec<BracketInterval<NaiveDate>> {
    vec![
        BracketInterval {
            ne: Some(NaiveDate::MIN),
            lt: Some(NaiveDate::MIN),
            ..Default::default()
        },
        BracketInterval {
            ne: Some(NaiveDate::MIN),
            lte: Some(NaiveDate::MIN),
            ..Default::default()
        },
        BracketInterval {
            ne: Some(NaiveDate::MIN),
            gt: Some(NaiveDate::MIN),
            ..Default::default()
        },
        BracketInterval {
            ne: Some(NaiveDate::MIN),
            gte: Some(NaiveDate::MIN),
            ..Default::default()
        },
        BracketInterval {
            lt: Some(NaiveDate::MIN),
            lte: Some(NaiveDate::MIN),
            ..Default::default()
        },
        BracketInterval {
            gt: Some(NaiveDate::MIN),
            gte: Some(NaiveDate::MIN),
            ..Default::default()
        },
    ]
}
