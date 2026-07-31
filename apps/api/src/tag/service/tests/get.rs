use tokio::test;

use crate::{
    error::{Resource, service::Error},
    tag::{
        service::{TagService, TagServiceTrait},
        types::{TagID, route::CreateRequest},
    },
    tests::mocks::tag::MockTagRepository,
    user::types::UserID,
};

#[test]
async fn success() {
    let service = TagService::init(MockTagRepository::new());

    let test_user_id = service.repo.add_user().await;
    let test_tag = service
        .create(test_user_id, CreateRequest::default())
        .await
        .unwrap();

    let res = service.get(test_tag.id, test_user_id).await;
    assert!(res.is_ok());
}

#[test]
async fn not_owned() {
    let service = TagService::init(MockTagRepository::new());

    let other_user_id = service.repo.add_user().await;
    let other_tag = service
        .create(other_user_id, CreateRequest::default())
        .await
        .unwrap();

    let res = service.get(other_tag.id, UserID::new_v4()).await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(err, Error::NotFound(Resource::Tag)));
    }
}

#[test]
async fn not_exists() {
    let service = TagService::init(MockTagRepository::new());

    let test_user_id = service.repo.add_user().await;

    let res = service.get(TagID::new_v4(), test_user_id).await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(err, Error::NotFound(Resource::Tag)));
    }
}

// repo errors
#[test]
async fn repo_backend_error() {
    let service = TagService::init(MockTagRepository::new_backend_error());

    let res = service.get(TagID::new_v4(), UserID::new_v4()).await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(err, Error::Internal(_)));
    }
}

#[test]
async fn repo_programming_error() {
    let service = TagService::init(MockTagRepository::new_programming_error());

    let res = service.get(TagID::new_v4(), UserID::new_v4()).await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(err, Error::Unhandled(_)));
    }
}
