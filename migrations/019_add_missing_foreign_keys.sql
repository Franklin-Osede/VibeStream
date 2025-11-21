-- Migration: 019_add_missing_foreign_keys.sql
-- Description: Agregar todas las foreign keys faltantes para garantizar integridad referencial
-- Author: AI Assistant
-- Created: Diciembre 2024

-- =====================================================
-- PAYMENTS TABLE - Foreign Keys
-- =====================================================

-- Payer y Payee deben referenciar users
ALTER TABLE payments 
    ADD CONSTRAINT fk_payments_payer_id 
    FOREIGN KEY (payer_id) REFERENCES users(id) ON DELETE SET NULL;

ALTER TABLE payments 
    ADD CONSTRAINT fk_payments_payee_id 
    FOREIGN KEY (payee_id) REFERENCES users(id) ON DELETE SET NULL;

-- Transaction ID opcional
ALTER TABLE payments 
    ADD CONSTRAINT fk_payments_transaction_id 
    FOREIGN KEY (transaction_id) REFERENCES transactions(id) ON DELETE SET NULL;

-- =====================================================
-- ROYALTY DISTRIBUTIONS - Foreign Keys
-- =====================================================

ALTER TABLE royalty_distributions 
    ADD CONSTRAINT fk_royalty_distributions_song_id 
    FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE RESTRICT;

ALTER TABLE royalty_distributions 
    ADD CONSTRAINT fk_royalty_distributions_artist_id 
    FOREIGN KEY (artist_id) REFERENCES users(id) ON DELETE RESTRICT;

-- =====================================================
-- REVENUE SHARING DISTRIBUTIONS - Foreign Keys
-- =====================================================

ALTER TABLE revenue_sharing_distributions 
    ADD CONSTRAINT fk_revenue_sharing_contract_id 
    FOREIGN KEY (contract_id) REFERENCES ownership_contracts(id) ON DELETE RESTRICT;

ALTER TABLE revenue_sharing_distributions 
    ADD CONSTRAINT fk_revenue_sharing_song_id 
    FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE RESTRICT;

-- =====================================================
-- SHAREHOLDER DISTRIBUTIONS - Foreign Keys
-- =====================================================

ALTER TABLE shareholder_distributions 
    ADD CONSTRAINT fk_shareholder_distributions_shareholder_id 
    FOREIGN KEY (shareholder_id) REFERENCES users(id) ON DELETE RESTRICT;

ALTER TABLE shareholder_distributions 
    ADD CONSTRAINT fk_shareholder_distributions_payment_id 
    FOREIGN KEY (payment_id) REFERENCES payments(id) ON DELETE SET NULL;

-- =====================================================
-- PAYMENT BATCH ITEMS - Foreign Keys
-- =====================================================

ALTER TABLE payment_batch_items 
    ADD CONSTRAINT fk_payment_batch_items_payment_id 
    FOREIGN KEY (payment_id) REFERENCES payments(id) ON DELETE RESTRICT;

-- =====================================================
-- FRAUD ALERTS - Foreign Keys
-- =====================================================

ALTER TABLE fraud_alerts 
    ADD CONSTRAINT fk_fraud_alerts_payment_id 
    FOREIGN KEY (payment_id) REFERENCES payments(id) ON DELETE RESTRICT;

ALTER TABLE fraud_alerts 
    ADD CONSTRAINT fk_fraud_alerts_user_id 
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT;

ALTER TABLE fraud_alerts 
    ADD CONSTRAINT fk_fraud_alerts_reviewed_by 
    FOREIGN KEY (reviewed_by) REFERENCES users(id) ON DELETE SET NULL;

-- =====================================================
-- OWNERSHIP CONTRACTS - Foreign Keys
-- =====================================================

ALTER TABLE ownership_contracts 
    ADD CONSTRAINT fk_ownership_contracts_artist_id 
    FOREIGN KEY (artist_id) REFERENCES users(id) ON DELETE RESTRICT;

-- =====================================================
-- SHARE TRANSACTIONS - Foreign Keys
-- =====================================================

ALTER TABLE share_transactions 
    ADD CONSTRAINT fk_share_transactions_contract_id 
    FOREIGN KEY (contract_id) REFERENCES ownership_contracts(id) ON DELETE RESTRICT;

ALTER TABLE share_transactions 
    ADD CONSTRAINT fk_share_transactions_buyer_id 
    FOREIGN KEY (buyer_id) REFERENCES users(id) ON DELETE SET NULL;

ALTER TABLE share_transactions 
    ADD CONSTRAINT fk_share_transactions_seller_id 
    FOREIGN KEY (seller_id) REFERENCES users(id) ON DELETE SET NULL;

-- =====================================================
-- REVENUE DISTRIBUTIONS - Foreign Keys
-- =====================================================

ALTER TABLE revenue_distributions 
    ADD CONSTRAINT fk_revenue_distributions_contract_id 
    FOREIGN KEY (contract_id) REFERENCES ownership_contracts(id) ON DELETE RESTRICT;

-- =====================================================
-- CAMPAIGNS - Foreign Keys
-- =====================================================

ALTER TABLE campaigns 
    ADD CONSTRAINT fk_campaigns_song_id 
    FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE RESTRICT;

ALTER TABLE campaigns 
    ADD CONSTRAINT fk_campaigns_artist_id 
    FOREIGN KEY (artist_id) REFERENCES users(id) ON DELETE RESTRICT;

-- =====================================================
-- NFT PURCHASES - Foreign Keys
-- =====================================================

ALTER TABLE nft_purchases 
    ADD CONSTRAINT fk_nft_purchases_campaign_id 
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE RESTRICT;

ALTER TABLE nft_purchases 
    ADD CONSTRAINT fk_nft_purchases_user_id 
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT;

-- =====================================================
-- CAMPAIGN ANALYTICS - Foreign Keys
-- =====================================================

ALTER TABLE campaign_analytics 
    ADD CONSTRAINT fk_campaign_analytics_campaign_id 
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE;

-- =====================================================
-- LISTEN SESSIONS - Foreign Keys
-- =====================================================

ALTER TABLE listen_sessions 
    ADD CONSTRAINT fk_listen_sessions_user_id 
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT;

ALTER TABLE listen_sessions 
    ADD CONSTRAINT fk_listen_sessions_song_id 
    FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE CASCADE;

ALTER TABLE listen_sessions 
    ADD CONSTRAINT fk_listen_sessions_artist_id 
    FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE RESTRICT;

-- =====================================================
-- USER REWARD HISTORY - Foreign Keys
-- =====================================================

ALTER TABLE user_reward_history 
    ADD CONSTRAINT fk_user_reward_history_user_id 
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT;

ALTER TABLE user_reward_history 
    ADD CONSTRAINT fk_user_reward_history_session_id 
    FOREIGN KEY (session_id) REFERENCES listen_sessions(id) ON DELETE SET NULL;

ALTER TABLE user_reward_history 
    ADD CONSTRAINT fk_user_reward_history_distribution_id 
    FOREIGN KEY (distribution_id) REFERENCES reward_distributions(id) ON DELETE SET NULL;

-- =====================================================
-- NOTIFICATIONS - Foreign Keys
-- =====================================================

ALTER TABLE notifications 
    ADD CONSTRAINT fk_notifications_user_id 
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT;

-- =====================================================
-- FAN VERIFICATIONS - Foreign Keys
-- =====================================================

ALTER TABLE fan_verifications 
    ADD CONSTRAINT fk_fan_verifications_fan_id 
    FOREIGN KEY (fan_id) REFERENCES users(id) ON DELETE RESTRICT;

-- =====================================================
-- NFT WRISTBANDS - Foreign Keys
-- =====================================================

ALTER TABLE nft_wristbands 
    ADD CONSTRAINT fk_nft_wristbands_fan_id 
    FOREIGN KEY (fan_id) REFERENCES users(id) ON DELETE RESTRICT;

-- =====================================================
-- ÍNDICES ADICIONALES PARA FOREIGN KEYS
-- =====================================================

-- Índices para mejorar performance de joins
CREATE INDEX IF NOT EXISTS idx_payments_payer_id ON payments(payer_id);
CREATE INDEX IF NOT EXISTS idx_payments_payee_id ON payments(payee_id);
CREATE INDEX IF NOT EXISTS idx_payments_transaction_id ON payments(transaction_id) WHERE transaction_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_royalty_distributions_song_id ON royalty_distributions(song_id);
CREATE INDEX IF NOT EXISTS idx_royalty_distributions_artist_id ON royalty_distributions(artist_id);

CREATE INDEX IF NOT EXISTS idx_revenue_sharing_contract_id ON revenue_sharing_distributions(contract_id);
CREATE INDEX IF NOT EXISTS idx_revenue_sharing_song_id ON revenue_sharing_distributions(song_id);

CREATE INDEX IF NOT EXISTS idx_shareholder_distributions_shareholder_id ON shareholder_distributions(shareholder_id);
CREATE INDEX IF NOT EXISTS idx_shareholder_distributions_payment_id ON shareholder_distributions(payment_id) WHERE payment_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_payment_batch_items_payment_id ON payment_batch_items(payment_id);

CREATE INDEX IF NOT EXISTS idx_fraud_alerts_payment_id ON fraud_alerts(payment_id);
CREATE INDEX IF NOT EXISTS idx_fraud_alerts_user_id ON fraud_alerts(user_id);
CREATE INDEX IF NOT EXISTS idx_fraud_alerts_reviewed_by ON fraud_alerts(reviewed_by) WHERE reviewed_by IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_ownership_contracts_artist_id ON ownership_contracts(artist_id);

CREATE INDEX IF NOT EXISTS idx_share_transactions_contract_id ON share_transactions(contract_id);
CREATE INDEX IF NOT EXISTS idx_share_transactions_buyer_id ON share_transactions(buyer_id) WHERE buyer_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_share_transactions_seller_id ON share_transactions(seller_id) WHERE seller_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_revenue_distributions_contract_id ON revenue_distributions(contract_id);

CREATE INDEX IF NOT EXISTS idx_campaigns_song_id ON campaigns(song_id);
CREATE INDEX IF NOT EXISTS idx_campaigns_artist_id ON campaigns(artist_id);

CREATE INDEX IF NOT EXISTS idx_nft_purchases_campaign_id ON nft_purchases(campaign_id);
CREATE INDEX IF NOT EXISTS idx_nft_purchases_user_id ON nft_purchases(user_id);

CREATE INDEX IF NOT EXISTS idx_campaign_analytics_campaign_id ON campaign_analytics(campaign_id);

CREATE INDEX IF NOT EXISTS idx_listen_sessions_user_id ON listen_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_listen_sessions_song_id ON listen_sessions(song_id);
CREATE INDEX IF NOT EXISTS idx_listen_sessions_artist_id ON listen_sessions(artist_id);

CREATE INDEX IF NOT EXISTS idx_user_reward_history_user_id ON user_reward_history(user_id);
CREATE INDEX IF NOT EXISTS idx_user_reward_history_session_id ON user_reward_history(session_id) WHERE session_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_user_reward_history_distribution_id ON user_reward_history(distribution_id) WHERE distribution_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_notifications_user_id ON notifications(user_id);

CREATE INDEX IF NOT EXISTS idx_fan_verifications_fan_id ON fan_verifications(fan_id);

CREATE INDEX IF NOT EXISTS idx_nft_wristbands_fan_id ON nft_wristbands(fan_id);

-- =====================================================
-- VALIDACIÓN
-- =====================================================

-- Verificar que todas las foreign keys se crearon correctamente
DO $$
DECLARE
    fk_count INTEGER;
    expected_fk_count INTEGER := 35; -- Número esperado de foreign keys
BEGIN
    SELECT COUNT(*) INTO fk_count
    FROM information_schema.table_constraints
    WHERE constraint_type = 'FOREIGN KEY'
    AND table_schema = 'public';
    
    IF fk_count < expected_fk_count THEN
        RAISE WARNING 'Se esperaban al menos % foreign keys, se encontraron %', expected_fk_count, fk_count;
    ELSE
        RAISE NOTICE 'Migration completada: % foreign keys creadas', fk_count;
    END IF;
END $$;

COMMENT ON CONSTRAINT fk_payments_payer_id ON payments IS 'Usuario que realiza el pago';
COMMENT ON CONSTRAINT fk_payments_payee_id ON payments IS 'Usuario que recibe el pago';
COMMENT ON CONSTRAINT fk_payments_transaction_id ON payments IS 'Transacción blockchain relacionada (opcional)';

COMMENT ON CONSTRAINT fk_royalty_distributions_song_id ON royalty_distributions IS 'Canción de la cual se distribuyen royalties';
COMMENT ON CONSTRAINT fk_royalty_distributions_artist_id ON royalty_distributions IS 'Artista que recibe royalties';

COMMENT ON CONSTRAINT fk_revenue_sharing_contract_id ON revenue_sharing_distributions IS 'Contrato de ownership relacionado';
COMMENT ON CONSTRAINT fk_revenue_sharing_song_id ON revenue_sharing_distributions IS 'Canción relacionada';

COMMENT ON CONSTRAINT fk_shareholder_distributions_shareholder_id ON shareholder_distributions IS 'Usuario accionista';
COMMENT ON CONSTRAINT fk_shareholder_distributions_payment_id ON shareholder_distributions IS 'Pago relacionado (opcional)';

COMMENT ON CONSTRAINT fk_payment_batch_items_payment_id ON payment_batch_items IS 'Pago incluido en el batch';

COMMENT ON CONSTRAINT fk_fraud_alerts_payment_id ON fraud_alerts IS 'Pago con alerta de fraude';
COMMENT ON CONSTRAINT fk_fraud_alerts_user_id ON fraud_alerts IS 'Usuario relacionado';
COMMENT ON CONSTRAINT fk_fraud_alerts_reviewed_by ON fraud_alerts IS 'Usuario que revisó (opcional)';

COMMENT ON CONSTRAINT fk_ownership_contracts_artist_id ON ownership_contracts IS 'Artista propietario del contrato';

COMMENT ON CONSTRAINT fk_share_transactions_contract_id ON share_transactions IS 'Contrato de la transacción';
COMMENT ON CONSTRAINT fk_share_transactions_buyer_id ON share_transactions IS 'Comprador (opcional)';
COMMENT ON CONSTRAINT fk_share_transactions_seller_id ON share_transactions IS 'Vendedor (opcional)';

COMMENT ON CONSTRAINT fk_revenue_distributions_contract_id ON revenue_distributions IS 'Contrato relacionado';

COMMENT ON CONSTRAINT fk_campaigns_song_id ON campaigns IS 'Canción de la campaña';
COMMENT ON CONSTRAINT fk_campaigns_artist_id ON campaigns IS 'Artista creador';

COMMENT ON CONSTRAINT fk_nft_purchases_campaign_id ON nft_purchases IS 'Campaña relacionada';
COMMENT ON CONSTRAINT fk_nft_purchases_user_id ON nft_purchases IS 'Usuario comprador';

COMMENT ON CONSTRAINT fk_campaign_analytics_campaign_id ON campaign_analytics IS 'Campaña analizada';

COMMENT ON CONSTRAINT fk_listen_sessions_user_id ON listen_sessions IS 'Usuario que escucha';
COMMENT ON CONSTRAINT fk_listen_sessions_song_id ON listen_sessions IS 'Canción escuchada';
COMMENT ON CONSTRAINT fk_listen_sessions_artist_id ON listen_sessions IS 'Artista de la canción';

COMMENT ON CONSTRAINT fk_user_reward_history_user_id ON user_reward_history IS 'Usuario que recibió recompensa';
COMMENT ON CONSTRAINT fk_user_reward_history_session_id ON user_reward_history IS 'Sesión relacionada (opcional)';
COMMENT ON CONSTRAINT fk_user_reward_history_distribution_id ON user_reward_history IS 'Distribución relacionada (opcional)';

COMMENT ON CONSTRAINT fk_notifications_user_id ON notifications IS 'Usuario destinatario';

COMMENT ON CONSTRAINT fk_fan_verifications_fan_id ON fan_verifications IS 'Fan verificado';

COMMENT ON CONSTRAINT fk_nft_wristbands_fan_id ON nft_wristbands IS 'Fan propietario del wristband';

