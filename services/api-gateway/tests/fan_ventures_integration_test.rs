use api_gateway::bounded_contexts::fan_ventures::domain::entities::{
    ArtistVenture, FanInvestment, VentureCategory, RiskLevel, VentureStatus, InvestmentType, InvestmentStatus
};
use api_gateway::bounded_contexts::fan_ventures::infrastructure::postgres_repository::PostgresFanVenturesRepository;
#[path = "testcontainers_setup.rs"]
mod testcontainers_setup;
use testcontainers_setup::TestContainersSetup;
use chrono::Utc;
use uuid::Uuid;
use sqlx::PgPool;

/// Test completo del flujo de Fan Ventures
#[tokio::test]
async fn test_fan_ventures_complete_flow() {
    // 1. Setup TestContainers (Postgres + Redis)
    let setup = TestContainersSetup::new();
    setup.setup_env();
    
    // Wait for DB and run migrations
    setup.wait_for_postgres().await.expect("Postgres failed to start");
    setup.run_migrations().await.expect("Migrations failed");

    let pool = PgPool::connect(&setup.get_postgres_url()).await.expect("Failed to connect to pool");
    let repo = PostgresFanVenturesRepository::new(pool);

    // 2. Create a Venture Entity
    let venture_id = Uuid::new_v4();
    let artist_id = Uuid::new_v4();
    
    let new_venture = ArtistVenture {
        id: venture_id,
        artist_id,
        title: "Test Venture Integration".to_string(),
        description: Some("A test venture created by integration test".to_string()),
        category: VentureCategory::Music,
        tags: vec!["pop".to_string(), "album".to_string()],
        risk_level: RiskLevel::Medium,
        expected_return: 15.5,
        artist_rating: 4.5,
        artist_previous_ventures: 0,
        artist_success_rate: 0.0,
        funding_goal: 10000.0,
        current_funding: 0.0,
        min_investment: 10.0,
        max_investment: Some(1000.0),
        status: VentureStatus::Draft,
        start_date: None,
        end_date: Some(Utc::now() + chrono::Duration::days(30)),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        benefits: vec![],
    };

    // 3. Test Create Venture
    repo.create_venture(&new_venture).await.expect("Failed to create venture");

    // 4. Test Get Venture
    let fetched_venture = repo.get_venture(venture_id).await.expect("Failed to fetch venture");
    assert!(fetched_venture.is_some(), "Venture should exist");
    
    let v = fetched_venture.unwrap();
    assert_eq!(v.id, venture_id);
    assert_eq!(v.title, "Test Venture Integration");
    assert_eq!(v.status, VentureStatus::Draft);
    assert_eq!(v.tags, vec!["pop".to_string(), "album".to_string()]);

    // 5. Test Get Ventures by Category
    let music_ventures = repo.get_ventures_by_category("music").await.expect("Failed to get ventures by category");
    assert!(music_ventures.iter().any(|v| v.id == venture_id), "Should find our venture by category");

    // 6. Test Get Ventures by Status
    let draft_ventures = repo.get_ventures_by_status("draft").await.expect("Failed to get ventures by status");
    assert!(draft_ventures.iter().any(|v| v.id == venture_id), "Should find our venture by status");

    // 7. Test Search Ventures
    let search_results = repo.search_ventures("Test", Some(10)).await.expect("Failed to search ventures");
    assert!(search_results.iter().any(|v| v.id == venture_id), "Should find our venture by search");

    // 8. Test Update Venture to Open
    let mut open_venture = new_venture.clone();
    open_venture.status = VentureStatus::Open;
    repo.create_venture(&open_venture).await.expect("Failed to update venture to Open");

    let open_ventures = repo.list_open_ventures(None).await.expect("Failed to list ventures");
    assert!(open_ventures.iter().any(|v| v.id == venture_id), "Our venture should be in open list");

    // 9. Test Create Investment
    let fan_id = Uuid::new_v4();
    let investment = FanInvestment {
        id: Uuid::new_v4(),
        fan_id,
        venture_id,
        investment_amount: 500.0,
        investment_type: InvestmentType::RevenueShare,
        status: InvestmentStatus::Pending,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    repo.create_fan_investment(&investment).await.expect("Failed to create investment");

    // 10. Test Get Fan Investments
    let fan_investments = repo.get_fan_investments(fan_id).await.expect("Failed to get fan investments");
    assert_eq!(fan_investments.len(), 1, "Should have one investment");
    assert_eq!(fan_investments[0].investment_amount, 500.0);

    // 11. Test Get Investments by Venture
    let venture_investments = repo.get_fan_investments_by_venture(venture_id).await.expect("Failed to get investments by venture");
    assert_eq!(venture_investments.len(), 1, "Should have one investment for this venture");

    // 12. Test Get Venture Revenue
    // First, update investment status to confirmed
    let mut confirmed_investment = investment.clone();
    confirmed_investment.status = InvestmentStatus::Active;
    repo.create_fan_investment(&confirmed_investment).await.expect("Failed to update investment");

    let revenue = repo.get_venture_revenue(venture_id).await.expect("Failed to get revenue");
    assert_eq!(revenue, 500.0, "Revenue should be 500.0");

    // 13. Test Get Venture Count
    let count = repo.get_venture_count().await.expect("Failed to get venture count");
    assert!(count >= 1, "Should have at least one venture");

    println!("✅ Fan Ventures Complete Integration Test Passed!");
}

/// Test de búsqueda y filtros
#[tokio::test]
async fn test_venture_search_and_filters() {
    let setup = TestContainersSetup::new();
    setup.setup_env();
    setup.wait_for_postgres().await.expect("Postgres failed to start");
    setup.run_migrations().await.expect("Migrations failed");

    let pool = PgPool::connect(&setup.get_postgres_url()).await.expect("Failed to connect");
    let repo = PostgresFanVenturesRepository::new(pool);

    let artist_id = Uuid::new_v4();

    // Create multiple ventures with different categories
    let venture1 = ArtistVenture {
        id: Uuid::new_v4(),
        artist_id,
        title: "Music Album Project".to_string(),
        description: Some("A music production venture".to_string()),
        category: VentureCategory::Music,
        tags: vec!["pop".to_string()],
        risk_level: RiskLevel::Low,
        expected_return: 10.0,
        artist_rating: 4.0,
        artist_previous_ventures: 2,
        artist_success_rate: 80.0,
        funding_goal: 5000.0,
        current_funding: 0.0,
        min_investment: 25.0,
        max_investment: Some(500.0),
        status: VentureStatus::Open,
        start_date: None,
        end_date: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        benefits: vec![],
    };

    let venture2 = ArtistVenture {
        id: Uuid::new_v4(),
        artist_id,
        title: "Film Production".to_string(),
        description: Some("A film venture".to_string()),
        category: VentureCategory::Film,
        tags: vec!["indie".to_string()],
        risk_level: RiskLevel::High,
        expected_return: 20.0,
        artist_rating: 3.5,
        artist_previous_ventures: 1,
        artist_success_rate: 50.0,
        funding_goal: 20000.0,
        current_funding: 0.0,
        min_investment: 100.0,
        max_investment: None,
        status: VentureStatus::Open,
        start_date: None,
        end_date: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        benefits: vec![],
    };

    repo.create_venture(&venture1).await.expect("Failed to create venture1");
    repo.create_venture(&venture2).await.expect("Failed to create venture2");

    // Test category filter
    let music_ventures = repo.get_ventures_by_category("music").await.expect("Failed to get by category");
    assert!(music_ventures.iter().any(|v| v.id == venture1.id), "Should find music venture");
    assert!(!music_ventures.iter().any(|v| v.id == venture2.id), "Should not find film venture");

    // Test status filter
    let open_ventures = repo.get_ventures_by_status("open").await.expect("Failed to get by status");
    assert_eq!(open_ventures.len(), 2, "Should find both open ventures");

    // Test search
    let search_results = repo.search_ventures("Music", Some(10)).await.expect("Failed to search");
    assert!(search_results.iter().any(|v| v.id == venture1.id), "Should find music venture by search");

    println!("✅ Venture Search and Filters Test Passed!");
}

/// Test de inversiones y portfolio
#[tokio::test]
async fn test_investments_and_portfolio() {
    let setup = TestContainersSetup::new();
    setup.setup_env();
    setup.wait_for_postgres().await.expect("Postgres failed to start");
    setup.run_migrations().await.expect("Migrations failed");

    let pool = PgPool::connect(&setup.get_postgres_url()).await.expect("Failed to connect");
    let repo = PostgresFanVenturesRepository::new(pool);

    let artist_id = Uuid::new_v4();
    let fan_id = Uuid::new_v4();

    // Create venture
    let venture = ArtistVenture {
        id: Uuid::new_v4(),
        artist_id,
        title: "Portfolio Test Venture".to_string(),
        description: Some("Test venture for portfolio".to_string()),
        category: VentureCategory::Music,
        tags: vec![],
        risk_level: RiskLevel::Medium,
        expected_return: 12.0,
        artist_rating: 4.0,
        artist_previous_ventures: 0,
        artist_success_rate: 0.0,
        funding_goal: 10000.0,
        current_funding: 0.0,
        min_investment: 50.0,
        max_investment: Some(1000.0),
        status: VentureStatus::Open,
        start_date: None,
        end_date: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        benefits: vec![],
    };

    repo.create_venture(&venture).await.expect("Failed to create venture");

    // Create multiple investments
    let investment1 = FanInvestment {
        id: Uuid::new_v4(),
        fan_id,
        venture_id: venture.id,
        investment_amount: 200.0,
        investment_type: InvestmentType::RevenueShare,
        status: InvestmentStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let investment2 = FanInvestment {
        id: Uuid::new_v4(),
        fan_id,
        venture_id: venture.id,
        investment_amount: 300.0,
        investment_type: InvestmentType::ExclusiveContent,
        status: InvestmentStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    repo.create_fan_investment(&investment1).await.expect("Failed to create investment1");
    repo.create_fan_investment(&investment2).await.expect("Failed to create investment2");

    // Test get fan investments
    let investments = repo.get_fan_investments(fan_id).await.expect("Failed to get investments");
    assert_eq!(investments.len(), 2, "Should have 2 investments");
    
    let total_invested: f64 = investments.iter().map(|i| i.investment_amount).sum();
    assert_eq!(total_invested, 500.0, "Total invested should be 500.0");

    // Test get investments by venture
    let venture_investments = repo.get_fan_investments_by_venture(venture.id).await.expect("Failed to get by venture");
    assert_eq!(venture_investments.len(), 2, "Should have 2 investments for this venture");

    // Test revenue calculation
    let revenue = repo.get_venture_revenue(venture.id).await.expect("Failed to get revenue");
    assert_eq!(revenue, 500.0, "Revenue should be 500.0");

    println!("✅ Investments and Portfolio Test Passed!");
}
