use std::collections::HashMap;

use crate::domain::{Email, Password, User, UserStore, UserStoreError};

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        self.users
            .get(email)
            .cloned()
            .ok_or(UserStoreError::UserNotFound)
    }

    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError> {
        let user = self.get_user(email).await?;
        if user.password.as_ref() != password.as_ref() {
            return Err(UserStoreError::InvalidCredentials);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Email, Password, UserStore};

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new(
            Email::parse("a@example.com".to_owned()).unwrap(),
            Password::parse("password".to_owned()).unwrap(),
            false,
        );

        assert_eq!(store.add_user(user.clone()).await, Ok(()));
        assert_eq!(store.add_user(user).await, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let email = Email::parse("b@example.com".to_owned()).unwrap();
        let user = User::new(email.clone(), Password::parse("password".to_owned()).unwrap(), true);
        store.add_user(user.clone()).await.unwrap();

        assert_eq!(store.get_user(&email).await, Ok(user));
        assert_eq!(
            store.get_user(&Email::parse("missing@example.com".to_owned()).unwrap()).await,
            Err(UserStoreError::UserNotFound)
        );
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let email = Email::parse("c@example.com".to_owned()).unwrap();
        let password = Password::parse("secret12".to_owned()).unwrap();
        let user = User::new(email.clone(), password.clone(), false);
        store.add_user(user).await.unwrap();

        assert_eq!(store.validate_user(&email, &password).await, Ok(()));
        assert_eq!(
            store.validate_user(&email, &Password::parse("wrongpwd".to_owned()).unwrap()).await,
            Err(UserStoreError::InvalidCredentials)
        );
        assert_eq!(
            store
                .validate_user(&Email::parse("missing@example.com".to_owned()).unwrap(), &password)
                .await,
            Err(UserStoreError::UserNotFound)
        );
    }
}
