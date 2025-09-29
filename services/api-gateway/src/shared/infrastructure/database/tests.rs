#[cfg(test)]
mod integration_tests {
    use super::*;
    use sqlx::PgPool;
    use uuid::Uuid;
    use chrono::Utc;
    use crate::bounded_contexts::user::domain::{
        aggregates::UserAggregate,
        entities::{User, UserProfile, UserPreferences, UserStats},
        value_objects::{UserId, Email, Username, PasswordHash},
    };

    async fn setup_test_db() -> PgPool {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://vibestream:vibestream123@localhost:5432/vibestream_test".to_string());
        
        PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to test database")
    }

    #[tokio::test]
    async fn test_user_repository_crud_operations() {
        let pool = setup_test_db().await;
        let repo = PostgresUserRepository::new(Arc::new(pool));
        
        // Create test user
        let email = Email::new("test@example.com".to_string()).unwrap();
        let username = Username::new("testuser".to_string()).unwrap();
        let password_hash = PasswordHash::new("hashed_password".to_string());
        
        let user = User::new(email.clone(), username.clone(), password_hash);
        let profile = UserProfile::new(UserId::new());
        let preferences = UserPreferences::new(UserId::new());
        let stats = UserStats::new(UserId::new());
        
        let user_aggregate = UserAggregate::load(user, profile, preferences, stats, 1);
        
        // Test save
        repo.save(&user_aggregate).await.expect("Failed to save user");
        
        // Test find by email
        let found_user = repo.find_by_email(&email).await.expect("Failed to find user by email");
        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().user.email.value(), email.value());
        
        // Test find by username
        let found_user = repo.find_by_username(&username).await.expect("Failed to find user by username");
        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().user.username.value(), username.value());
        
        // Test email exists
        let exists = repo.email_exists(&email).await.expect("Failed to check email existence");
        assert!(exists);
        
        // Test username exists
        let exists = repo.username_exists(&username).await.expect("Failed to check username existence");
        assert!(exists);
        
        // Test count users
        let count = repo.count_users().await.expect("Failed to count users");
        assert!(count > 0);
    }

    #[tokio::test]
    async fn test_user_repository_search_operations() {
        let pool = setup_test_db().await;
        let repo = PostgresUserRepository::new(Arc::new(pool));
        
        // Test search users
        let users = repo.search_users(Some("test"), 10, 0).await.expect("Failed to search users");
        // Should return users matching the search term
        
        // Test find active users
        let active_users = repo.find_active_users(1, 10).await.expect("Failed to find active users");
        // Should return users active in the last 30 days
        
        // Test find by role
        let artist_users = repo.find_by_role("artist", 1, 10).await.expect("Failed to find users by role");
        // Should return users with artist role
        
        // Test find top users by rewards
        let top_users = repo.find_top_users_by_rewards(10).await.expect("Failed to find top users by rewards");
        // Should return users ordered by total rewards
        
        // Test find top users by listening time
        let top_listeners = repo.find_top_users_by_listening_time(10).await.expect("Failed to find top users by listening time");
        // Should return users ordered by total listening time
    }

    #[tokio::test]
    async fn test_user_repository_follow_operations() {
        let pool = setup_test_db().await;
        let repo = PostgresUserRepository::new(Arc::new(pool));
        
        let follower_id = UserId::new();
        let followee_id = UserId::new();
        
        // Test add follower
        repo.add_follower(&follower_id, &followee_id).await.expect("Failed to add follower");
        
        // Test remove follower
        repo.remove_follower(&follower_id, &followee_id).await.expect("Failed to remove follower");
    }

    #[tokio::test]
    async fn test_user_repository_date_range_operations() {
        let pool = setup_test_db().await;
        let repo = PostgresUserRepository::new(Arc::new(pool));
        
        let start_date = Utc::now() - chrono::Duration::days(30);
        let end_date = Utc::now();
        
        // Test find users registered between dates
        let users = repo.find_users_registered_between(start_date, end_date).await
            .expect("Failed to find users registered between dates");
        // Should return users registered in the specified date range
    }

    #[tokio::test]
    async fn test_user_repository_tier_operations() {
        let pool = setup_test_db().await;
        let repo = PostgresUserRepository::new(Arc::new(pool));
        
        // Test find by tier
        let tier_users = repo.find_by_tier("premium", 1, 10).await.expect("Failed to find users by tier");
        // Should return users with premium tier
        
        // Test find by tier points range
        let range_users = repo.find_users_by_tier_points_range(100, 500, 1, 10).await
            .expect("Failed to find users by tier points range");
        // Should return users with tier points in the specified range
    }

    #[tokio::test]
    async fn test_user_repository_wallet_operations() {
        let pool = setup_test_db().await;
        let repo = PostgresUserRepository::new(Arc::new(pool));
        
        // Test find users with wallets
        let wallet_users = repo.find_users_with_wallets(1, 10).await.expect("Failed to find users with wallets");
        // Should return users who have active wallets
    }

    #[tokio::test]
    async fn test_user_repository_stats_operations() {
        let pool = setup_test_db().await;
        let repo = PostgresUserRepository::new(Arc::new(pool));
        
        let user_id = UserId::new();
        
        // Test get user stats
        let stats = repo.get_user_stats(&user_id).await.expect("Failed to get user stats");
        // Should return user statistics or None if user doesn't exist
    }
}




