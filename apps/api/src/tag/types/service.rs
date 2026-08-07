use crate::user::types::UserID;

#[derive(Debug, Clone, Copy)]
pub struct UserContext {
    pub id: UserID,
}
