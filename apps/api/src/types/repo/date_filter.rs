use sqlx::QueryBuilder;

#[derive(Debug, Clone)]
pub enum DateFilter<T> {
    Exists(bool),
    On(T),
    NotOn(T),
    StartRange(DateBound<T>),
    EndRange(DateBound<T>),
    Range(DateBound<T>, DateBound<T>),
}

impl<'a, T: 'a> DateFilter<T>
where
    T: Copy + sqlx::Type<sqlx::Postgres> + sqlx::Encode<'a, sqlx::Postgres>,
{
    pub fn add_to_builder(&self, builder: &mut QueryBuilder<'a, sqlx::Postgres>) {
        match self {
            DateFilter::Exists(exists) => {
                if *exists {
                    builder.push("start IS NOT NULL");
                } else {
                    builder.push("start IS NULL");
                }
            }
            DateFilter::On(dt) => {
                builder.push("start = ");
                builder.push_bind(*dt);
            }
            DateFilter::NotOn(dt) => {
                builder.push("start <> ");
                builder.push_bind(*dt);
            }
            DateFilter::StartRange(date_bound) => match date_bound {
                DateBound::Exclusive(dt) => {
                    builder.push("start > ");
                    builder.push_bind(*dt);
                }
                DateBound::Inclusive(dt) => {
                    builder.push("start >= ");
                    builder.push_bind(*dt);
                }
            },
            DateFilter::EndRange(date_bound) => match date_bound {
                DateBound::Exclusive(dt) => {
                    builder.push("start < ");
                    builder.push_bind(*dt);
                }
                DateBound::Inclusive(dt) => {
                    builder.push("start <= ");
                    builder.push_bind(*dt);
                }
            },
            DateFilter::Range(start_bound, end_bound) => {
                match start_bound {
                    DateBound::Exclusive(dt) => {
                        builder.push("start > ");
                        builder.push_bind(*dt);
                    }
                    DateBound::Inclusive(dt) => {
                        builder.push("start >= ");
                        builder.push_bind(*dt);
                    }
                }
                match end_bound {
                    DateBound::Exclusive(dt) => {
                        builder.push("start < ");
                        builder.push_bind(*dt);
                    }
                    DateBound::Inclusive(dt) => {
                        builder.push("start <= ");
                        builder.push_bind(*dt);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum DateBound<T> {
    Exclusive(T),
    Inclusive(T),
}

impl<T> DateBound<T> {
    pub fn map<U, F>(self, f: F) -> DateBound<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Exclusive(x) => DateBound::Exclusive(f(x)),
            Self::Inclusive(x) => DateBound::Inclusive(f(x)),
        }
    }
}
