mod create;
mod delete;
mod get;
mod list;
mod update;

use crate::{category::service::CategoryService, tests::mocks::category::MockCategoryRepository};

fn init_test_setup() -> CategoryService<MockCategoryRepository> {
    let category_repo = MockCategoryRepository::new();

    CategoryService::init(category_repo)
}
