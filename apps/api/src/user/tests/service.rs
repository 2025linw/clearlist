mod create;
mod get;
mod update;

use crate::{tests::mocks::MockUserRepository, user::service::UserService};

fn init_test_setup() -> UserService<MockUserRepository> {
    UserService::init(MockUserRepository::new())
}
