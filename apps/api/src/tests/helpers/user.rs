use crate::user::types::{
    UserID,
    repo::{CreateModel, UpdateModel},
};

use super::get_today_date_pg;

impl Default for CreateModel {
    fn default() -> Self {
        Self {
            id: UserID::new_v4(),
            display_name: "Test User".to_string(),
            completed_task_retention: None,
            created_at: get_today_date_pg(),
        }
    }
}

impl Default for UpdateModel {
    fn default() -> Self {
        Self {
            display_name: Some("Updated User".to_string()),
            completed_task_retention: None,
        }
    }
}
