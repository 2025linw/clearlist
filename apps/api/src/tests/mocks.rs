use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

pub mod tag;
pub mod task;
pub mod user;

pub type MockDB<K, V> = Arc<RwLock<HashMap<K, V>>>;
