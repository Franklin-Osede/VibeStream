// Listen Reward Bounded Context
//
// This module contains all components for the Listen Reward bounded context,
// responsible for managing listening sessions, reward calculations, and distributions.

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;

// Re-export main public interfaces
pub use application::{
    ListenRewardApplicationService,
    StartListeningCommand, CompleteListeningCommand, ProcessRewardsCommand,
    GetUserListeningHistoryQuery, GetArtistAnalyticsQuery,
    StartListeningResponse, CompleteListeningResponse, ProcessRewardsResponse,
    UserListeningHistory, ArtistAnalytics,
};

pub use presentation::controllers::{
    ListenRewardController, AnalyticsController,
    listen_reward_routes, analytics_routes,
};

pub use infrastructure::{
    ListenSessionRepository, RewardDistributionRepository, RewardAnalyticsRepository,
    EventPublisher, ZkProofVerificationService,
    ListenRewardConfig, ListenRewardBoundedContext,
};

pub use domain::{
    entities::{ListenSession, SessionStatus, SessionAnalytics},
    value_objects::{
        ListenSessionId, RewardAmount, ListenDuration, QualityScore, 
        ZkProofHash, RewardPoolId, RewardTier, ValidationPeriod,
    },
    events::{
        ListenSessionStarted, ListenSessionCompleted, RewardCalculated,
        RewardDistributed, ArtistRoyaltyPaid, ZkProofVerificationFailed,
        RewardPoolDepleted,
    },
    aggregates::{RewardDistribution},
}; 