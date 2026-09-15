pub mod category;
pub mod tag;
pub mod task;
pub mod user;

use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

pub use tag::MockTagRepository;
pub use task::MockTaskRepository;
pub use user::MockUserRepository;

use crate::{
    Config, GenericAppState, category::service::CategoryService, tag::service::TagService, task::service::TaskService, tests::mocks::category::MockCategoryRepository, user::service::UserService,
};

pub type MockDB<K, V> = Arc<RwLock<HashMap<K, V>>>;

pub type MockAppState = GenericAppState<
    MockUserRepository,
    MockTaskRepository,
    MockTagRepository,
    MockCategoryRepository,
>;

impl MockAppState {
    pub fn init(config: Config) -> Self {
        let user_repo = MockUserRepository::new();
        let category_repo = MockCategoryRepository::init(user_repo.clone());
        let tag_repo = MockTagRepository::init(user_repo.clone(), category_repo.clone());
        let task_repo = MockTaskRepository::init(user_repo.clone(), tag_repo.clone());

        Self {
            config,
            user_service: UserService::init(user_repo),
            task_service: TaskService::init(task_repo),
            tag_service: TagService::init(tag_repo, category_repo.clone()),
            category_service: CategoryService::init(category_repo),
        }
    }
}
