use api_gateway::bounded_contexts::fan_ventures::domain::entities::{
    ArtistVenture, VentureCategory, RiskLevel, VentureStatus
};
use api_gateway::bounded_contexts::fan_ventures::infrastructure::postgres_repository::PostgresFanVenturesRepository;
// use api_gateway::testcontainers_setup::TestContainersSetup; // This doesn't work because it's in tests/
#[path = "testcontainers_setup.rs"]
mod testcontainers_setup;
use testcontainers_setup::TestContainersSetup;
use chrono::Utc;
use uuid::Uuid;
use sqlx::PgPool;

#[tokio::test]
async fn test_fan_ventures_repository_flow() {
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
        description: "A test venture created by integration test".to_string(),
        category: VentureCategory::MusicProduction,
        tags: vec!["pop".to_string(), "album".to_string()],
        risk_level: RiskLevel::Medium,
        expected_return: 15.5,
        artist_rating: 4.5,
        artist_previous_ventures: 0,
        artist_success_rate: 0.0,
        funding_goal: 10000.0,
        current_funding: 0.0,
        min_investment: 10.0,
        max_investment: 1000.0,
        status: VentureStatus::Draft,
        start_date: Utc::now(),
        end_date: Utc::now() + chrono::Duration::days(30),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        benefits: vec![], // Repository currently doesn't load/save this inline, handled separately
    };

    // 3. Test Create
    repo.create_venture(&new_venture).await.expect("Failed to create venture");

    // 4. Test Get
    let fetched_venture = repo.get_venture(venture_id).await.expect("Failed to fetch venture");
    assert!(fetched_venture.is_some(), "Venture should exist");
    
    let v = fetched_venture.unwrap();
    assert_eq!(v.id, venture_id);
    assert_eq!(v.title, "Test Venture Integration");
    assert_eq!(v.status, VentureStatus::Draft);
    // Verify JSON serialization of tags worked
    assert_eq!(v.tags, vec!["pop".to_string(), "album".to_string()]);

    // 5. Test List
    // We need to change status to Open to be listed by list_open_ventures
    // Since update_venture is a TODO, we can't test it yet via repo methods easily if it's not implemented.
    // However, create_venture has upsert logic "ON CONFLICT (id) DO UPDATE".
    // Let's try to "Update" by calling create again with modified status.
    
    let mut open_venture = new_venture.clone();
    open_venture.status = VentureStatus::Open;
    repo.create_venture(&open_venture).await.expect("Failed to update venture to Open");

    let open_ventures = repo.list_open_ventures(None).await.expect("Failed to list ventures");
    assert!(!open_ventures.is_empty(), "Should find open ventures");
    assert!(open_ventures.iter().any(|v| v.id == venture_id), "Our venture should be in the list");

    println!("Fan Ventures Integration Test Passed!");
}
