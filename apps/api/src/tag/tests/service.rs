mod create;
mod delete;
mod get;
mod list;
mod update;

use crate::{
    tag::service::TagService,
    tests::mocks::{MockTagRepository, category::MockCategoryRepository},
};

fn init_test_setup() -> TagService<MockTagRepository, MockCategoryRepository> {
    let tag_repo = MockTagRepository::new();

    TagService::init(tag_repo.clone(), tag_repo.categories())
}
