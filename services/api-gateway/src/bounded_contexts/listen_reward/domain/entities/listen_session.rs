use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::listen_reward::domain::value_objects::{
    ListenSessionId, RewardAmount, ListenDuration, QualityScore, ZkProofHash, RewardTier
};
use crate::bounded_contexts::listen_reward::domain::events::{
    DomainEvent, ListenSessionStarted, ListenSessionCompleted, RewardCalculated, 
    ZkProofVerificationFailed
};
use crate::bounded_contexts::music::domain::value_objects::{SongId, ArtistId};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SessionStatus {
    Active,
    Completed,
    Verified,
    Rewarded,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenSession {
    id: ListenSessionId,
    user_id: Uuid,
    song_id: SongId,
    artist_id: ArtistId,
    user_tier: RewardTier,
    status: SessionStatus,
    listen_duration: Option<ListenDuration>,
    quality_score: Option<QualityScore>,
    zk_proof: Option<ZkProofHash>,
    base_reward: Option<RewardAmount>,
    final_reward: Option<RewardAmount>,
    started_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
    verified_at: Option<DateTime<Utc>>,
}

impl ListenSession {
    pub fn new(
        user_id: Uuid,
        song_id: SongId,
        artist_id: ArtistId,
        user_tier: RewardTier,
    ) -> (Self, Box<dyn DomainEvent>) {
        let session_id = ListenSessionId::new();
        let started_at = Utc::now();

        let session = Self {
            id: session_id.clone(),
            user_id,
            song_id: song_id.clone(),
            artist_id: artist_id.clone(),
            user_tier: user_tier.clone(),
            status: SessionStatus::Active,
            listen_duration: None,
            quality_score: None,
            zk_proof: None,
            base_reward: None,
            final_reward: None,
            started_at,
            completed_at: None,
            verified_at: None,
        };

        let event = Box::new(ListenSessionStarted::new(
            session_id,
            user_id,
            song_id,
            artist_id,
            user_tier,
        ));

        (session, event)
    }

    // Getters
    pub fn id(&self) -> &ListenSessionId {
        &self.id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn song_id(&self) -> &SongId {
        &self.song_id
    }

    pub fn artist_id(&self) -> &ArtistId {
        &self.artist_id
    }

    pub fn user_tier(&self) -> &RewardTier {
        &self.user_tier
    }

    pub fn status(&self) -> &SessionStatus {
        &self.status
    }

    pub fn listen_duration(&self) -> Option<&ListenDuration> {
        self.listen_duration.as_ref()
    }

    pub fn quality_score(&self) -> Option<&QualityScore> {
        self.quality_score.as_ref()
    }

    pub fn final_reward(&self) -> Option<&RewardAmount> {
        self.final_reward.as_ref()
    }

    pub fn started_at(&self) -> DateTime<Utc> {
        self.started_at
    }

    pub fn zk_proof(&self) -> Option<&ZkProofHash> {
        self.zk_proof.as_ref()
    }

    pub fn base_reward(&self) -> Option<&RewardAmount> {
        self.base_reward.as_ref()
    }

    pub fn completed_at(&self) -> Option<DateTime<Utc>> {
        self.completed_at
    }

    pub fn verified_at(&self) -> Option<DateTime<Utc>> {
        self.verified_at
    }

    // Métodos auxiliares para acceso a datos
    pub fn created_at(&self) -> DateTime<Utc> {
        self.started_at
    }

    pub fn version(&self) -> i32 {
        0 // Versión por defecto
    }

    // Business logic methods
    pub fn complete_session(
        &mut self,
        listen_duration: ListenDuration,
        quality_score: QualityScore,
        zk_proof: ZkProofHash,
        song_duration: u32,
    ) -> Result<Box<dyn DomainEvent>, String> {
        if self.status != SessionStatus::Active {
            return Err("Session is not active".to_string());
        }

        // Validate listen duration against song duration
        if !listen_duration.is_valid_for_reward(song_duration) {
            return Err("Listen duration is too short for reward eligibility".to_string());
        }

        // Validate session hasn't been running too long (anti-fraud)
        let session_duration = Utc::now() - self.started_at;
        if session_duration.num_seconds() > 7200 {
            return Err("Session has been running too long".to_string());
        }

        self.listen_duration = Some(listen_duration.clone());
        self.quality_score = Some(quality_score.clone());
        self.zk_proof = Some(zk_proof.clone());
        self.status = SessionStatus::Completed;
        self.completed_at = Some(Utc::now());

        Ok(Box::new(ListenSessionCompleted::new(
            self.id.clone(),
            self.user_id,
            self.song_id.clone(),
            listen_duration,
            quality_score,
            zk_proof,
        )))
    }

    pub fn verify_and_calculate_reward(
        &mut self,
        base_reward_rate: f64,
        is_zk_proof_valid: bool,
    ) -> Result<Box<dyn DomainEvent>, String> {
        if self.status != SessionStatus::Completed {
            return Err("Session must be completed before verification".to_string());
        }

        if !is_zk_proof_valid {
            self.status = SessionStatus::Failed;
            let zk_proof = self.zk_proof.as_ref().unwrap().clone();
            return Ok(Box::new(ZkProofVerificationFailed::new(
                self.id.clone(),
                self.user_id,
                self.song_id.clone(),
                zk_proof,
                "ZK proof verification failed".to_string(),
            )));
        }

        // Calculate base reward based on listen duration
        let listen_duration = self.listen_duration.as_ref().unwrap();
        let base_reward_tokens = (listen_duration.minutes() * base_reward_rate).min(100.0);
        let base_reward = RewardAmount::new(base_reward_tokens)
            .map_err(|e| format!("Invalid base reward: {}", e))?;

        // Apply quality multiplier
        let quality_score = self.quality_score.as_ref().unwrap();
        let quality_adjusted_reward = quality_score.multiply_reward(&base_reward)
            .map_err(|e| format!("Failed to apply quality multiplier: {}", e))?;

        // Apply tier multiplier
        let tier_multiplier = self.user_tier.multiplier();
        let final_reward_tokens = quality_adjusted_reward.tokens() * tier_multiplier;
        let final_reward = RewardAmount::new(final_reward_tokens)
            .map_err(|e| format!("Invalid final reward: {}", e))?;

        self.base_reward = Some(base_reward.clone());
        self.final_reward = Some(final_reward.clone());
        self.status = SessionStatus::Verified;
        self.verified_at = Some(Utc::now());

        Ok(Box::new(RewardCalculated::new(
            self.id.clone(),
            self.user_id,
            self.song_id.clone(),
            self.artist_id.clone(),
            base_reward,
            final_reward,
            self.user_tier.clone(),
            quality_score.score(),
        )))
    }

    pub fn mark_rewarded(&mut self) -> Result<(), String> {
        if self.status != SessionStatus::Verified {
            return Err("Session must be verified before marking as rewarded".to_string());
        }

        self.status = SessionStatus::Rewarded;
        Ok(())
    }

    pub fn can_be_rewarded(&self) -> bool {
        matches!(self.status, SessionStatus::Verified)
    }

    pub fn is_eligible_for_reward(&self, song_duration: u32) -> bool {
        if let Some(duration) = &self.listen_duration {
            duration.is_valid_for_reward(song_duration)
        } else {
            false
        }
    }

    pub fn get_session_analytics(&self) -> SessionAnalytics {
        SessionAnalytics {
            session_id: self.id.clone(),
            user_id: self.user_id,
            song_id: self.song_id.clone(),
            user_tier: self.user_tier.clone(),
            listen_duration_seconds: self.listen_duration.as_ref().map(|d| d.seconds()),
            quality_score: self.quality_score.as_ref().map(|q| q.score()),
            base_reward_tokens: self.base_reward.as_ref().map(|r| r.tokens()),
            final_reward_tokens: self.final_reward.as_ref().map(|r| r.tokens()),
            tier_multiplier: self.user_tier.multiplier(),
            session_duration_seconds: self.completed_at
                .map(|completed| (completed - self.started_at).num_seconds() as u32),
            status: self.status.clone(),
        }
    }

    pub fn from_parts(
        id: ListenSessionId,
        user_id: Uuid,
        song_id: SongId,
        artist_id: ArtistId,
        user_tier: RewardTier,
        status: SessionStatus,
        listen_duration: Option<ListenDuration>,
        quality_score: Option<QualityScore>,
        zk_proof: Option<ZkProofHash>,
        base_reward: Option<RewardAmount>,
        final_reward: Option<RewardAmount>,
        started_at: DateTime<Utc>,
        completed_at: Option<DateTime<Utc>>,
        verified_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            user_id,
            song_id,
            artist_id,
            user_tier,
            status,
            listen_duration,
            quality_score,
            zk_proof,
            base_reward,
            final_reward,
            started_at,
            completed_at,
            verified_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionAnalytics {
    pub session_id: ListenSessionId,
    pub user_id: Uuid,
    pub song_id: SongId,
    pub user_tier: RewardTier,
    pub listen_duration_seconds: Option<u32>,
    pub quality_score: Option<f64>,
    pub base_reward_tokens: Option<f64>,
    pub final_reward_tokens: Option<f64>,
    pub tier_multiplier: f64,
    pub session_duration_seconds: Option<u32>,
    pub status: SessionStatus,
}

impl SessionStatus {
    pub fn to_string(&self) -> String {
        match self {
            SessionStatus::Active => "active".to_string(),
            SessionStatus::Completed => "completed".to_string(),
            SessionStatus::Verified => "verified".to_string(),
            SessionStatus::Rewarded => "rewarded".to_string(),
            SessionStatus::Failed => "failed".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        match s {
            "active" => Ok(SessionStatus::Active),
            "completed" => Ok(SessionStatus::Completed),
            "verified" => Ok(SessionStatus::Verified),
            "rewarded" => Ok(SessionStatus::Rewarded),
            "failed" => Ok(SessionStatus::Failed),
            _ => Err(format!("Invalid session status: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_session() -> ListenSession {
        let (session, _) = ListenSession::new(
            Uuid::new_v4(),
            SongId::new(),
            ArtistId::new(),
            RewardTier::Basic,
        );
        session
    }

    #[test]
    fn test_session_creation() {
        let (session, event) = ListenSession::new(
            Uuid::new_v4(),
            SongId::new(),
            ArtistId::new(),
            RewardTier::Premium,
        );

        assert_eq!(session.status, SessionStatus::Active);
        assert_eq!(session.user_tier, RewardTier::Premium);
        assert_eq!(event.event_type(), "ListenSessionStarted");
    }

    #[test]
    fn test_complete_session_valid() {
        let mut session = create_test_session();
        let duration = ListenDuration::new(45).unwrap();
        let quality = QualityScore::new(0.8).unwrap();
        let proof = ZkProofHash::new("a".repeat(64)).unwrap();

        let result = session.complete_session(duration, quality, proof, 180);
        assert!(result.is_ok());
        assert_eq!(session.status, SessionStatus::Completed);
    }

    #[test]
    fn test_complete_session_invalid_duration() {
        let mut session = create_test_session();
        let duration = ListenDuration::new(10).unwrap();
        let quality = QualityScore::new(0.8).unwrap();
        let proof = ZkProofHash::new("a".repeat(64)).unwrap();

        let result = session.complete_session(duration, quality, proof, 180);
        assert!(result.is_err());
        assert_eq!(session.status, SessionStatus::Active);
    }

    #[test]
    fn test_verify_and_calculate_reward() {
        let mut session = create_test_session();
        
        // Complete session first
        let duration = ListenDuration::new(120).unwrap();
        let quality = QualityScore::new(0.9).unwrap();
        let proof = ZkProofHash::new("a".repeat(64)).unwrap();
        let _ = session.complete_session(duration, quality, proof, 180);

        // Verify and calculate reward
        let result = session.verify_and_calculate_reward(1.0, true);
        assert!(result.is_ok());
        assert_eq!(session.status, SessionStatus::Verified);
        assert!(session.final_reward.is_some());
    }

    #[test]
    fn test_verify_with_invalid_zk_proof() {
        let mut session = create_test_session();
        
        // Complete session first
        let duration = ListenDuration::new(120).unwrap();
        let quality = QualityScore::new(0.9).unwrap();
        let proof = ZkProofHash::new("a".repeat(64)).unwrap();
        let _ = session.complete_session(duration, quality, proof, 180);

        // Verify with invalid proof
        let result = session.verify_and_calculate_reward(1.0, false);
        assert!(result.is_ok());
        assert_eq!(session.status, SessionStatus::Failed);
    }

    #[test]
    fn test_reward_calculation_with_tiers() {
        let mut basic_session = create_test_session();
        let (mut premium_session, _) = ListenSession::new(
            Uuid::new_v4(),
            SongId::new(),
            ArtistId::new(),
            RewardTier::Premium,
        );

        // Complete both sessions with same parameters
        let duration = ListenDuration::new(120).unwrap(); // 2 minutes
        let quality = QualityScore::perfect();
        let proof = ZkProofHash::new("a".repeat(64)).unwrap();

        let _ = basic_session.complete_session(duration.clone(), quality.clone(), proof.clone(), 180);
        let _ = premium_session.complete_session(duration, quality, proof, 180);

        // Verify both
        let _ = basic_session.verify_and_calculate_reward(1.0, true);
        let _ = premium_session.verify_and_calculate_reward(1.0, true);

        // Premium should have 1.5x the reward of basic
        let basic_reward = basic_session.final_reward().unwrap().tokens();
        let premium_reward = premium_session.final_reward().unwrap().tokens();
        
        assert!((premium_reward / basic_reward - 1.5).abs() < 0.001);
    }
} 