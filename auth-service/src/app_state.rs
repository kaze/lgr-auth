use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::UserStore;

pub struct AppState<T: UserStore> {
    pub user_store: Arc<RwLock<T>>,
}

// Manual Clone impl because #[derive(Clone)] would add `T: Clone`,
// but Arc<RwLock<T>> is Clone regardless of whether T itself is Clone.
impl<T: UserStore> Clone for AppState<T> {
    fn clone(&self) -> Self {
        Self {
            user_store: self.user_store.clone(),
        }
    }
}

impl<T: UserStore> AppState<T> {
    pub fn new(user_store: Arc<RwLock<T>>) -> Self {
        Self { user_store }
    }
}
