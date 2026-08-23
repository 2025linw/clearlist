use sqlx::QueryBuilder;

use crate::{category::types::CategoryID, types::repo::Pagination};

#[derive(Debug, Default, Clone)]
pub struct QueryOpts {
    pub filter: Filter,
    pub pagination: Pagination,
}

impl QueryOpts {
    pub fn add_to_builder(self, builder: &mut QueryBuilder<'_, sqlx::Postgres>) {
        // Filter
        if let Some(category_id) = self.filter.category {
            builder.push(" AND category_id = ");
            builder.push_bind(category_id);
        }

        // Sort (forced)
        builder.push(" ORDER BY tc.position_key, t.position_key, t.updated_at DESC");

        // Pagination
        if let Some(limit) = self.pagination.limit {
            builder.push(format!(" LIMIT {limit}"));
        }
        if let Some(offset) = self.pagination.offset {
            builder.push(format!(" OFFSET {offset}"));
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Filter {
    pub category: Option<CategoryID>,
}

impl Filter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn category(&mut self, category: CategoryID) {
        self.category = Some(category);
    }
}

#[derive(Debug, Clone)]
pub struct CreateModel {
    pub label: String,
    pub category_id: Option<CategoryID>,

    pub position_key: String,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateModel {
    pub label: Option<String>,
    pub category_id: Option<Option<CategoryID>>,

    pub position_key: Option<String>,
}

impl UpdateModel {
    pub fn add_to_builder(self, builder: &mut QueryBuilder<'_, sqlx::Postgres>) {
        let mut separated = builder.separated(", ");
        if let Some(label) = self.label {
            separated.push("label = ");
            separated.push_bind_unseparated(label);
        }
        if let Some(category_id) = self.category_id {
            separated.push("category_id = ");
            separated.push_bind_unseparated(category_id);
        }

        if let Some(position_key) = self.position_key {
            separated.push("position_key = ");
            separated.push_bind_unseparated(position_key);
        }
    }
}
