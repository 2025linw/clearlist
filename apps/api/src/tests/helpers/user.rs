use crate::user::repo::UpdateModel;

impl Default for UpdateModel {
    fn default() -> Self {
        Self {
            display_name: Some("Updated User".to_string()),
        }
    }
}
