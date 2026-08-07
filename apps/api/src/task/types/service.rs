use chrono_tz::Tz;

use crate::user::types::UserID;

#[derive(Debug, Clone, Copy)]
pub struct UserContext {
    pub id: UserID,
    pub tz: Tz,
}
