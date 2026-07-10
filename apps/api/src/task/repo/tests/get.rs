use sqlx::{PgPool, test};

use crate::{
    tag::{
        repo::{CreateModel as TagCreateModel, PgTagRepository, TagRepository},
        
    },
    task::{
        repo::{CreateModel, PgTaskRepository, TaskRepository},
        types::TaskID,
    },
    tests::helpers::{create_test_user, soft_delete_task},
    user::repo::PgUserRepository,
};

#[test]
async fn get_existing(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let test_task = repo.create(user.id, CreateModel::default()).await.unwrap();

    let res = repo.get(test_task.id, user.id).await;
    assert!(res.is_ok());
    if let Ok(task_opt) = res {
        assert!(task_opt.is_some());
        if let Some(task) = task_opt {
            assert_eq!(task, test_task);
        }
    }
}

#[test]
async fn get_soft_deleted(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let test_task = repo.create(user.id, CreateModel::default()).await.unwrap();
    soft_delete_task(&repo, test_task.id, user.id).await;

    let res = repo.get(test_task.id, user.id).await;
    assert!(res.is_ok());
    if let Ok(task_opt) = res {
        assert!(task_opt.is_none());
    }
}

#[test]
async fn get_deleted(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let test_task = repo.create(user.id, CreateModel::default()).await.unwrap();
    repo.delete(test_task.id, user.id).await.unwrap();

    let res = repo.get(test_task.id, user.id).await;
    assert!(res.is_ok());
    if let Ok(task_opt) = res {
        assert!(task_opt.is_none());
    }
}

#[test]
async fn get_not_owned(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let other_user = create_test_user(&user_repo).await;
    let other_task = repo
        .create(other_user.id, CreateModel::default())
        .await
        .unwrap();

    let res = repo.get(other_task.id, user.id).await;
    assert!(res.is_ok());
    if let Ok(task_opt) = res {
        assert!(task_opt.is_none());
    }
}

#[test]
async fn get_nonexistent(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    let res = repo.get(TaskID::new_v4(), user.id).await;
    assert!(res.is_ok());
    if let Ok(task_opt) = res {
        assert!(task_opt.is_none());
    }
}

#[test]
async fn get_has_tags(pool: PgPool) {
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
                tags: test_tags.iter().map(|tag| tag.id).collect(),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    let res = repo.get(test_task.id, user.id).await;
    assert!(res.is_ok());
    if let Ok(task_opt) = res {
        assert!(task_opt.is_some());
        if let Some(task) = task_opt {
            assert_eq!(task.tags.len(), test_task.tags.len());
            for tag in task.tags {
                assert!(test_tags.contains(&tag));
            }
        }
    }
}
