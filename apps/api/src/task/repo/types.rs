#![allow(warnings)]
#![allow(clippy::all)]
// WARN: REMOVE ABOVE

use chrono::{DateTime, NaiveDate, Utc};

use crate::{
    tag::types::TagID,
    task::types::SortBy,
    types::{date::DateFilter, order::SortOrder},
};

#[derive(Debug, Default)]
pub struct Filter {
    pub start: Option<DateFilter<DateTime<Utc>>>,
    pub deadline: Option<DateFilter<NaiveDate>>,

    pub completed: Option<bool>,
    pub deleted: Option<bool>,

    pub tags: Option<Vec<TagID>>,
}

impl Filter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(mut self, start: DateFilter<DateTime<Utc>>) -> Self {
        self.start = Some(start);

        self
    }

    pub fn deadline(mut self, deadline: DateFilter<NaiveDate>) -> Self {
        self.deadline = Some(deadline);

        self
    }

    pub fn completed(mut self, completed: bool) -> Self {
        self.completed = Some(completed);

        self
    }

    pub fn deleted(mut self, deleted: bool) -> Self {
        self.deleted = Some(deleted);

        self
    }

    pub fn tags(mut self, tags: Vec<TagID>) -> Self {
        self.tags = Some(tags);

        self
    }
}

#[derive(Debug, Default)]
pub struct Sort {
    pub sort: Option<(SortBy, SortOrder)>,
}

impl Sort {
    pub fn new(by: Option<SortBy>, order: SortOrder) -> Self {
        if let Some(by) = by {
            Self {
                sort: Some((by, order)),
            }
        } else {
            Self {
                sort: Some((SortBy::ID, order)),
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Pagination {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl Pagination {
    pub fn new(limit: Option<usize>, offset: Option<usize>) -> Self {
        Self { limit, offset }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TaskState<T> {
    Existing(T),
    Deleted(T),
    Missing,
}

impl<T> TaskState<T> {
    pub fn exists(&self) -> bool {
        matches!(self, Self::Existing(_))
    }

    pub fn deleted(&self) -> bool {
        matches!(self, Self::Deleted(_))
    }

    pub fn missing(&self) -> bool {
        matches!(self, Self::Missing)
    }

    pub fn expect(self, msg: &str) -> T {
        match self {
            Self::Existing(var) => var,
            Self::Deleted(_) => {
                panic!("{msg}: Deleted")
            }
            Self::Missing => {
                panic!("{msg}: Missing")
            }
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            Self::Existing(var) => var,
            Self::Deleted(_) | Self::Missing => {
                panic!("called `TaskState::unwrap()` on a non-`Existing` value")
            }
        }
    }

    pub fn map<U, F>(self, f: F) -> TaskState<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Existing(x) => TaskState::Existing(f(x)),
            Self::Deleted(x) => TaskState::Deleted(f(x)),
            Self::Missing => TaskState::Missing,
        }
    }
}

impl<T> std::fmt::Display for TaskState<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Existing(_) => write!(f, "operation succeeded"),
            Self::Deleted(_) => write!(f, "operation performed with deleted task"),
            Self::Missing => write!(f, "operation performed with missing task"),
        }
    }
}
