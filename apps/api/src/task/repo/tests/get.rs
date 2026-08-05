use sqlx::PgPool;

use crate::{
    task::{
        repo::{CreateModel, PgTaskRepository, TaskRepository},
        types::TaskID,
    },
    tests::helpers::{create_test_user, generate_a_z, get_today_date_pg, soft_delete_task},
    user::{repo::PgUserRepository, types::UserID},
};

async fn init(pool: PgPool) -> (UserID, TaskID, PgTaskRepository) {
    let user_repo = PgUserRepository::init(pool.clone());
    let task_repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let task = task_repo
        .create(user.id, CreateModel::default())
        .await
        .unwrap();

    (user.id, task.id, task_repo)
}

mod success {
    use sqlx::test;

    use super::*;

    #[test]
    async fn success(pool: PgPool) {
        let (user_id, task_id, task_repo) = init(pool).await;

        let res = task_repo.get(task_id, user_id).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_output(pool: PgPool) {
        let (user_id, _, task_repo) = init(pool).await;

        let create_model = CreateModel {
            title: "Test Task".to_string(),
            notes: Some("Note for 'Test Task'".to_string()),
            start: Some(get_today_date_pg()),
            has_time: true,
            deadline: Some(get_today_date_pg().date_naive()),
            position_key: generate_a_z(0).to_string(),
        };
        let test_task = task_repo
            .create(user_id, create_model.clone())
            .await
            .unwrap();

        let task = task_repo.get(test_task.id, user_id).await.unwrap().unwrap();

        let CreateModel {
            title,
            notes,
            start,
            has_time,
            deadline,
            position_key,
        } = create_model;
        assert_eq!(task.title, title);
        assert_eq!(task.notes, notes);
        assert_eq!(task.start_dt, start);
        assert_eq!(task.has_time, has_time);
        assert_eq!(task.deadline, deadline);
        assert_eq!(task.position_key, position_key);
    }
}

mod existence {
    use sqlx::test;

    use super::*;

    #[test]
    async fn soft_deleted(pool: PgPool) {
        let (user_id, task_id, task_repo) = init(pool).await;
        soft_delete_task(&task_repo, task_id, user_id).await;

        let task_state = task_repo.get(task_id, user_id).await.unwrap();
        assert!(task_state.deleted());
    }

    #[test]
    async fn not_owned(pool: PgPool) {
        let (_, task_id, _) = init(pool.clone()).await; // other task
        let (user_id, _, task_repo) = init(pool).await;

        let task_state = task_repo.get(task_id, user_id).await.unwrap();
        assert!(task_state.missing());
    }

    #[test]
    async fn not_exists(pool: PgPool) {
        let (user_id, _, task_repo) = init(pool).await;

        let task_state = task_repo.get(TaskID::new_v4(), user_id).await.unwrap();
        assert!(task_state.missing());
    }
}
