use crate::user::types::{UserID, repo::CreateModel, route::CreateRequest};

use super::get_today_date_pg;

impl Default for CreateRequest {
    fn default() -> Self {
        Self {
            id: UserID::new_v4(),
            display_name: String::new(),
            preferred_timezone: None,
            completed_task_retention: None,
            created_at: get_today_date_pg(),
        }
    }
}

impl Default for CreateModel {
    fn default() -> Self {
        Self {
            id: UserID::new_v4(),
            display_name: String::new(),
            preferred_timezone: None,
            completed_task_retention: None,
            created_at: get_today_date_pg(),
        }
    }
}
