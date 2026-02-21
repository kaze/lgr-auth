use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        self.users
            .get(email)
            .cloned()
            .ok_or(UserStoreError::UserNotFound)
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email)?;
        if user.password != password {
            return Err(UserStoreError::InvalidCredentials);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new("a@example.com".to_owned(), "pass".to_owned(), false);

        assert_eq!(store.add_user(user.clone()), Ok(()));
        assert_eq!(store.add_user(user), Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new("b@example.com".to_owned(), "pass".to_owned(), true);
        store.add_user(user.clone()).unwrap();

        assert_eq!(store.get_user("b@example.com"), Ok(user));
        assert_eq!(
            store.get_user("missing@example.com"),
            Err(UserStoreError::UserNotFound)
        );
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new("c@example.com".to_owned(), "secret".to_owned(), false);
        store.add_user(user).unwrap();

        assert_eq!(store.validate_user("c@example.com", "secret"), Ok(()));
        assert_eq!(
            store.validate_user("c@example.com", "wrong"),
            Err(UserStoreError::InvalidCredentials)
        );
        assert_eq!(
            store.validate_user("missing@example.com", "secret"),
            Err(UserStoreError::UserNotFound)
        );
    }
}
