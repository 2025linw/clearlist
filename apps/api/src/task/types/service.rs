use chrono_tz::Tz;

use crate::user::types::UserID;

pub struct UserContext {
    pub user_id: UserID,
    pub user_tz: Tz,
}
