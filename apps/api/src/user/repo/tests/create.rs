use sqlx::{PgPool, test};

use crate::{
    tests::helpers::create_user_model,
    user::repo::{PgUserRepository, UserRepository},
};

#[test]
async fn create_required(pool: PgPool) {
    let repo = PgUserRepository::init(pool.clone());

    let res = repo.create(create_user_model()).await;
    assert!(res.is_ok());
    if let Ok(user) = res {
        assert_eq!(user.display_name, "Test User");
    }
}
