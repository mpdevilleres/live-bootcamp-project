use std::collections::HashMap;
use crate::domain::data_stores::{UserStore, UserStoreError};
use crate::domain::User;

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<String, User>,
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

    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if user.password.eq(password) {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let email = "sample@domain.com".to_string();
        let password = "p@$$w0rd".to_string();
        let user = User::new(
            email.clone(),
            password.clone(),
            false,
        );
        let result = store.add_user(user.clone()).await;
        assert!(result.is_ok());
        let new_user = store.users.get(&email);
        assert!(new_user.is_some());
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let email = "sample@domain.com".to_string();
        let password = "p@$$w0rd".to_string();
        // Why using new and using add_user syntax didn't work on this code
        // error[E0308]: mismatched types
        //   --> src/services/hashmap_user_store.rs:73:28
        //    |
        // 73 |         assert_eq!(result, Ok(user));
        //    |                            ^^^^^^^^ expected `Result<User, UserStoreError>`, found `Result<Pin<Box<...>>, ...>`

        // previous implementation
        // let user = User::new(
        //     email.clone(),
        //     password.clone(),
        //     false,
        // );
        // let _ = store.add_user(user.clone());

        let user = User {
            email: email.clone(),
            password: password.clone(),
            requires_2fa: false,
        };
        store.users.insert(email.clone(), user.clone());

        let result = store.get_user(&email).await;
        assert_eq!(result, Ok(user));
        let result = store.get_user("wrong@domain.com").await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let email = "sample@domain.com".to_string();
        let password = "p@$$w0rd".to_string();
        let user = User {
            email: email.clone(),
            password: password.clone(),
            requires_2fa: false,
        };
        store.users.insert(email.clone(), user.clone());

        let result = store.validate_user(&email, &password).await;
        assert_eq!(result, Ok(()));
        let result = store.validate_user(&email, "wrong password").await;
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));
    }
}
