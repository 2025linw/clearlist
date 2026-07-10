use sqlx::{PgPool, test};

use crate::{
    tests::helpers::create_user_model,
    user::{
        repo::{PgUserRepository, UserRepository},
        types::UserID,
    },
};

#[test]
async fn get_existing(pool: PgPool) {
    let repo = PgUserRepository::init(pool.clone());

    let test_user = repo.create(create_user_model()).await.unwrap();

    let res = repo.get(test_user.id).await;
    assert!(res.is_ok());
    if let Ok(user_opt) = res {
        assert!(user_opt.is_some());
        if let Some(user) = user_opt {
            assert_eq!(user, test_user);
        }
    }
}

#[test]
async fn get_nonexistent(pool: PgPool) {
    let repo = PgUserRepository::init(pool.clone());

    let res = repo.get(UserID::new_v4()).await;
    assert!(res.is_ok());
    if let Ok(task_opt) = res {
        assert!(task_opt.is_none());
    }
}
