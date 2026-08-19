pub mod tag;
pub mod task;
pub mod user;

use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

pub use tag::MockTagRepository;
pub use task::MockTaskRepository;
pub use user::MockUserRepository;

use crate::{
    Config, GenericAppState, tag::service::TagService, task::service::TaskService,
    user::service::UserService,
};

pub type MockDB<K, V> = Arc<RwLock<HashMap<K, V>>>;

pub type MockAppState = GenericAppState<MockUserRepository, MockTaskRepository, MockTagRepository>;

impl MockAppState {
    pub fn init(config: Config) -> Self {
        let user_repo = MockUserRepository::new();
        let tag_repo = MockTagRepository::init(user_repo.clone());
        let task_repo = MockTaskRepository::init(user_repo.clone(), tag_repo.clone());

        Self {
            config,
            user_service: UserService::init(user_repo),
            task_service: TaskService::init(task_repo),
            tag_service: TagService::init(tag_repo),
        }
    }
}
