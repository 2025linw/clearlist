use serde::Deserialize;

#[derive(Debug, Default, Clone)]
pub enum UpdateField<T> {
    #[default]
    Unchanged,
    Clear,
    Set(T),
}

impl<T> UpdateField<T> {
    pub fn is_none(&self) -> bool {
        return matches!(self, UpdateField::Unchanged);
    }

    pub fn as_mut(&mut self) -> UpdateField<&mut T> {
        match *self {
            UpdateField::Unchanged => UpdateField::Unchanged,
            UpdateField::Clear => UpdateField::Clear,
            UpdateField::Set(ref mut value) => UpdateField::Set(value),
        }
    }

    pub fn map<U, F>(self, f: F) -> UpdateField<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            UpdateField::Unchanged => UpdateField::Unchanged,
            UpdateField::Clear => UpdateField::Clear,
            UpdateField::Set(value) => UpdateField::Set(f(value)),
        }
    }
}

impl<T> From<UpdateField<T>> for Option<Option<T>> {
    fn from(value: UpdateField<T>) -> Self {
        match value {
            UpdateField::Unchanged => None,
            UpdateField::Clear => Some(None),
            UpdateField::Set(value) => Some(Some(value)),
        }
    }
}

impl<'de, T> Deserialize<'de> for UpdateField<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match Option::<T>::deserialize(deserializer)? {
            None => Self::Clear,
            Some(value) => Self::Set(value),
        })
    }
}
