use crate::{
    category::{
        repo::{CategoryRepository, PgCategoryRepository},
        types::{repo::CreateModel, route::CreateRequest},
    },
    user::types::UserID,
};

use super::generate_a_z;

pub async fn seed_categories(
    repo: &PgCategoryRepository,
    n: usize,
    user_id: UserID,
    builder: impl Fn(usize) -> CreateModel,
) {
    for i in 0..n {
        repo.create(user_id, builder(i)).await.unwrap();
    }
}

pub fn default_category(i: usize) -> CreateModel {
    CreateModel {
        name: format!("Default Category {i}"),
        position_key: format!("def{}", generate_a_z(i)),
    }
}

impl Default for CreateRequest {
    fn default() -> Self {
        Self {
            name: String::new(),
            position_key: generate_a_z(0).to_string(),
        }
    }
}

impl Default for CreateModel {
    fn default() -> Self {
        Self {
            name: String::new(),
            position_key: generate_a_z(0).to_string(),
        }
    }
}
