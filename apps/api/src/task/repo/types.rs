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
