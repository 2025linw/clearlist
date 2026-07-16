use sqlx::{PgPool, test};

use crate::{
    tag::repo::{CreateModel as TagCreateModel, PgTagRepository, TagRepository},
    task::{
        repo::{CreateModel, PgTaskRepository, TaskRepository},
        types::TaskID,
    },
    tests::helpers::{create_test_user, soft_delete_task},
    user::repo::PgUserRepository,
};

#[test]
async fn list_tags_task_existing(pool: PgPool) {
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
                tags: test_tags.iter().map(|task| task.id).collect(),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    let res = repo.list_tags(test_task.id, user.id).await;
    assert!(res.is_ok());
    if let Ok(task_tag_state) = res {
        assert!(task_tag_state.exists());
        let tags = task_tag_state.unwrap();
        for tag in tags {
            assert!(test_tags.contains(&tag));
        }
    }
}

#[test]
async fn list_tags_task_soft_deleted(pool: PgPool) {
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
                tags: test_tags.iter().map(|task| task.id).collect(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    soft_delete_task(&repo, test_task.id, user.id).await;

    let res = repo.list_tags(test_task.id, user.id).await;
    assert!(res.is_ok());
    if let Ok(task_tag_state) = res {
        assert!(task_tag_state.deleted());
    }
}

#[test]
async fn list_tags_task_deleted(pool: PgPool) {
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
                tags: test_tags.iter().map(|task| task.id).collect(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    repo.delete(test_task.id, user.id).await.unwrap();

    let res = repo.list_tags(test_task.id, user.id).await;
    assert!(res.is_ok());
    if let Ok(task_tag_state) = res {
        assert!(task_tag_state.missing());
    }
}

#[test]
async fn list_tags_task_not_owned(pool: PgPool) {
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
    let other_task = repo
        .create(
            other_user.id,
            CreateModel {
                tags: other_tags.iter().map(|task| task.id).collect(),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    let res = repo.list_tags(other_task.id, user.id).await;
    assert!(res.is_ok());
    if let Ok(task_tag_state) = res {
        assert!(task_tag_state.missing());
    }
}

#[test]
async fn list_tags_task_nonexistent(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    let res = repo.list_tags(TaskID::new_v4(), user.id).await;
    assert!(res.is_ok());
    if let Ok(task_tag_state) = res {
        assert!(task_tag_state.missing());
    }
}
