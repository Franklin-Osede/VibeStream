-- Migración para el Sistema de Notificaciones
-- Fecha: 2024-07-26

-- Tabla principal de notificaciones
CREATE TABLE IF NOT EXISTS notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    notification_type VARCHAR(50) NOT NULL,
    priority VARCHAR(20) NOT NULL DEFAULT 'normal',
    status VARCHAR(20) NOT NULL DEFAULT 'unread',
    metadata JSONB,
    read_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Tabla de preferencias de notificaciones por usuario
CREATE TABLE IF NOT EXISTS notification_preferences (
    user_id UUID PRIMARY KEY,
    email_enabled BOOLEAN NOT NULL DEFAULT true,
    push_enabled BOOLEAN NOT NULL DEFAULT true,
    in_app_enabled BOOLEAN NOT NULL DEFAULT true,
    venture_notifications BOOLEAN NOT NULL DEFAULT true,
    reward_notifications BOOLEAN NOT NULL DEFAULT true,
    campaign_notifications BOOLEAN NOT NULL DEFAULT true,
    system_notifications BOOLEAN NOT NULL DEFAULT true,
    quiet_hours_start SMALLINT CHECK (quiet_hours_start >= 0 AND quiet_hours_start <= 23),
    quiet_hours_end SMALLINT CHECK (quiet_hours_end >= 0 AND quiet_hours_end <= 23),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Tabla de plantillas de notificaciones
CREATE TABLE IF NOT EXISTS notification_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL UNIQUE,
    title_template TEXT NOT NULL,
    message_template TEXT NOT NULL,
    notification_type VARCHAR(50) NOT NULL,
    priority VARCHAR(20) NOT NULL DEFAULT 'normal',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Índices para optimizar consultas
CREATE INDEX IF NOT EXISTS idx_notifications_user_id ON notifications(user_id);
CREATE INDEX IF NOT EXISTS idx_notifications_status ON notifications(status);
CREATE INDEX IF NOT EXISTS idx_notifications_created_at ON notifications(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_notifications_type ON notifications(notification_type);
CREATE INDEX IF NOT EXISTS idx_notifications_priority ON notifications(priority);
CREATE INDEX IF NOT EXISTS idx_notifications_user_status ON notifications(user_id, status);

-- Índices para notification_preferences
CREATE INDEX IF NOT EXISTS idx_notification_preferences_user_id ON notification_preferences(user_id);

-- Índices para notification_templates
CREATE INDEX IF NOT EXISTS idx_notification_templates_name ON notification_templates(name);
CREATE INDEX IF NOT EXISTS idx_notification_templates_active ON notification_templates(is_active);
CREATE INDEX IF NOT EXISTS idx_notification_templates_type ON notification_templates(notification_type);

-- Trigger para actualizar updated_at automáticamente
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Aplicar trigger a todas las tablas
CREATE TRIGGER update_notifications_updated_at 
    BEFORE UPDATE ON notifications 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_notification_preferences_updated_at 
    BEFORE UPDATE ON notification_preferences 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_notification_templates_updated_at 
    BEFORE UPDATE ON notification_templates 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Insertar plantillas de notificación por defecto
INSERT INTO notification_templates (name, title_template, message_template, notification_type, priority) VALUES
-- Fan Ventures
('venture_created', 'Nueva venture: {{venture_title}}', 'El artista ha creado una nueva venture: {{venture_title}}. ¡Invierte ahora!', 'venture_created', 'normal'),
('venture_funded', 'Venture financiada: {{venture_title}}', '¡La venture "{{venture_title}}" ha sido completamente financiada!', 'venture_funded', 'high'),
('venture_expired', 'Venture expirada: {{venture_title}}', 'La venture "{{venture_title}}" ha expirado sin alcanzar su meta de financiamiento.', 'venture_expired', 'normal'),
('investment_made', 'Inversión realizada', 'Has invertido ${{amount}} en la venture: {{venture_title}}', 'investment_made', 'normal'),
('benefit_delivered', 'Beneficio entregado', 'Tu beneficio "{{benefit_title}}" de la venture "{{venture_title}}" ha sido entregado', 'benefit_delivered', 'high'),
('revenue_distributed', 'Ingresos distribuidos', 'Se han distribuido ${{amount}} de ingresos de la venture "{{venture_title}}"', 'revenue_distributed', 'normal'),

-- Listen Rewards
('listen_session_completed', 'Sesión completada', 'Has completado la escucha de "{{song_title}}" y ganado ${{reward_amount}}', 'listen_session_completed', 'low'),
('reward_earned', 'Recompensa ganada', '¡Has ganado ${{amount}} por escuchar música!', 'reward_earned', 'normal'),
('zk_proof_verified', 'Prueba ZK verificada', 'Tu prueba ZK {{proof_id}} ha sido verificada exitosamente', 'zk_proof_verified', 'normal'),

-- Campaigns
('campaign_launched', 'Campaña lanzada: {{campaign_title}}', 'Se ha lanzado una nueva campaña: {{campaign_title}}', 'campaign_launched', 'normal'),
('campaign_ended', 'Campaña finalizada: {{campaign_title}}', 'La campaña "{{campaign_title}}" ha finalizado', 'campaign_ended', 'normal'),
('campaign_milestone_reached', 'Hito alcanzado: {{campaign_title}}', 'La campaña "{{campaign_title}}" ha alcanzado un nuevo hito: {{milestone}}', 'campaign_milestone_reached', 'high'),

-- User
('account_created', '¡Bienvenido a VibeStream!', 'Tu cuenta ha sido creada exitosamente. ¡Disfruta de la música!', 'account_created', 'normal'),
('profile_updated', 'Perfil actualizado', 'Tu perfil ha sido actualizado exitosamente', 'profile_updated', 'low'),
('wallet_linked', 'Wallet conectada', 'Tu wallet ha sido conectada exitosamente', 'wallet_linked', 'normal'),

-- System
('system_maintenance', 'Mantenimiento del sistema', 'El sistema estará en mantenimiento el {{date}} de {{time}} a {{end_time}}', 'system_maintenance', 'high'),
('security_alert', 'Alerta de seguridad', 'Se ha detectado actividad sospechosa en tu cuenta. Por favor, verifica tu seguridad.', 'security_alert', 'urgent'),
('welcome_message', '¡Bienvenido!', 'Gracias por unirte a VibeStream. ¡Esperamos que disfrutes de la experiencia!', 'welcome_message', 'normal')

ON CONFLICT (name) DO NOTHING;

-- Comentarios para documentación
COMMENT ON TABLE notifications IS 'Tabla principal para almacenar todas las notificaciones del sistema';
COMMENT ON TABLE notification_preferences IS 'Preferencias de notificación por usuario';
COMMENT ON TABLE notification_templates IS 'Plantillas reutilizables para notificaciones';

COMMENT ON COLUMN notifications.notification_type IS 'Tipo de notificación (venture_created, investment_made, etc.)';
COMMENT ON COLUMN notifications.priority IS 'Prioridad de la notificación (low, normal, high, urgent)';
COMMENT ON COLUMN notifications.status IS 'Estado de la notificación (unread, read, archived)';
COMMENT ON COLUMN notifications.metadata IS 'Datos adicionales en formato JSON';
COMMENT ON COLUMN notifications.read_at IS 'Timestamp cuando la notificación fue marcada como leída';

COMMENT ON COLUMN notification_preferences.quiet_hours_start IS 'Hora de inicio de horas silenciosas (0-23)';
COMMENT ON COLUMN notification_preferences.quiet_hours_end IS 'Hora de fin de horas silenciosas (0-23)'; 