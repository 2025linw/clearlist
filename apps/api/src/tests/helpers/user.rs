use crate::user::types::{UserID, repo::CreateModel, route::ProvisionRequest};

use super::get_today_date_pg;

impl Default for ProvisionRequest {
    fn default() -> Self {
        Self {
            id: UserID::new_random(),
            display_name: String::new(),
            created_at: get_today_date_pg(),
        }
    }
}

impl Default for CreateModel {
    fn default() -> Self {
        Self {
            id: UserID::new_random(),
            display_name: String::new(),
            created_at: get_today_date_pg(),
        }
    }
}
