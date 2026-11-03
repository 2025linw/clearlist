mod create;
mod delete;
mod get;
mod list;
mod restore;
mod update;

mod complete;
mod reopen;

mod add_tag;
mod list_tags;
mod remove_tag;
mod set_tags;

use crate::{task::service::TaskService, tests::mocks::MockTaskRepository};

fn init_test_setup() -> TaskService<MockTaskRepository> {
    TaskService::init(MockTaskRepository::new())
}
