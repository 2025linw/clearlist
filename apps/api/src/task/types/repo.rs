use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{QueryBuilder, prelude::FromRow};

use crate::{
    tag::types::{Model as TagModel, TagID},
    task::types::TaskID,
    types::{
        date::{DateFilter, StartPrecision},
        order::SortOrder,
    },
};

use super::SortBy;

#[derive(Debug, Default)]
pub struct QueryOpts {
    pub pagination: Pagination,
    pub sort: Sort,
    pub filter: Filter,
}

impl QueryOpts {
    pub fn add_to_builder(self, builder: &mut QueryBuilder<'_, sqlx::Postgres>) {
        // Filter
        if let Some(filter) = self.filter.start {
            builder.push(" AND ");
            filter.add_to_builder(builder);
        }
        if let Some(filter) = self.filter.deadline {
            builder.push(" AND ");
            filter.add_to_builder(builder);
        }
        if let Some(completed) = self.filter.completed {
            builder.push(" AND ");
            if completed {
                builder.push("completed_at IS NOT NULL");
            } else {
                builder.push("completed_at IS NULL");
            }
        }
        if let Some(deleted) = self.filter.deleted {
            builder.push(" AND ");
            if deleted {
                builder.push("deleted_at IS NOT NULL");
            } else {
                builder.push("deleted_at IS NULL");
            }
        }
        if let Some(tags) = self.filter.tags
            && !tags.is_empty()
        {
            // NOTE: Make sure this is last as it will contain `HAVING` clauses
            builder.push(" AND tt.tag_id = ANY(");
            builder.push_bind(tags.clone());
            builder.push(") GROUP BY t.id HAVING COUNT(DISTINCT tt.tag_id) = cardinality(");
            builder.push_bind(tags.clone());
            builder.push(")");
        } else {
            builder.push(" GROUP BY t.id");
        }

        // Sort
        if let Some((by, order)) = self.sort.sort {
            builder.push(format!(" ORDER BY {} {} NULLS LAST", by, order));
        } else {
            builder.push(" ORDER BY id ASC");
        }

        // Pagination
        if let Some(limit) = self.pagination.limit {
            builder.push(format!(" LIMIT {limit}"));
        }
        if let Some(offset) = self.pagination.offset {
            builder.push(format!(" OFFSET {offset}"));
        }
    }
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct CreateModel {
    pub title: String,
    pub notes: Option<String>,
    pub start: Option<chrono::DateTime<chrono::Utc>>,
    pub start_precision: StartPrecision,
    pub deadline: Option<chrono::NaiveDate>,
    pub tags: Vec<TagID>,

    pub position_key: String,
}

#[derive(Debug)]
#[cfg_attr(test, derive(Clone))]
pub struct UpdateModel {
    pub title: Option<String>,
    pub notes: Option<Option<String>>,
    pub start: Option<Option<chrono::DateTime<chrono::Utc>>>,
    pub start_precision: Option<StartPrecision>,
    pub deadline: Option<Option<chrono::NaiveDate>>,
    pub tags: Option<Vec<TagID>>,

    pub completed: Option<bool>,
    pub deleted: Option<bool>,

    pub position_key: Option<String>,
}

impl UpdateModel {
    pub fn add_to_builder(self, builder: &mut QueryBuilder<'_, sqlx::Postgres>) {
        let mut separated = builder.separated(", ");
        if let Some(title) = self.title {
            separated.push("title = ");
            separated.push_bind_unseparated(title);
        }
        if let Some(notes) = self.notes {
            separated.push("notes = ");
            separated.push_bind_unseparated(notes);
        }
        if let Some(start) = self.start {
            separated.push("start_dt = ");
            separated.push_bind_unseparated(start);
        }
        if let Some(start_precision) = self.start_precision {
            separated.push("has_time = ");
            separated.push_bind_unseparated(matches!(start_precision, StartPrecision::DateTime));
        }
        if let Some(deadline) = self.deadline {
            separated.push("deadline = ");
            separated.push_bind_unseparated(deadline);
        }

        if let Some(completed) = self.completed {
            if completed {
                separated.push("completed_at = CURRENT_TIMESTAMP");
            } else {
                separated.push("completed_at = NULL");
            }
        }
        if let Some(deleted) = self.deleted {
            if deleted {
                separated.push("deleted_at = CURRENT_TIMESTAMP");
            } else {
                separated.push("deleted_at = NULL");
            }
        }

        if let Some(position_key) = self.position_key {
            separated.push("position_key = ");
            separated.push_bind_unseparated(position_key);
        }
    }
}

#[derive(FromRow)]
pub struct TaskTag {
    pub task_id: TaskID,

    #[sqlx(flatten)]
    pub tag: TagModel,
}

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
}

impl<T> TaskState<T> {
    pub fn exists(&self) -> bool {
        matches!(self, Self::Existing(_))
    }

    pub fn deleted(&self) -> bool {
        matches!(self, Self::Deleted(_))
    }

    pub fn expect(self, msg: &str) -> T {
        match self {
            Self::Existing(var) => var,
            Self::Deleted(_) => {
                panic!("{msg}: Deleted")
            }
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            Self::Existing(var) => var,
            Self::Deleted(_) => {
                panic!("called `TaskState::unwrap()` on a soft-deleted value")
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
        }
    }
}

impl<T> std::fmt::Display for TaskState<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Existing(_) => write!(f, "operation succeeded"),
            Self::Deleted(_) => write!(f, "operation performed with soft-deleted task"),
        }
    }
}
