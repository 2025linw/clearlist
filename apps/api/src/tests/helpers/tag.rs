use super::generate_a_z;
use crate::{
    tag::repo::{CreateModel, PgTagRepository, TagRepository, UpdateModel},
    user::types::UserID,
};

pub async fn seed_tags(
    repo: &PgTagRepository,
    n: usize,
    user_id: UserID,
    builder: impl Fn(usize) -> CreateModel,
) {
    for i in 0..n {
        repo.create(user_id, builder(i)).await.unwrap();
    }
}

pub fn default_tag(i: usize) -> CreateModel {
    CreateModel {
        position_key: format!("def{}", generate_a_z(i)),
        ..Default::default()
    }
}

pub fn tag_with_workflow_category(i: usize) -> CreateModel {
    CreateModel {
        category: Some("Workflow".to_string()),
        position_key: format!("workcat{}", generate_a_z(i)),
        ..Default::default()
    }
}

pub fn tag_with_workflow_priority(i: usize) -> CreateModel {
    CreateModel {
        category: Some("Priority".to_string()),
        position_key: format!("priocat{}", generate_a_z(i)),
        ..Default::default()
    }
}

impl Default for UpdateModel {
    fn default() -> Self {
        Self {
            label: Some("Updated Tag".to_string()),
            category: Some(Some("New Category".to_string())),
            position_key: Some(generate_a_z(1).to_string()),
        }
    }
}
