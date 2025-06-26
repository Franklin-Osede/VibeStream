use async_trait::async_trait;
use std::sync::Mutex;
use uuid::Uuid;

use crate::bounded_contexts::user::domain::{User, UserRepository};
use crate::shared::domain::repositories::RepoResult;

pub struct InMemoryUserRepository {
    data: Mutex<Vec<User>>, 
}

impl InMemoryUserRepository {
    pub fn new() -> Self { Self { data: Mutex::new(vec![]) } }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn find_by_id(&self, id: Uuid) -> RepoResult<Option<User>> {
        let data = self.data.lock().unwrap();
        Ok(data.iter().cloned().find(|u| u.id == id))
    }

    async fn find_by_email(&self, email: &str) -> RepoResult<Option<User>> {
        let data = self.data.lock().unwrap();
        Ok(data.iter().cloned().find(|u| u.email.as_str() == email))
    }

    async fn save(&self, user: &User) -> RepoResult<()> {
        let mut data = self.data.lock().unwrap();
        data.push(user.clone());
        Ok(())
    }
} 