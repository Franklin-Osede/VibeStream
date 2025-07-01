use std::sync::Arc;
use chrono::{Duration, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use api_gateway::bounded_contexts::fractional_ownership::{
    // Integration
    FractionalOwnershipBoundedContext, FractionalOwnershipConfig,
    // Commands
    CreateOwnershipContract, ActivateOwnershipContract, PurchaseShares,
    TradeShares, DistributeRevenue, TerminateOwnershipContract,
    // Queries
    GetOwnershipContract, GetUserPortfolio, GetContractAnalytics,
    // Infrastructure
    PostgresOwnershipContractRepository, EventPublisher,
};
use api_gateway::shared::domain::errors::AppError;

/// Integration tests for the complete Fractional Ownership bounded context
/// 
/// These tests validate the entire system working together:
/// - Database operations
/// - Event publishing
/// - End-to-end workflows
/// - Cross-cutting concerns

#[cfg(test)]
mod integration_tests {
    use super::*;

    async fn setup_test_database() -> PgPool {
        // Create test database connection
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/vibestream_test".to_string());

        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to test database");

        // Run migrations
        sqlx::migrate!("../../../migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        // Clean up any existing test data
        cleanup_test_data(&pool).await;

        pool
    }

    async fn cleanup_test_data(pool: &PgPool) {
        let _ = sqlx::query("DELETE FROM shareholder_distributions").execute(pool).await;
        let _ = sqlx::query("DELETE FROM revenue_distributions").execute(pool).await;
        let _ = sqlx::query("DELETE FROM share_trading_history").execute(pool).await;
        let _ = sqlx::query("DELETE FROM fractional_shares").execute(pool).await;
        let _ = sqlx::query("DELETE FROM ownership_contracts").execute(pool).await;
        let _ = sqlx::query("DELETE FROM domain_events").execute(pool).await;
        let _ = sqlx::query("DELETE FROM event_outbox").execute(pool).await;
    }

    #[tokio::test]
    #[ignore] // Run with: cargo test --ignored
    async fn test_complete_ownership_contract_lifecycle() {
        let pool = setup_test_database().await;
        let mut context = FractionalOwnershipBoundedContext::initialize(pool.clone()).await.unwrap();

        // Start event processing
        context.start_event_processing().await.unwrap();

        let app_service = context.get_application_service();

        // Test data
        let song_id = Uuid::new_v4();
        let artist_id = Uuid::new_v4();
        let fan1_id = Uuid::new_v4();
        let fan2_id = Uuid::new_v4();

        // 1. CREATE CONTRACT
        println!("=== Testing Contract Creation ===");
        
        let create_command = CreateOwnershipContract {
            song_id,
            artist_id,
            total_shares: 1000,
            price_per_share: 10.0,
            artist_retained_percentage: 51.0,
            minimum_investment: Some(100.0),
            maximum_ownership_per_user: Some(20.0),
        };

        let create_result = app_service.create_ownership_contract(create_command).await.unwrap();
        let contract_id = create_result.contract_id;

        assert_eq!(create_result.shares_available_for_sale, 490); // 49% of 1000
        assert_eq!(create_result.total_market_cap, 10000.0);

        // Verify contract in database
        let db_contract = sqlx::query!("SELECT * FROM ownership_contracts WHERE id = $1", contract_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        assert_eq!(db_contract.total_shares, 1000);
        assert_eq!(db_contract.contract_status, "Draft");

        // 2. ACTIVATE CONTRACT
        println!("=== Testing Contract Activation ===");
        
        let activate_command = ActivateOwnershipContract { contract_id };
        let activate_result = app_service.activate_ownership_contract(activate_command).await.unwrap();

        assert_eq!(activate_result.contract_id, contract_id);

        // Verify activation in database
        let db_contract = sqlx::query!("SELECT contract_status FROM ownership_contracts WHERE id = $1", contract_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        assert_eq!(db_contract.contract_status, "Active");

        // 3. PURCHASE SHARES (Fan 1)
        println!("=== Testing Share Purchase (Fan 1) ===");
        
        let purchase_command1 = PurchaseShares {
            contract_id,
            buyer_id: fan1_id,
            ownership_percentage: 15.0,
            vesting_start_date: None,
            vesting_end_date: None,
        };

        let purchase_result1 = app_service.purchase_shares(purchase_command1).await.unwrap();
        
        assert_eq!(purchase_result1.ownership_percentage, 15.0);
        assert_eq!(purchase_result1.investment_amount, 1500.0); // 15% * $10 * 1000 shares
        assert!(!purchase_result1.events_triggered.is_empty());

        // Verify share in database
        let db_shares_count = sqlx::query!("SELECT COUNT(*) as count FROM fractional_shares WHERE contract_id = $1", contract_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        assert_eq!(db_shares_count.count.unwrap(), 1);

        // 4. PURCHASE SHARES (Fan 2) WITH VESTING
        println!("=== Testing Share Purchase with Vesting (Fan 2) ===");
        
        let vesting_start = Utc::now() + Duration::days(1);
        let vesting_end = vesting_start + Duration::days(365);
        
        let purchase_command2 = PurchaseShares {
            contract_id,
            buyer_id: fan2_id,
            ownership_percentage: 10.0,
            vesting_start_date: Some(vesting_start),
            vesting_end_date: Some(vesting_end),
        };

        let purchase_result2 = app_service.purchase_shares(purchase_command2).await.unwrap();
        
        assert_eq!(purchase_result2.ownership_percentage, 10.0);

        // Verify vesting in database
        let db_vesting_share = sqlx::query!(
            "SELECT vesting_start_date, vesting_end_date FROM fractional_shares WHERE owner_id = $1",
            fan2_id
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        
        assert!(db_vesting_share.vesting_start_date.is_some());
        assert!(db_vesting_share.vesting_end_date.is_some());

        // 5. QUERY CONTRACT DETAILS
        println!("=== Testing Contract Query ===");
        
        let get_contract_query = GetOwnershipContract { contract_id };
        let contract_details = app_service.get_ownership_contract(get_contract_query).await.unwrap();

        assert_eq!(contract_details.shares_sold, 250); // 15% + 10% of 1000 shares
        assert_eq!(contract_details.unique_shareholders, 2);
        assert_eq!(contract_details.completion_percentage, 25.0); // 250/1000 * 100

        // 6. QUERY USER PORTFOLIO
        println!("=== Testing Portfolio Query ===");
        
        let portfolio_query = GetUserPortfolio { user_id: fan1_id };
        let portfolio = app_service.get_user_portfolio(portfolio_query).await.unwrap();

        assert_eq!(portfolio.contracts_invested, 1);
        assert_eq!(portfolio.shares.len(), 1);
        assert_eq!(portfolio.shares[0].ownership_percentage, 15.0);
        assert_eq!(portfolio.total_portfolio_value, 1500.0);

        // 7. SHARE TRADING
        println!("=== Testing Share Trading ===");
        
        let share_id = purchase_result1.share_id;
        let trade_command = TradeShares {
            share_id,
            from_user_id: fan1_id,
            to_user_id: fan2_id,
            trade_price: 1600.0, // Premium price
        };

        let trade_result = app_service.trade_shares(trade_command).await.unwrap();
        
        assert_eq!(trade_result.trade_price, 1600.0);
        assert_eq!(trade_result.from_user_id, fan1_id);
        assert_eq!(trade_result.to_user_id, fan2_id);

        // Verify trade in database
        let db_trade = sqlx::query!("SELECT * FROM share_trading_history WHERE share_id = $1", share_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        assert_eq!(db_trade.trade_price, 1600.0.into());
        assert_eq!(db_trade.from_user_id, fan1_id);
        assert_eq!(db_trade.to_user_id, fan2_id);

        // 8. REVENUE DISTRIBUTION
        println!("=== Testing Revenue Distribution ===");
        
        let distribute_command = DistributeRevenue {
            contract_id,
            total_revenue: 1000.0,
            distribution_period_start: Utc::now() - Duration::days(30),
            distribution_period_end: Utc::now(),
            platform_fee_percentage: 5.0,
        };

        let distribution_result = app_service.distribute_revenue(distribute_command).await.unwrap();
        
        assert_eq!(distribution_result.total_revenue, 1000.0);
        assert!(distribution_result.total_distributed > 0.0);
        assert!(distribution_result.artist_share > 0.0);
        assert!(distribution_result.platform_fee > 0.0);
        assert_eq!(distribution_result.shareholder_count, 2);

        // Verify distribution in database
        let db_distribution = sqlx::query!("SELECT * FROM revenue_distributions WHERE contract_id = $1", contract_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        assert_eq!(db_distribution.total_revenue, 1000.0.into());

        // 9. CONTRACT ANALYTICS
        println!("=== Testing Contract Analytics ===");
        
        let analytics_query = GetContractAnalytics { contract_id };
        let analytics = app_service.get_contract_analytics(analytics_query).await.unwrap();

        assert_eq!(analytics.analytics.total_shares, 1000);
        assert_eq!(analytics.analytics.shares_sold, 250);
        assert_eq!(analytics.analytics.unique_shareholders, 2);
        assert_eq!(analytics.shareholder_breakdown.len(), 2);

        // 10. CONTRACT TERMINATION
        println!("=== Testing Contract Termination ===");
        
        let terminate_command = TerminateOwnershipContract {
            contract_id,
            terminated_by: artist_id,
            termination_reason: "artist_request".to_string(),
        };

        let termination_result = app_service.terminate_ownership_contract(terminate_command).await.unwrap();
        
        assert_eq!(termination_result.contract_id, contract_id);
        assert_eq!(termination_result.termination_reason, "artist_request");

        // Verify termination in database
        let db_contract = sqlx::query!("SELECT contract_status FROM ownership_contracts WHERE id = $1", contract_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        assert_eq!(db_contract.contract_status, "Terminated");

        println!("=== All Integration Tests Passed! ===");
        
        // Cleanup
        cleanup_test_data(&pool).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_event_sourcing_and_publishing() {
        let pool = setup_test_database().await;
        let mut context = FractionalOwnershipBoundedContext::initialize(pool.clone()).await.unwrap();

        context.start_event_processing().await.unwrap();
        let app_service = context.get_application_service();

        // Create a contract to generate events
        let create_command = CreateOwnershipContract {
            song_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            total_shares: 1000,
            price_per_share: 10.0,
            artist_retained_percentage: 51.0,
            minimum_investment: Some(100.0),
            maximum_ownership_per_user: Some(20.0),
        };

        let create_result = app_service.create_ownership_contract(create_command).await.unwrap();

        // Give events time to be processed
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Check that domain events were stored
        let domain_events = sqlx::query!("SELECT * FROM domain_events WHERE aggregate_id = $1", create_result.contract_id)
            .fetch_all(&pool)
            .await
            .unwrap();

        assert!(!domain_events.is_empty());
        
        // Check that events were published to outbox
        let outbox_events = sqlx::query!("SELECT * FROM event_outbox WHERE aggregate_id = $1", create_result.contract_id)
            .fetch_all(&pool)
            .await
            .unwrap();

        assert!(!outbox_events.is_empty());

        println!("Event sourcing test passed!");
        cleanup_test_data(&pool).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_business_rule_validations() {
        let pool = setup_test_database().await;
        let context = FractionalOwnershipBoundedContext::initialize(pool.clone()).await.unwrap();
        let app_service = context.get_application_service();

        // Test invalid artist retained percentage
        let invalid_command = CreateOwnershipContract {
            song_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            total_shares: 1000,
            price_per_share: 10.0,
            artist_retained_percentage: 101.0, // Invalid: > 100%
            minimum_investment: Some(100.0),
            maximum_ownership_per_user: Some(20.0),
        };

        let result = app_service.create_ownership_contract(invalid_command).await;
        assert!(result.is_err());

        // Test duplicate contract for same song
        let song_id = Uuid::new_v4();
        let valid_command = CreateOwnershipContract {
            song_id,
            artist_id: Uuid::new_v4(),
            total_shares: 1000,
            price_per_share: 10.0,
            artist_retained_percentage: 51.0,
            minimum_investment: Some(100.0),
            maximum_ownership_per_user: Some(20.0),
        };

        // First contract should succeed
        app_service.create_ownership_contract(valid_command.clone()).await.unwrap();

        // Second contract for same song should fail
        let duplicate_result = app_service.create_ownership_contract(valid_command).await;
        assert!(duplicate_result.is_err());

        println!("Business rule validation tests passed!");
        cleanup_test_data(&pool).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_concurrent_operations() {
        let pool = setup_test_database().await;
        let context = FractionalOwnershipBoundedContext::initialize(pool.clone()).await.unwrap();
        let app_service = context.get_application_service();

        // Create a contract
        let create_command = CreateOwnershipContract {
            song_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            total_shares: 1000,
            price_per_share: 10.0,
            artist_retained_percentage: 51.0,
            minimum_investment: Some(100.0),
            maximum_ownership_per_user: Some(20.0),
        };

        let create_result = app_service.create_ownership_contract(create_command).await.unwrap();
        let contract_id = create_result.contract_id;

        // Activate contract
        let activate_command = ActivateOwnershipContract { contract_id };
        app_service.activate_ownership_contract(activate_command).await.unwrap();

        // Simulate concurrent purchases
        let purchase_tasks: Vec<_> = (0..5)
            .map(|i| {
                let app_service = Arc::clone(&app_service);
                let user_id = Uuid::new_v4();
                
                tokio::spawn(async move {
                    let purchase_command = PurchaseShares {
                        contract_id,
                        buyer_id: user_id,
                        ownership_percentage: 5.0, // 5% each
                        vesting_start_date: None,
                        vesting_end_date: None,
                    };

                    app_service.purchase_shares(purchase_command).await
                })
            })
            .collect();

        // Wait for all purchases to complete
        let results: Vec<_> = futures::future::join_all(purchase_tasks).await;

        // Check results - some should succeed, some might fail due to insufficient shares
        let successful_purchases = results.iter().filter(|r| r.as_ref().unwrap().is_ok()).count();
        assert!(successful_purchases > 0);

        // Verify database consistency
        let db_contract = sqlx::query!("SELECT shares_sold FROM ownership_contracts WHERE id = $1", contract_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        let total_shares_sold = db_contract.shares_sold;
        let individual_shares = sqlx::query!("SELECT SUM(ownership_percentage * 10) as total FROM fractional_shares WHERE contract_id = $1", contract_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        // Database should be consistent
        assert_eq!(total_shares_sold, individual_shares.total.unwrap_or(0.0) as i32);

        println!("Concurrent operations test passed!");
        cleanup_test_data(&pool).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_health_check() {
        let pool = setup_test_database().await;
        let context = FractionalOwnershipBoundedContext::initialize(pool.clone()).await.unwrap();

        let health = context.health_check().await.unwrap();
        
        assert_eq!(health.name, "FractionalOwnership");
        assert_eq!(health.status, "Healthy");
        assert!(health.repository_status);
        assert!(health.event_publisher_status);

        cleanup_test_data(&pool).await;
    }
} 