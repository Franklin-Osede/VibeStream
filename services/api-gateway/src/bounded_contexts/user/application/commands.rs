use async_trait::async_trait;
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};

use crate::shared::application::command::{Command, CommandHandler};
use crate::shared::domain::errors::AppError;

use crate::bounded_contexts::user::domain::{Email, Username, User, UserRepository};

// ---------------- Command ----------------
#[derive(Debug)]
pub struct RegisterUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

impl Command for RegisterUser {}

// --------------- Result ------------------
#[derive(Debug)]
pub struct RegisterUserResult {
    pub user_id: Uuid,
}

// --------------- Handler -----------------
pub struct RegisterUserHandler<R: UserRepository> {
    pub repo: R,
}

#[async_trait]
impl<R> CommandHandler<RegisterUser> for RegisterUserHandler<R>
where
    R: UserRepository + Send + Sync,
{
    type Output = RegisterUserResult;

    async fn handle(&self, cmd: RegisterUser) -> Result<Self::Output, AppError> {
        // Validate
        let email_vo = Email::parse(&cmd.email)?;
        let username_vo = Username::parse(&cmd.username)?;

        // Check duplicates
        if self.repo.find_by_email(email_vo.as_str()).await?.is_some() {
            return Err(AppError::DomainRuleViolation("Email already in use".into()));
        }

        // Hash password
        let pwd_hash = hash(cmd.password, DEFAULT_COST).map_err(|e| AppError::Internal(e.to_string()))?;

        let (user, _event) = User::register(email_vo, username_vo, pwd_hash);
        self.repo.save(&user).await?;
        Ok(RegisterUserResult { user_id: user.id })
    }
}

// ---------------- Tests ------------------
#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;
    use crate::shared::domain::repositories::RepoResult;

    struct InMemoryUserRepo { data: Mutex<Vec<User>> }
    impl InMemoryUserRepo { fn new() -> Self { Self { data: Mutex::new(vec![]) }} }

    #[async_trait]
    impl UserRepository for InMemoryUserRepo {
        async fn find_by_id(&self, id: Uuid) -> RepoResult<Option<User>> {
            let data = self.data.lock().unwrap();
            Ok(data.iter().cloned().find(|u| u.id == id))
        }
        async fn find_by_email(&self, email: &str) -> RepoResult<Option<User>> {
            let data = self.data.lock().unwrap();
            Ok(data.iter().cloned().find(|u| u.email.as_str() == email))
        }
        async fn save(&self, user: &User) -> RepoResult<()> {
            self.data.lock().unwrap().push(user.clone());
            Ok(())
        }
    }

    #[tokio::test]
    async fn register_user_happy_path() {
        let repo = InMemoryUserRepo::new();
        let handler = RegisterUserHandler { repo };

        let cmd = RegisterUser {
            email: "test@example.com".into(),
            username: "tester".into(),
            password: "secret123".into(),
        };
        let res = handler.handle(cmd).await.unwrap();
        assert_ne!(res.user_id, Uuid::nil());
    }
} 