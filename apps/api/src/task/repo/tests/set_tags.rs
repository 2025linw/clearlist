use sqlx::{PgPool, test};

use crate::{
    error::repo::{ConstraintViolation, Error, Resource},
    tag::{
        repo::{PgTagRepository, TagRepository},
        types::{TagID, repo::CreateModel as TagCreateModel},
    },
    task::{
        repo::{CreateModel, PgTaskRepository, TaskRepository},
        types::TaskID,
    },
    tests::helpers::{create_test_user, soft_delete_task},
    user::repo::PgUserRepository,
};

// Existence Tests
#[test]
async fn task_exists(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let mut test_tags = Vec::with_capacity(5);
    for _ in 0..5 {
        test_tags.push(
            tag_repo
                .create(test_user.id, TagCreateModel::default())
                .await
                .unwrap(),
        );
    }
    let test_task = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    let res = repo
        .set_tags(
            test_task.id,
            test_user.id,
            test_tags.iter().map(|tag| tag.id).collect(),
        )
        .await;
    assert!(res.is_ok());
    if let Ok(task_state) = res {
        assert!(task_state.exists());

        let tags = task_state.unwrap();
        assert_eq!(tags.len(), test_tags.len());
        for tag in tags {
            assert!(test_tags.contains(&tag));
        }
    }

    let task = repo
        .get(test_task.id, test_user.id)
        .await
        .unwrap()
        .expect("task was just created for the test");
    assert_eq!(task.tags.len(), test_tags.len());
    for tag in task.tags {
        assert!(test_tags.contains(&tag));
    }
}

#[test]
async fn task_soft_deleted(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let mut test_tags = Vec::with_capacity(5);
    for _ in 0..5 {
        let tag = tag_repo
            .create(test_user.id, TagCreateModel::default())
            .await
            .unwrap();

        test_tags.push(tag);
    }
    let test_task = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();
    soft_delete_task(&repo, test_task.id, test_user.id).await;

    let res = repo
        .set_tags(
            test_task.id,
            test_user.id,
            test_tags.iter().map(|tag| tag.id).collect(),
        )
        .await;
    assert!(res.is_ok());
    if let Ok(task_state) = res {
        assert!(task_state.deleted());
    }
}

#[test]
async fn task_not_owned(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let other_user = create_test_user(&user_repo).await;
    let mut test_tags = Vec::with_capacity(5);
    for _ in 0..5 {
        let tag = tag_repo
            .create(test_user.id, TagCreateModel::default())
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
            test_user.id,
            test_tags.iter().map(|tag| tag.id).collect(),
        )
        .await;
    assert!(res.is_ok());
    if let Ok(task_state) = res {
        assert!(task_state.missing());
    }
}

#[test]
async fn task_not_exist(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let mut test_tags = Vec::with_capacity(5);
    for _ in 0..5 {
        let tag = tag_repo
            .create(test_user.id, TagCreateModel::default())
            .await
            .unwrap();

        test_tags.push(tag);
    }

    let res = repo
        .set_tags(
            TaskID::new_v4(),
            test_user.id,
            test_tags.iter().map(|tag| tag.id).collect(),
        )
        .await;
    assert!(res.is_ok());
    if let Ok(task_state) = res {
        assert!(task_state.missing());
    }
}

#[test]
async fn tags_not_owned(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let other_user = create_test_user(&user_repo).await;
    let mut other_tags = Vec::with_capacity(5);
    for _ in 0..5 {
        let tag = tag_repo
            .create(other_user.id, TagCreateModel::default())
            .await
            .unwrap();

        other_tags.push(tag);
    }
    let test_task = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    let res = repo
        .set_tags(
            test_task.id,
            test_user.id,
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
async fn tags_not_exist(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_task = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    let res = repo
        .set_tags(test_task.id, test_user.id, vec![TagID::new_v4()])
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Tag))
        ))
    }
}

// Behavior Tests
#[test]
async fn updates_updated_at(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let mut test_tags = Vec::with_capacity(5);
    for _ in 0..5 {
        let tag = tag_repo
            .create(test_user.id, TagCreateModel::default())
            .await
            .unwrap();

        test_tags.push(tag);
    }
    let test_task = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    let res = repo
        .set_tags(
            test_task.id,
            test_user.id,
            test_tags.iter().map(|tag| tag.id).collect(),
        )
        .await;
    assert!(res.is_ok());
    let task = repo
        .get(test_task.id, test_user.id)
        .await
        .unwrap()
        .expect("task was just created for the test");
    assert!(task.updated_at > test_task.updated_at);
}

#[test]
async fn full_replacement(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let mut test_tags = Vec::with_capacity(5);
    for _ in 0..5 {
        let tag = tag_repo
            .create(test_user.id, TagCreateModel::default())
            .await
            .unwrap();

        test_tags.push(tag);
    }
    let test_task = repo
        .create(
            test_user.id,
            CreateModel {
                tags: test_tags[0..2].iter().map(|tag| tag.id).collect(),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    let res = repo
        .set_tags(
            test_task.id,
            test_user.id,
            test_tags.iter().map(|tag| tag.id).collect(),
        )
        .await;
    assert!(res.is_ok());
    if let Ok(task_state) = res {
        assert!(task_state.exists());
        let tags = task_state.unwrap();
        for tag in tags {
            assert!(test_tags.contains(&tag));
        }
    }
    let task = repo
        .get(test_task.id, test_user.id)
        .await
        .unwrap()
        .expect("task was just created for the test");
    for tag in task.tags {
        assert!(test_tags.contains(&tag));
    }
}
