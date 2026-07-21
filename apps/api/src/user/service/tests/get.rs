use tokio::test;

use crate::{
    error::{Resource, service::Error},
    tests::mocks::user::MockUserRepository,
    user::{
        service::{UserService, UserServiceTrait},
        types::{UserID, route::CreateRequest},
    },
};

#[test]
async fn success() {
    let service = UserService::init(MockUserRepository::new());

    let test_user = service.create(CreateRequest::default()).await.unwrap();

    let res = service.get(test_user.id).await;
    assert!(res.is_ok());
}

#[test]
async fn not_exists() {
    let service = UserService::init(MockUserRepository::new());

    let res = service.get(UserID::new_v4()).await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(err, Error::NotFound(Resource::User)))
    }
}

#[test]
async fn repo_backend_error() {
    let service = UserService::init(MockUserRepository::new_backend_error());

    let res = service.get(UserID::new_v4()).await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(err, Error::Internal(_)));
    }
}

#[test]
async fn repo_programming_error() {
    let service = UserService::init(MockUserRepository::new_programming_error());

    let res = service.get(UserID::new_v4()).await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(err, Error::Unhandled(_)));
    }
}
