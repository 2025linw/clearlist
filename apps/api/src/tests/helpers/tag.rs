use crate::{
    tag::{
        repo::{PgTagRepository, TagRepository},
        types::{
            CategoryID,
            repo::{CreateModel, UpdateModel},
            route::{CreateRequest, UpdateRequest},
        },
    },
    user::types::UserID,
};

use super::generate_a_z;

pub async fn seed_tags(
    repo: &PgTagRepository,
    n: usize,
    user_id: UserID,
    builder: impl Fn(usize, Option<CategoryID>) -> CreateModel,
) {
    for i in 0..n {
        repo.create(user_id, builder(i, None)).await.unwrap();
    }
}

pub async fn seed_tags_with_category(
    repo: &PgTagRepository,
    n: usize,
    user_id: UserID,
    tag_category_id: CategoryID,
    builder: impl Fn(usize, Option<CategoryID>) -> CreateModel,
) {
    for i in 0..n {
        repo.create(user_id, builder(i, Some(tag_category_id)))
            .await
            .unwrap();
    }
}

pub fn default_tag(i: usize, tag_category_id: Option<CategoryID>) -> CreateModel {
    CreateModel {
        label: format!("Default Tag {}", i),
        category_id: tag_category_id,
        position_key: format!("def{}", generate_a_z(i)),
    }
}

pub fn tag_with_workflow_category(i: usize, tag_category_id: Option<CategoryID>) -> CreateModel {
    CreateModel {
        label: format!("Workflow Tag {}", i),
        category_id: tag_category_id,
        position_key: format!("workflow{}", generate_a_z(i)),
    }
}

pub fn tag_with_priority_category(i: usize, tag_category_id: Option<CategoryID>) -> CreateModel {
    CreateModel {
        label: format!("Priority Tag {}", i),
        category_id: tag_category_id,
        position_key: format!("priority{}", generate_a_z(i)),
    }
}

pub fn full_tag(i: usize, tag_category_id: Option<CategoryID>) -> CreateModel {
    CreateModel {
        label: format!("Full Tag {}", i),
        category_id: tag_category_id,
        position_key: format!("full{}", generate_a_z(i)),
    }
}

impl Default for CreateRequest {
    fn default() -> Self {
        Self {
            label: "Test Tag".to_string(),
            category: None,
            position_key: generate_a_z(0).to_string(),
        }
    }
}

impl Default for UpdateRequest {
    fn default() -> Self {
        Self {
            label: Some("Updated Tag".to_string()),
            category: None,
            position_key: None,
        }
    }
}

impl Default for CreateModel {
    fn default() -> Self {
        Self {
            label: "Test Tag".to_string(),
            category_id: None,
            position_key: generate_a_z(0).to_string(),
        }
    }
}

impl Default for UpdateModel {
    fn default() -> Self {
        Self {
            label: Some("Updated Tag".to_string()),
            category_id: None,
            position_key: None,
        }
    }
}
