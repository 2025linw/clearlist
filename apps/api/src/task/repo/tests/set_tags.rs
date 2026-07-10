use sqlx::{PgPool, test};

use crate::{
    error::repo::{ConstraintViolation, Error, Resource},
    tag::{
        repo::{CreateModel as TagCreateModel,PgTagRepository, TagRepository},
        types::{ TagID},
    },
    task::{
        repo::{CreateModel, PgTaskRepository, TaskRepository},
        types::TaskID,
    },
    tests::helpers::{create_test_user, soft_delete_task},
    user::repo::PgUserRepository,
};

#[test]
async fn set_tags_task_existing(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let mut test_tags = Vec::with_capacity(5);
    for _ in 0..5 {
        let tag = tag_repo
            .create(user.id, TagCreateModel::default())
            .await
            .unwrap();

        test_tags.push(tag);
    }
    let test_task = repo.create(user.id, CreateModel::default()).await.unwrap();

    let res = repo
        .set_tags(
            test_task.id,
            user.id,
            test_tags.iter().map(|tag| tag.id).collect(),
        )
        .await;
    assert!(res.is_ok());
    if let Ok(tags) = res {
        for tag in tags {
            assert!(test_tags.contains(&tag));
        }
    }

    let task = repo
        .get(test_task.id, user.id)
        .await
        .unwrap()
        .expect("task was just created for the test");
    for tag in task.tags {
        assert!(test_tags.contains(&tag));
    }
}

#[test]
async fn set_tags_updates_task_updated_at(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let mut test_tags = Vec::with_capacity(5);
    for _ in 0..5 {
        let tag = tag_repo
            .create(user.id, TagCreateModel::default())
            .await
            .unwrap();

        test_tags.push(tag);
    }
    let test_task = repo.create(user.id, CreateModel::default()).await.unwrap();

    let res = repo
        .set_tags(
            test_task.id,
            user.id,
            test_tags.iter().map(|tag| tag.id).collect(),
        )
        .await;
    assert!(res.is_ok());
    let task = repo
        .get(test_task.id, user.id)
        .await
        .unwrap()
        .expect("task was just created for the test");
    assert!(task.updated_at > test_task.updated_at);
}

#[test]
async fn set_tags_task_soft_deleted(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let mut test_tags = Vec::with_capacity(5);
    for _ in 0..5 {
        let tag = tag_repo
            .create(user.id, TagCreateModel::default())
            .await
            .unwrap();

        test_tags.push(tag);
    }
    let test_task = repo.create(user.id, CreateModel::default()).await.unwrap();
    soft_delete_task(&repo, test_task.id, user.id).await;

    let res = repo
        .set_tags(
            test_task.id,
            user.id,
            test_tags.iter().map(|tag| tag.id).collect(),
        )
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
        ))
    }
}

#[test]
async fn set_tags_task_deleted(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let mut test_tags = Vec::with_capacity(5);
    for _ in 0..5 {
        let tag = tag_repo
            .create(user.id, TagCreateModel::default())
            .await
            .unwrap();

        test_tags.push(tag);
    }
    let test_task = repo.create(user.id, CreateModel::default()).await.unwrap();
    repo.delete(test_task.id, user.id).await.unwrap();

    let res = repo
        .set_tags(
            test_task.id,
            user.id,
            test_tags.iter().map(|tag| tag.id).collect(),
        )
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
        ))
    }
}

#[test]
async fn set_tags_task_not_owned(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let other_user = create_test_user(&user_repo).await;
    let mut test_tags = Vec::with_capacity(5);
    for _ in 0..5 {
        let tag = tag_repo
            .create(user.id, TagCreateModel::default())
            .await
            .unwrap();

        test_tags.push(tag);
    }
    let other_task = repo
        .create(other_user.id, CreateModel::default())
        .await
        .unwrap();

    let res = repo
        .set_tags(
            other_task.id,
            user.id,
            test_tags.iter().map(|tag| tag.id).collect(),
        )
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
        ))
    }
}

#[test]
async fn set_tags_task_nonexistent(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let mut test_tags = Vec::with_capacity(5);
    for _ in 0..5 {
        let tag = tag_repo
            .create(user.id, TagCreateModel::default())
            .await
            .unwrap();

        test_tags.push(tag);
    }

    let res = repo
        .set_tags(
            TaskID::new_v4(),
            user.id,
            test_tags.iter().map(|tag| tag.id).collect(),
        )
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
        ))
    }
}

#[test]
async fn set_tags_not_owned(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let other_user = create_test_user(&user_repo).await;
    let mut other_tags = Vec::with_capacity(5);
    for _ in 0..5 {
        let tag = tag_repo
            .create(other_user.id, TagCreateModel::default())
            .await
            .unwrap();

        other_tags.push(tag);
    }
    let test_task = repo.create(user.id, CreateModel::default()).await.unwrap();

    let res = repo
        .set_tags(
            test_task.id,
            user.id,
            other_tags.iter().map(|tag| tag.id).collect(),
        )
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Tag))
        ))
    }
}

#[test]
async fn set_tags_nonexistent(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let test_task = repo.create(user.id, CreateModel::default()).await.unwrap();

    let res = repo
        .set_tags(test_task.id, user.id, vec![TagID::new_v4()])
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Tag))
        ))
    }
}

#[test]
async fn set_tags_full_replacement(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let mut test_tags = Vec::with_capacity(5);
    for _ in 0..5 {
        let tag = tag_repo
            .create(user.id, TagCreateModel::default())
            .await
            .unwrap();

        test_tags.push(tag);
    }
    let test_task = repo
        .create(
            user.id,
            CreateModel {
                tags: test_tags[0..2].iter().map(|tag| tag.id).collect(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    for tag in test_task.tags {
        assert!(test_tags[0..2].contains(&tag));
    }

    let res = repo
        .set_tags(
            test_task.id,
            user.id,
            test_tags.iter().map(|tag| tag.id).collect(),
        )
        .await;
    assert!(res.is_ok());
    if let Ok(tags) = res {
        for tag in tags {
            assert!(test_tags.contains(&tag));
        }
    }
    let task = repo
        .get(test_task.id, user.id)
        .await
        .unwrap()
        .expect("task was just created for the test");
    for tag in task.tags {
        assert!(test_tags.contains(&tag));
    }
}
