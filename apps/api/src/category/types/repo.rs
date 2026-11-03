use sqlx::QueryBuilder;

#[derive(Debug, Clone)]
pub struct CreateModel {
    pub name: String,

    pub position_key: String,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateModel {
    pub name: Option<String>,

    pub position_key: Option<String>,
}

impl UpdateModel {
    pub fn add_to_builder(self, builder: &mut QueryBuilder<'_, sqlx::Postgres>) {
        let mut separated = builder.separated(", ");
        if let Some(name) = self.name {
            separated.push("name = ");
            separated.push_bind_unseparated(name);
        }

        if let Some(position_key) = self.position_key {
            separated.push("position_key = ");
            separated.push_bind_unseparated(position_key);
        }
    }
}
