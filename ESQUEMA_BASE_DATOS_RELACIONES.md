# Esquema de Base de Datos - Relaciones y Constraints

> **Documento completo** que define todas las relaciones entre tablas, primary keys, foreign keys y constraints del sistema VibeStream.

---

## 📊 Resumen Ejecutivo

| Categoría | Cantidad |
|-----------|----------|
| **Tablas Principales** | ~35 tablas |
| **Foreign Keys** | ~50 relaciones |
| **Índices** | ~80 índices |
| **Constraints** | ~30 constraints |
| **Triggers** | ~10 triggers |

---

## 🔑 Tablas Principales y Primary Keys

### 1. Core User Management

#### `users`
```sql
PRIMARY KEY: id (UUID)
UNIQUE: email, username
```

**Campos clave**:
- `id` - UUID (Primary Key)
- `email` - VARCHAR(255) UNIQUE NOT NULL
- `username` - VARCHAR(100) UNIQUE NOT NULL
- `password_hash` - VARCHAR(255) NOT NULL
- `wallet_address` - VARCHAR(255)
- `role` - VARCHAR(50) DEFAULT 'user'
- `tier` - VARCHAR(20) DEFAULT 'free'
- `is_verified` - BOOLEAN DEFAULT FALSE

#### `artists`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: user_id → users(id) ON DELETE CASCADE
```

**Relaciones**:
- `user_id` → `users.id` (CASCADE) - Un artista pertenece a un usuario

#### `user_achievements`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: user_id → users(id) ON DELETE CASCADE
UNIQUE: (user_id, achievement_id)
```

**Relaciones**:
- `user_id` → `users.id` (CASCADE) - Un logro pertenece a un usuario

#### `user_tier_progress`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: user_id → users(id) ON DELETE CASCADE
```

**Relaciones**:
- `user_id` → `users.id` (CASCADE) - Progreso de tier pertenece a un usuario

---

### 2. Music & Content

#### `songs`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: artist_id → artists(id) ON DELETE CASCADE
```

**Relaciones**:
- `artist_id` → `artists.id` (CASCADE) - Una canción pertenece a un artista

**Campos clave**:
- `id` - UUID (Primary Key)
- `title` - VARCHAR(255) NOT NULL
- `artist_id` - UUID NOT NULL → `artists.id`
- `genre` - VARCHAR(100)
- `royalty_percentage` - DECIMAL(5,2) DEFAULT 10.00
- `play_count` - INTEGER DEFAULT 0
- `revenue_generated` - DECIMAL(10,4) DEFAULT 0.0

#### `playlists`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: user_id → users(id) ON DELETE CASCADE
```

**Relaciones**:
- `user_id` → `users.id` (CASCADE) - Una playlist pertenece a un usuario

#### `playlist_songs`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: playlist_id → playlists(id) ON DELETE CASCADE
FOREIGN KEY: song_id → songs(id) ON DELETE CASCADE
UNIQUE: (playlist_id, song_id)
```

**Relaciones**:
- `playlist_id` → `playlists.id` (CASCADE) - Pertenece a una playlist
- `song_id` → `songs.id` (CASCADE) - Referencia una canción

**Relación Many-to-Many**: `playlists` ↔ `songs` (a través de `playlist_songs`)

#### `song_analytics`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: song_id → songs(id) ON DELETE CASCADE
```

**Relaciones**:
- `song_id` → `songs.id` (CASCADE) - Analytics de una canción

---

### 3. Payment System

#### `payments`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: payer_id → users(id) [NO CASCADE - SET NULL si se elimina]
FOREIGN KEY: payee_id → users(id) [NO CASCADE - SET NULL si se elimina]
FOREIGN KEY: transaction_id → transactions(id) [OPCIONAL]
```

**Relaciones**:
- `payer_id` → `users.id` - Usuario que paga
- `payee_id` → `users.id` - Usuario que recibe
- `transaction_id` → `transactions.id` (OPCIONAL) - Transacción blockchain relacionada

**Campos clave**:
- `id` - UUID (Primary Key)
- `payer_id` - UUID NOT NULL → `users.id`
- `payee_id` - UUID NOT NULL → `users.id`
- `amount_value` - DECIMAL(15,6) NOT NULL
- `amount_currency` - VARCHAR(10) NOT NULL
- `status` - VARCHAR(20) DEFAULT 'Pending'

#### `royalty_distributions`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: song_id → songs(id) [NO CASCADE]
FOREIGN KEY: artist_id → users(id) [NO CASCADE]
```

**Relaciones**:
- `song_id` → `songs.id` - Canción de la cual se distribuyen royalties
- `artist_id` → `users.id` - Artista que recibe royalties

#### `revenue_sharing_distributions`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: contract_id → ownership_contracts(id) [NO CASCADE]
FOREIGN KEY: song_id → songs(id) [NO CASCADE]
```

**Relaciones**:
- `contract_id` → `ownership_contracts.id` - Contrato de ownership
- `song_id` → `songs.id` - Canción relacionada

#### `shareholder_distributions`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: distribution_id → revenue_sharing_distributions(id) ON DELETE CASCADE
FOREIGN KEY: shareholder_id → users(id) [NO CASCADE]
FOREIGN KEY: payment_id → payments(id) [OPCIONAL]
UNIQUE: (distribution_id, shareholder_id)
```

**Relaciones**:
- `distribution_id` → `revenue_sharing_distributions.id` (CASCADE) - Distribución padre
- `shareholder_id` → `users.id` - Usuario accionista
- `payment_id` → `payments.id` (OPCIONAL) - Pago relacionado

#### `payment_batches`
```sql
PRIMARY KEY: id (UUID)
```

**Sin foreign keys directos** - Tabla independiente para procesamiento por lotes

#### `payment_batch_items`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: batch_id → payment_batches(id) ON DELETE CASCADE
FOREIGN KEY: payment_id → payments(id) [NO CASCADE]
UNIQUE: (batch_id, payment_id)
```

**Relaciones**:
- `batch_id` → `payment_batches.id` (CASCADE) - Pertenece a un batch
- `payment_id` → `payments.id` - Pago incluido en el batch

#### `payment_events`
```sql
PRIMARY KEY: id (UUID)
UNIQUE: (aggregate_id, event_version)
```

**Event Sourcing** - Sin foreign keys, usa `aggregate_id` para referenciar entidades

#### `fraud_alerts`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: payment_id → payments(id) [NO CASCADE]
FOREIGN KEY: user_id → users(id) [NO CASCADE]
FOREIGN KEY: reviewed_by → users(id) [OPCIONAL]
UNIQUE: payment_id
```

**Relaciones**:
- `payment_id` → `payments.id` - Pago con alerta de fraude
- `user_id` → `users.id` - Usuario relacionado
- `reviewed_by` → `users.id` (OPCIONAL) - Usuario que revisó

---

### 4. Fractional Ownership / Fan Ventures

#### `ownership_contracts`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: song_id → songs(id) ON DELETE CASCADE
FOREIGN KEY: artist_id → users(id) [NO CASCADE]
```

**Relaciones**:
- `song_id` → `songs.id` (CASCADE) - Canción del contrato
- `artist_id` → `users.id` - Artista propietario

**Campos clave**:
- `id` - UUID (Primary Key)
- `song_id` - UUID NOT NULL → `songs.id`
- `artist_id` - UUID NOT NULL → `users.id`
- `total_shares` - INTEGER NOT NULL
- `shares_available` - INTEGER NOT NULL
- `price_per_share` - DECIMAL(10,4) NOT NULL

#### `user_shares`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: user_id → users(id) ON DELETE CASCADE
FOREIGN KEY: contract_id → ownership_contracts(id) ON DELETE CASCADE
UNIQUE: (user_id, contract_id)
```

**Relaciones**:
- `user_id` → `users.id` (CASCADE) - Usuario propietario de shares
- `contract_id` → `ownership_contracts.id` (CASCADE) - Contrato relacionado

**Relación Many-to-Many**: `users` ↔ `ownership_contracts` (a través de `user_shares`)

#### `share_transactions`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: contract_id → ownership_contracts(id) [NO CASCADE]
FOREIGN KEY: buyer_id → users(id) [OPCIONAL]
FOREIGN KEY: seller_id → users(id) [OPCIONAL]
```

**Relaciones**:
- `contract_id` → `ownership_contracts.id` - Contrato de la transacción
- `buyer_id` → `users.id` (OPCIONAL) - Comprador
- `seller_id` → `users.id` (OPCIONAL) - Vendedor

#### `revenue_distributions`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: contract_id → ownership_contracts(id) [NO CASCADE]
```

**Relaciones**:
- `contract_id` → `ownership_contracts.id` - Contrato relacionado

---

### 5. Campaign & NFT System

#### `campaigns`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: song_id → songs(id) [NO CASCADE]
FOREIGN KEY: artist_id → users(id) [NO CASCADE]
```

**Relaciones**:
- `song_id` → `songs.id` - Canción de la campaña
- `artist_id` → `users.id` - Artista creador

**Campos clave**:
- `id` - UUID (Primary Key)
- `song_id` - UUID NOT NULL → `songs.id`
- `artist_id` - UUID NOT NULL → `users.id`
- `name` - VARCHAR(200) NOT NULL
- `start_date` - TIMESTAMPTZ NOT NULL
- `end_date` - TIMESTAMPTZ NOT NULL
- `status` - VARCHAR(20) DEFAULT 'upcoming'

#### `nft_purchases`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: campaign_id → campaigns(id) [NO CASCADE]
FOREIGN KEY: user_id → users(id) [NO CASCADE]
```

**Relaciones**:
- `campaign_id` → `campaigns.id` - Campaña relacionada
- `user_id` → `users.id` - Usuario comprador

#### `campaign_analytics`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: campaign_id → campaigns(id) [NO CASCADE]
UNIQUE: (campaign_id, date)
```

**Relaciones**:
- `campaign_id` → `campaigns.id` - Campaña analizada

---

### 6. Listen Reward System

#### `listen_sessions`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: user_id → users(id) [NO CASCADE]
FOREIGN KEY: song_id → songs(id) ON DELETE CASCADE
FOREIGN KEY: artist_id → artists(id) [NO CASCADE]
```

**Relaciones**:
- `user_id` → `users.id` - Usuario que escucha
- `song_id` → `songs.id` (CASCADE) - Canción escuchada
- `artist_id` → `artists.id` - Artista de la canción

**Campos clave**:
- `id` - UUID (Primary Key)
- `user_id` - UUID NOT NULL → `users.id`
- `song_id` - UUID NOT NULL → `songs.id`
- `artist_id` - UUID NOT NULL → `artists.id`
- `status` - VARCHAR(20) DEFAULT 'active'
- `final_reward_tokens` - DECIMAL(10,4)

#### `reward_distributions`
```sql
PRIMARY KEY: id (UUID)
```

**Sin foreign keys directos** - Tabla independiente para pools de recompensas

#### `user_reward_history`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: user_id → users(id) [NO CASCADE]
FOREIGN KEY: session_id → listen_sessions(id) [OPCIONAL]
FOREIGN KEY: distribution_id → reward_distributions(id) [OPCIONAL]
```

**Relaciones**:
- `user_id` → `users.id` - Usuario que recibió recompensa
- `session_id` → `listen_sessions.id` (OPCIONAL) - Sesión relacionada
- `distribution_id` → `reward_distributions.id` (OPCIONAL) - Distribución relacionada

---

### 7. Transactions (Blockchain)

#### `transactions`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: user_id → users(id) ON DELETE SET NULL
UNIQUE: request_id
```

**Relaciones**:
- `user_id` → `users.id` (SET NULL) - Usuario relacionado (puede ser NULL)

**Campos clave**:
- `id` - UUID (Primary Key)
- `request_id` - VARCHAR(255) UNIQUE NOT NULL
- `user_id` - UUID → `users.id` (SET NULL)
- `blockchain` - VARCHAR(50) NOT NULL ('Ethereum' o 'Solana')
- `tx_hash` - VARCHAR(255)
- `status` - VARCHAR(50) DEFAULT 'pending'

#### `royalty_payments`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: song_id → songs(id) ON DELETE CASCADE
FOREIGN KEY: artist_id → artists(id) ON DELETE CASCADE
FOREIGN KEY: transaction_id → transactions(id) ON DELETE SET NULL
```

**Relaciones**:
- `song_id` → `songs.id` (CASCADE) - Canción relacionada
- `artist_id` → `artists.id` (CASCADE) - Artista que recibe
- `transaction_id` → `transactions.id` (SET NULL) - Transacción blockchain

#### `listen_events`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: user_id → users(id) ON DELETE SET NULL
FOREIGN KEY: song_id → songs(id) ON DELETE CASCADE
```

**Relaciones**:
- `user_id` → `users.id` (SET NULL) - Usuario que escuchó
- `song_id` → `songs.id` (CASCADE) - Canción escuchada

---

### 8. Notifications

#### `notifications`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: user_id → users(id) [NO CASCADE]
```

**Relaciones**:
- `user_id` → `users.id` - Usuario destinatario

**Campos clave**:
- `id` - UUID (Primary Key)
- `user_id` - UUID NOT NULL → `users.id`
- `type` - VARCHAR(50) NOT NULL
- `related_id` - UUID (OPCIONAL) - Puede referenciar cualquier entidad
- `is_read` - BOOLEAN DEFAULT false

---

### 9. Fan Loyalty System

#### `fan_verifications`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: fan_id → users(id) [NO CASCADE]
UNIQUE: verification_id
```

**Relaciones**:
- `fan_id` → `users.id` - Fan verificado

**Campos clave**:
- `id` - UUID (Primary Key)
- `fan_id` - UUID NOT NULL → `users.id`
- `is_verified` - BOOLEAN NOT NULL DEFAULT FALSE
- `confidence_score` - DECIMAL(5,4) NOT NULL DEFAULT 0.0
- `verification_id` - VARCHAR(255) NOT NULL UNIQUE
- `wristband_eligible` - BOOLEAN NOT NULL DEFAULT FALSE

#### `nft_wristbands`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: fan_id → users(id) [NO CASCADE]
```

**Relaciones**:
- `fan_id` → `users.id` - Fan propietario del wristband

**Campos clave**:
- `id` - UUID (Primary Key)
- `fan_id` - UUID NOT NULL → `users.id`
- `concert_id` - VARCHAR(255) NOT NULL
- `artist_id` - VARCHAR(255) NOT NULL
- `wristband_type` - VARCHAR(50) NOT NULL ('General', 'VIP', 'Backstage', 'MeetAndGreet')
- `is_active` - BOOLEAN NOT NULL DEFAULT FALSE
- `nft_token_id` - VARCHAR(255)

#### `qr_codes`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: wristband_id → nft_wristbands(id) ON DELETE CASCADE
UNIQUE: code
```

**Relaciones**:
- `wristband_id` → `nft_wristbands.id` (CASCADE) - QR code pertenece a un wristband

#### `zk_proofs`
```sql
PRIMARY KEY: id (UUID)
UNIQUE: proof_id
```

**Sin foreign keys** - Tabla independiente para almacenar pruebas ZK

#### `fan_loyalty_events`
```sql
PRIMARY KEY: id (UUID)
UNIQUE: event_id
```

**Event Sourcing** - Sin foreign keys, usa `aggregate_id` para referenciar entidades

#### `fan_loyalty_audit_log`
```sql
PRIMARY KEY: id (UUID)
```

**Auditoría** - Sin foreign keys, almacena cambios en tablas del sistema

---

### 10. Fan Ventures

#### `artist_ventures`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: artist_id → users(id) ON DELETE CASCADE
```

**Relaciones**:
- `artist_id` → `users.id` (CASCADE) - Artista creador del venture

**Campos clave**:
- `id` - UUID (Primary Key)
- `artist_id` - UUID NOT NULL → `users.id`
- `title` - VARCHAR(255) NOT NULL
- `category` - VARCHAR(20) DEFAULT 'other'
- `status` - VARCHAR(20) DEFAULT 'draft'
- `funding_goal` - DOUBLE PRECISION NOT NULL
- `current_funding` - DOUBLE PRECISION DEFAULT 0.0

#### `fan_investments`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: fan_id → users(id) ON DELETE CASCADE
FOREIGN KEY: venture_id → artist_ventures(id) ON DELETE CASCADE
UNIQUE: (fan_id, venture_id)
```

**Relaciones**:
- `fan_id` → `users.id` (CASCADE) - Fan inversor
- `venture_id` → `artist_ventures.id` (CASCADE) - Venture relacionado

**Relación Many-to-Many**: `users` ↔ `artist_ventures` (a través de `fan_investments`)

#### `venture_tiers`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: venture_id → artist_ventures(id) ON DELETE CASCADE
```

**Relaciones**:
- `venture_id` → `artist_ventures.id` (CASCADE) - Tier pertenece a un venture

#### `venture_benefits`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: venture_id → artist_ventures(id) ON DELETE CASCADE
FOREIGN KEY: tier_id → venture_tiers(id) ON DELETE CASCADE
```

**Relaciones**:
- `venture_id` → `artist_ventures.id` (CASCADE) - Benefit pertenece a un venture
- `tier_id` → `venture_tiers.id` (CASCADE) - Benefit asociado a un tier

#### `benefit_deliveries`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: benefit_id → venture_benefits(id) ON DELETE CASCADE
FOREIGN KEY: venture_id → artist_ventures(id) ON DELETE CASCADE
FOREIGN KEY: fan_id → users(id) ON DELETE CASCADE
FOREIGN KEY: tier_id → venture_tiers(id) ON DELETE SET NULL
```

**Relaciones**:
- `benefit_id` → `venture_benefits.id` (CASCADE) - Delivery de un benefit
- `venture_id` → `artist_ventures.id` (CASCADE) - Venture relacionado
- `fan_id` → `users.id` (CASCADE) - Fan que recibe el benefit
- `tier_id` → `venture_tiers.id` (SET NULL) - Tier relacionado

#### `fan_preferences`
```sql
PRIMARY KEY: id (UUID)
FOREIGN KEY: fan_id → users(id) ON DELETE CASCADE
UNIQUE: fan_id
```

**Relaciones**:
- `fan_id` → `users.id` (CASCADE) - Preferencias de un fan (1:1)

---

## 🔗 Diagrama de Relaciones Principales

```
┌─────────────┐
│    users    │ (PK: id)
└──────┬──────┘
       │
       ├─────────────────────────────────────────────┐
       │                                             │
       │                                             │
┌──────▼──────┐                            ┌────────▼────────┐
│   artists   │                            │ user_achievements│
│ (PK: id)    │                            │  (PK: id)       │
│ (FK: user_id│                            │  (FK: user_id)  │
└──────┬──────┘                            └─────────────────┘
       │
       │
┌──────▼──────┐
│    songs    │ (PK: id)
│ (FK: artist_id)│
└──────┬──────┘
       │
       ├─────────────────────────────────────────────┐
       │                                             │
       │                                             │
┌──────▼──────────────┐                    ┌────────▼──────────────┐
│  listen_sessions   │                    │  ownership_contracts  │
│ (PK: id)           │                    │  (PK: id)            │
│ (FK: song_id)      │                    │  (FK: song_id)       │
│ (FK: user_id)      │                    │  (FK: artist_id)     │
└────────────────────┘                    └────────┬───────────────┘
                                                   │
                                                   │
                                          ┌────────▼──────────┐
                                          │   user_shares     │
                                          │  (PK: id)         │
                                          │  (FK: user_id)    │
                                          │  (FK: contract_id)│
                                          └───────────────────┘

┌─────────────┐
│  campaigns  │ (PK: id)
│ (FK: song_id)│
│ (FK: artist_id)│
└──────┬──────┘
       │
       │
┌──────▼──────────┐
│  nft_purchases │ (PK: id)
│ (FK: campaign_id)│
│ (FK: user_id)  │
└────────────────┘

┌─────────────┐
│  payments   │ (PK: id)
│ (FK: payer_id)│
│ (FK: payee_id)│
└──────┬──────┘
       │
       ├──────────────────────────────┐
       │                              │
┌──────▼──────────────┐    ┌──────────▼──────────────┐
│royalty_distributions│    │revenue_sharing_distrib. │
│ (PK: id)            │    │ (PK: id)                │
│ (FK: song_id)       │    │ (FK: contract_id)       │
│ (FK: artist_id)     │    └─────────────────────────┘
└─────────────────────┘
```

---

## 📋 Resumen de Foreign Keys por Tabla

### Tablas que Referencian `users`

| Tabla | Campo | Tipo de Relación | ON DELETE |
|-------|-------|------------------|-----------|
| `artists` | `user_id` | 1:1 | CASCADE |
| `user_achievements` | `user_id` | 1:N | CASCADE |
| `user_tier_progress` | `user_id` | 1:1 | CASCADE |
| `playlists` | `user_id` | 1:N | CASCADE |
| `payments` | `payer_id` | 1:N | SET NULL |
| `payments` | `payee_id` | 1:N | SET NULL |
| `transactions` | `user_id` | 1:N | SET NULL |
| `listen_sessions` | `user_id` | 1:N | NO ACTION |
| `listen_events` | `user_id` | 1:N | SET NULL |
| `notifications` | `user_id` | 1:N | NO ACTION |
| `nft_purchases` | `user_id` | 1:N | NO ACTION |
| `user_shares` | `user_id` | 1:N | CASCADE |
| `share_transactions` | `buyer_id` | 1:N | NO ACTION |
| `share_transactions` | `seller_id` | 1:N | NO ACTION |
| `fraud_alerts` | `user_id` | 1:N | NO ACTION |
| `fraud_alerts` | `reviewed_by` | 1:N | NO ACTION |
| `user_reward_history` | `user_id` | 1:N | NO ACTION |
| `fan_verifications` | `fan_id` | 1:1 | NO ACTION |
| `nft_wristbands` | `fan_id` | 1:N | NO ACTION |
| `fan_ventures` | `fan_id` | 1:N | CASCADE |
| `fan_investments` | `fan_id` | 1:N | CASCADE |
| `benefit_deliveries` | `fan_id` | 1:N | CASCADE |
| `fan_preferences` | `fan_id` | 1:1 | CASCADE |
| `ownership_contracts` | `artist_id` | 1:N | NO ACTION |
| `campaigns` | `artist_id` | 1:N | NO ACTION |
| `royalty_distributions` | `artist_id` | 1:N | NO ACTION |
| `listen_sessions` | `artist_id` | 1:N | NO ACTION |
| `artist_ventures` | `artist_id` | 1:N | CASCADE |

### Tablas que Referencian `songs`

| Tabla | Campo | Tipo de Relación | ON DELETE |
|-------|-------|------------------|-----------|
| `playlist_songs` | `song_id` | N:M (junction) | CASCADE |
| `listen_sessions` | `song_id` | 1:N | CASCADE |
| `listen_events` | `song_id` | 1:N | CASCADE |
| `song_analytics` | `song_id` | 1:1 | CASCADE |
| `royalty_payments` | `song_id` | 1:N | CASCADE |
| `royalty_distributions` | `song_id` | 1:N | NO ACTION |
| `ownership_contracts` | `song_id` | 1:1 | CASCADE |
| `campaigns` | `song_id` | 1:N | NO ACTION |
| `revenue_sharing_distributions` | `song_id` | 1:N | NO ACTION |

### Tablas que Referencian `artists`

| Tabla | Campo | Tipo de Relación | ON DELETE |
|-------|-------|------------------|-----------|
| `songs` | `artist_id` | 1:N | CASCADE |
| `royalty_payments` | `artist_id` | 1:N | CASCADE |
| `listen_sessions` | `artist_id` | 1:N | NO ACTION |

### Tablas que Referencian `campaigns`

| Tabla | Campo | Tipo de Relación | ON DELETE |
|-------|-------|------------------|-----------|
| `nft_purchases` | `campaign_id` | 1:N | NO ACTION |
| `campaign_analytics` | `campaign_id` | 1:N | NO ACTION |

### Tablas que Referencian `ownership_contracts`

| Tabla | Campo | Tipo de Relación | ON DELETE |
|-------|-------|------------------|-----------|
| `user_shares` | `contract_id` | 1:N | CASCADE |
| `share_transactions` | `contract_id` | 1:N | NO ACTION |
| `revenue_distributions` | `contract_id` | 1:N | NO ACTION |
| `revenue_sharing_distributions` | `contract_id` | 1:N | NO ACTION |

### Tablas que Referencian `artist_ventures`

| Tabla | Campo | Tipo de Relación | ON DELETE |
|-------|-------|------------------|-----------|
| `fan_investments` | `venture_id` | 1:N | CASCADE |
| `venture_tiers` | `venture_id` | 1:N | CASCADE |
| `venture_benefits` | `venture_id` | 1:N | CASCADE |
| `benefit_deliveries` | `venture_id` | 1:N | CASCADE |

### Tablas que Referencian `venture_tiers`

| Tabla | Campo | Tipo de Relación | ON DELETE |
|-------|-------|------------------|-----------|
| `venture_benefits` | `tier_id` | 1:N | CASCADE |
| `benefit_deliveries` | `tier_id` | 1:N | SET NULL |

### Tablas que Referencian `venture_benefits`

| Tabla | Campo | Tipo de Relación | ON DELETE |
|-------|-------|------------------|-----------|
| `benefit_deliveries` | `benefit_id` | 1:N | CASCADE |

### Tablas que Referencian `nft_wristbands`

| Tabla | Campo | Tipo de Relación | ON DELETE |
|-------|-------|------------------|-----------|
| `qr_codes` | `wristband_id` | 1:N | CASCADE |

### Tablas que Referencian `payments`

| Tabla | Campo | Tipo de Relación | ON DELETE |
|-------|-------|------------------|-----------|
| `payment_batch_items` | `payment_id` | 1:N | NO ACTION |
| `shareholder_distributions` | `payment_id` | 1:N | NO ACTION |
| `fraud_alerts` | `payment_id` | 1:1 | NO ACTION |

---

## 🔒 Constraints Importantes

### Unique Constraints

```sql
-- Users
UNIQUE(users.email)
UNIQUE(users.username)

-- User Achievements
UNIQUE(user_achievements.user_id, user_achievements.achievement_id)

-- Playlist Songs
UNIQUE(playlist_songs.playlist_id, playlist_songs.song_id)

-- Transactions
UNIQUE(transactions.request_id)

-- User Shares
UNIQUE(user_shares.user_id, user_shares.contract_id)

-- Shareholder Distributions
UNIQUE(shareholder_distributions.distribution_id, shareholder_distributions.shareholder_id)

-- Payment Batch Items
UNIQUE(payment_batch_items.batch_id, payment_batch_items.payment_id)

-- Campaign Analytics
UNIQUE(campaign_analytics.campaign_id, campaign_analytics.date)

-- Payment Events (Event Sourcing)
UNIQUE(payment_events.aggregate_id, payment_events.event_version)

-- Fraud Alerts
UNIQUE(fraud_alerts.payment_id)
```

### Check Constraints

```sql
-- Payments
CHECK (payments.amount_value >= 0)
CHECK (payments.amount_currency IN ('USD', 'ETH', 'SOL', 'USDC', 'VIBES'))
CHECK (payments.status IN ('Pending', 'Processing', 'Completed', 'Failed', 'Cancelled', 'Refunding', 'Refunded'))
CHECK (payments.net_amount_value <= payments.amount_value)

-- Royalty Distributions
CHECK (royalty_distributions.total_revenue_value > 0)
CHECK (royalty_distributions.artist_share_percentage >= 0 AND artist_share_percentage <= 100)
CHECK (royalty_distributions.period_end > royalty_distributions.period_start)
CHECK (royalty_distributions.artist_share_percentage + platform_fee_percentage <= 100)

-- Listen Sessions
CHECK (listen_sessions.user_tier IN ('basic', 'premium', 'vip', 'artist'))
CHECK (listen_sessions.status IN ('active', 'completed', 'verified', 'rewarded', 'failed', 'deleted'))
CHECK (listen_sessions.quality_score >= 0 AND quality_score <= 1)
CHECK (listen_sessions.base_reward_tokens >= 0)
CHECK (listen_sessions.final_reward_tokens >= 0)

-- Reward Distributions
CHECK (reward_distributions.total_tokens >= 0)
CHECK (reward_distributions.distributed_tokens >= 0)
CHECK (reward_distributions.reserved_tokens >= 0)
CHECK (reward_distributions.validation_period_end > validation_period_start)
CHECK (reward_distributions.distributed_tokens + reserved_tokens <= total_tokens)
```

---

## 📊 Índices Críticos para Performance

### Índices en `users`
```sql
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_wallet_address ON users(wallet_address);
CREATE INDEX idx_users_tier ON users(tier);
```

### Índices en `songs`
```sql
CREATE INDEX idx_songs_artist_id ON songs(artist_id);
CREATE INDEX idx_songs_genre ON songs(genre);
CREATE INDEX idx_songs_play_count ON songs(play_count DESC);
```

### Índices en `payments`
```sql
CREATE INDEX idx_payments_payer_id ON payments(payer_id);
CREATE INDEX idx_payments_payee_id ON payments(payee_id);
CREATE INDEX idx_payments_status ON payments(status);
CREATE INDEX idx_payments_created_at ON payments(created_at);
CREATE INDEX idx_payments_payer_status_date ON payments(payer_id, status, created_at);
```

### Índices en `listen_sessions`
```sql
CREATE INDEX idx_listen_sessions_user_id ON listen_sessions(user_id);
CREATE INDEX idx_listen_sessions_song_id ON listen_sessions(song_id);
CREATE INDEX idx_listen_sessions_status ON listen_sessions(status);
CREATE INDEX idx_listen_sessions_user_started ON listen_sessions(user_id, started_at);
```

### Índices en `campaigns`
```sql
CREATE INDEX idx_campaigns_song_id ON campaigns(song_id);
CREATE INDEX idx_campaigns_artist_id ON campaigns(artist_id);
CREATE INDEX idx_campaigns_status ON campaigns(status);
CREATE INDEX idx_campaigns_end_date ON campaigns(end_date);
```

---

## 🔄 Triggers y Funciones Automáticas

### Triggers de `updated_at`

```sql
-- Actualiza updated_at automáticamente
CREATE TRIGGER update_users_updated_at 
    BEFORE UPDATE ON users 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

-- Similar para: artists, songs, playlists, payments, etc.
```

### Triggers de Negocio

```sql
-- Actualiza tier del usuario basado en puntos
CREATE TRIGGER trigger_update_user_tier
    BEFORE UPDATE ON user_tier_progress
    FOR EACH ROW
    EXECUTE FUNCTION update_user_tier();

-- Incrementa play_count cuando se completa una sesión
CREATE TRIGGER trigger_increment_play_count
    AFTER UPDATE ON listen_sessions
    FOR EACH ROW
    EXECUTE FUNCTION increment_song_play_count();

-- Actualiza stats de ownership contracts
CREATE TRIGGER trigger_update_contract_stats
    AFTER INSERT OR UPDATE OR DELETE ON user_shares
    FOR EACH ROW
    EXECUTE FUNCTION update_contract_stats();
```

---

## 🎯 Reglas de Negocio por Relación

### 1. Users → Artists (1:1)
- **Regla**: Un usuario puede tener máximo un artista
- **Implementación**: `artists.user_id` es UNIQUE implícito (FK único)

### 2. Artists → Songs (1:N)
- **Regla**: Un artista puede tener múltiples canciones
- **Cascade**: Si se elimina un artista, se eliminan todas sus canciones

### 3. Users → Playlists (1:N)
- **Regla**: Un usuario puede tener múltiples playlists
- **Cascade**: Si se elimina un usuario, se eliminan todas sus playlists

### 4. Playlists ↔ Songs (N:M)
- **Regla**: Una playlist puede tener múltiples canciones, una canción puede estar en múltiples playlists
- **Junction Table**: `playlist_songs` con UNIQUE(playlist_id, song_id)
- **Cascade**: Si se elimina playlist o canción, se eliminan las relaciones

### 5. Users → Ownership Contracts (N:M)
- **Regla**: Un usuario puede tener shares en múltiples contratos, un contrato puede tener múltiples accionistas
- **Junction Table**: `user_shares` con UNIQUE(user_id, contract_id)
- **Cascade**: Si se elimina usuario o contrato, se eliminan las shares

### 6. Songs → Ownership Contracts (1:1)
- **Regla**: Una canción puede tener máximo un contrato de ownership
- **Implementación**: `ownership_contracts.song_id` debería ser UNIQUE

### 7. Campaigns → NFT Purchases (1:N)
- **Regla**: Una campaña puede tener múltiples compras de NFT
- **Sin cascade**: Las compras se mantienen aunque se elimine la campaña (histórico)

### 8. Listen Sessions → Songs (N:1)
- **Regla**: Múltiples sesiones pueden referenciar la misma canción
- **Cascade**: Si se elimina una canción, se eliminan todas sus sesiones

---

## 🔗 Lista Completa de Foreign Keys

### Foreign Keys Existentes en Migraciones

```sql
-- =====================================
-- CORE USER MANAGEMENT
-- =====================================
-- artists
ALTER TABLE artists ADD CONSTRAINT fk_artists_user_id 
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;

-- user_achievements
ALTER TABLE user_achievements ADD CONSTRAINT fk_user_achievements_user_id 
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;

-- user_tier_progress
ALTER TABLE user_tier_progress ADD CONSTRAINT fk_user_tier_progress_user_id 
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;

-- =====================================
-- MUSIC & CONTENT
-- =====================================
-- songs
ALTER TABLE songs ADD CONSTRAINT fk_songs_artist_id 
    FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE CASCADE;

-- playlists
ALTER TABLE playlists ADD CONSTRAINT fk_playlists_user_id 
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;

-- playlist_songs
ALTER TABLE playlist_songs ADD CONSTRAINT fk_playlist_songs_playlist_id 
    FOREIGN KEY (playlist_id) REFERENCES playlists(id) ON DELETE CASCADE;
ALTER TABLE playlist_songs ADD CONSTRAINT fk_playlist_songs_song_id 
    FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE CASCADE;

-- song_analytics
ALTER TABLE song_analytics ADD CONSTRAINT fk_song_analytics_song_id 
    FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE CASCADE;

-- =====================================
-- PAYMENT SYSTEM
-- =====================================
-- payments (FALTANTES - Agregar estas foreign keys)
-- ALTER TABLE payments ADD CONSTRAINT fk_payments_payer_id 
--     FOREIGN KEY (payer_id) REFERENCES users(id) ON DELETE SET NULL;
-- ALTER TABLE payments ADD CONSTRAINT fk_payments_payee_id 
--     FOREIGN KEY (payee_id) REFERENCES users(id) ON DELETE SET NULL;
-- ALTER TABLE payments ADD CONSTRAINT fk_payments_transaction_id 
--     FOREIGN KEY (transaction_id) REFERENCES transactions(id) ON DELETE SET NULL;

-- royalty_distributions (FALTANTES - Agregar estas foreign keys)
-- ALTER TABLE royalty_distributions ADD CONSTRAINT fk_royalty_distributions_song_id 
--     FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE RESTRICT;
-- ALTER TABLE royalty_distributions ADD CONSTRAINT fk_royalty_distributions_artist_id 
--     FOREIGN KEY (artist_id) REFERENCES users(id) ON DELETE RESTRICT;

-- revenue_sharing_distributions (FALTANTES)
-- ALTER TABLE revenue_sharing_distributions ADD CONSTRAINT fk_revenue_sharing_contract_id 
--     FOREIGN KEY (contract_id) REFERENCES ownership_contracts(id) ON DELETE RESTRICT;
-- ALTER TABLE revenue_sharing_distributions ADD CONSTRAINT fk_revenue_sharing_song_id 
--     FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE RESTRICT;

-- shareholder_distributions
ALTER TABLE shareholder_distributions ADD CONSTRAINT fk_shareholder_distributions_distribution_id 
    FOREIGN KEY (distribution_id) REFERENCES revenue_sharing_distributions(id) ON DELETE CASCADE;
-- ALTER TABLE shareholder_distributions ADD CONSTRAINT fk_shareholder_distributions_shareholder_id 
--     FOREIGN KEY (shareholder_id) REFERENCES users(id) ON DELETE RESTRICT;
-- ALTER TABLE shareholder_distributions ADD CONSTRAINT fk_shareholder_distributions_payment_id 
--     FOREIGN KEY (payment_id) REFERENCES payments(id) ON DELETE SET NULL;

-- payment_batch_items
ALTER TABLE payment_batch_items ADD CONSTRAINT fk_payment_batch_items_batch_id 
    FOREIGN KEY (batch_id) REFERENCES payment_batches(id) ON DELETE CASCADE;
-- ALTER TABLE payment_batch_items ADD CONSTRAINT fk_payment_batch_items_payment_id 
--     FOREIGN KEY (payment_id) REFERENCES payments(id) ON DELETE RESTRICT;

-- fraud_alerts (FALTANTES)
-- ALTER TABLE fraud_alerts ADD CONSTRAINT fk_fraud_alerts_payment_id 
--     FOREIGN KEY (payment_id) REFERENCES payments(id) ON DELETE RESTRICT;
-- ALTER TABLE fraud_alerts ADD CONSTRAINT fk_fraud_alerts_user_id 
--     FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT;
-- ALTER TABLE fraud_alerts ADD CONSTRAINT fk_fraud_alerts_reviewed_by 
--     FOREIGN KEY (reviewed_by) REFERENCES users(id) ON DELETE SET NULL;

-- =====================================
-- FRACTIONAL OWNERSHIP
-- =====================================
-- ownership_contracts
ALTER TABLE ownership_contracts ADD CONSTRAINT fk_ownership_contracts_song_id 
    FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE CASCADE;
-- ALTER TABLE ownership_contracts ADD CONSTRAINT fk_ownership_contracts_artist_id 
--     FOREIGN KEY (artist_id) REFERENCES users(id) ON DELETE RESTRICT;

-- user_shares
ALTER TABLE user_shares ADD CONSTRAINT fk_user_shares_user_id 
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;
ALTER TABLE user_shares ADD CONSTRAINT fk_user_shares_contract_id 
    FOREIGN KEY (contract_id) REFERENCES ownership_contracts(id) ON DELETE CASCADE;

-- share_transactions (FALTANTES)
-- ALTER TABLE share_transactions ADD CONSTRAINT fk_share_transactions_contract_id 
--     FOREIGN KEY (contract_id) REFERENCES ownership_contracts(id) ON DELETE RESTRICT;
-- ALTER TABLE share_transactions ADD CONSTRAINT fk_share_transactions_buyer_id 
--     FOREIGN KEY (buyer_id) REFERENCES users(id) ON DELETE SET NULL;
-- ALTER TABLE share_transactions ADD CONSTRAINT fk_share_transactions_seller_id 
--     FOREIGN KEY (seller_id) REFERENCES users(id) ON DELETE SET NULL;

-- revenue_distributions (FALTANTES)
-- ALTER TABLE revenue_distributions ADD CONSTRAINT fk_revenue_distributions_contract_id 
--     FOREIGN KEY (contract_id) REFERENCES ownership_contracts(id) ON DELETE RESTRICT;

-- =====================================
-- CAMPAIGNS & NFTs
-- =====================================
-- campaigns (FALTANTES)
-- ALTER TABLE campaigns ADD CONSTRAINT fk_campaigns_song_id 
--     FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE RESTRICT;
-- ALTER TABLE campaigns ADD CONSTRAINT fk_campaigns_artist_id 
--     FOREIGN KEY (artist_id) REFERENCES users(id) ON DELETE RESTRICT;

-- nft_purchases (FALTANTES)
-- ALTER TABLE nft_purchases ADD CONSTRAINT fk_nft_purchases_campaign_id 
--     FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE RESTRICT;
-- ALTER TABLE nft_purchases ADD CONSTRAINT fk_nft_purchases_user_id 
--     FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT;

-- campaign_nfts
ALTER TABLE campaign_nfts ADD CONSTRAINT fk_campaign_nfts_campaign_id 
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE;

-- campaign_analytics (FALTANTES)
-- ALTER TABLE campaign_analytics ADD CONSTRAINT fk_campaign_analytics_campaign_id 
--     FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE;

-- =====================================
-- LISTEN REWARDS
-- =====================================
-- listen_sessions (FALTANTES)
-- ALTER TABLE listen_sessions ADD CONSTRAINT fk_listen_sessions_user_id 
--     FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT;
-- ALTER TABLE listen_sessions ADD CONSTRAINT fk_listen_sessions_song_id 
--     FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE CASCADE;
-- ALTER TABLE listen_sessions ADD CONSTRAINT fk_listen_sessions_artist_id 
--     FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE RESTRICT;

-- user_reward_history (FALTANTES)
-- ALTER TABLE user_reward_history ADD CONSTRAINT fk_user_reward_history_user_id 
--     FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT;
-- ALTER TABLE user_reward_history ADD CONSTRAINT fk_user_reward_history_session_id 
--     FOREIGN KEY (session_id) REFERENCES listen_sessions(id) ON DELETE SET NULL;
-- ALTER TABLE user_reward_history ADD CONSTRAINT fk_user_reward_history_distribution_id 
--     FOREIGN KEY (distribution_id) REFERENCES reward_distributions(id) ON DELETE SET NULL;

-- =====================================
-- TRANSACTIONS
-- =====================================
-- transactions
ALTER TABLE transactions ADD CONSTRAINT fk_transactions_user_id 
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL;

-- royalty_payments
ALTER TABLE royalty_payments ADD CONSTRAINT fk_royalty_payments_song_id 
    FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE CASCADE;
ALTER TABLE royalty_payments ADD CONSTRAINT fk_royalty_payments_artist_id 
    FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE CASCADE;
ALTER TABLE royalty_payments ADD CONSTRAINT fk_royalty_payments_transaction_id 
    FOREIGN KEY (transaction_id) REFERENCES transactions(id) ON DELETE SET NULL;

-- listen_events
ALTER TABLE listen_events ADD CONSTRAINT fk_listen_events_user_id 
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL;
ALTER TABLE listen_events ADD CONSTRAINT fk_listen_events_song_id 
    FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE CASCADE;

-- =====================================
-- NOTIFICATIONS
-- =====================================
-- notifications (FALTANTES)
-- ALTER TABLE notifications ADD CONSTRAINT fk_notifications_user_id 
--     FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT;

-- =====================================
-- FAN LOYALTY
-- =====================================
-- fan_verifications (FALTANTES)
-- ALTER TABLE fan_verifications ADD CONSTRAINT fk_fan_verifications_fan_id 
--     FOREIGN KEY (fan_id) REFERENCES users(id) ON DELETE RESTRICT;

-- nft_wristbands (FALTANTES)
-- ALTER TABLE nft_wristbands ADD CONSTRAINT fk_nft_wristbands_fan_id 
--     FOREIGN KEY (fan_id) REFERENCES users(id) ON DELETE RESTRICT;

-- qr_codes
ALTER TABLE qr_codes ADD CONSTRAINT fk_qr_codes_wristband_id 
    FOREIGN KEY (wristband_id) REFERENCES nft_wristbands(id) ON DELETE CASCADE;

-- =====================================
-- FAN VENTURES
-- =====================================
-- artist_ventures
ALTER TABLE artist_ventures ADD CONSTRAINT fk_artist_ventures_artist_id 
    FOREIGN KEY (artist_id) REFERENCES users(id) ON DELETE CASCADE;

-- fan_investments
ALTER TABLE fan_investments ADD CONSTRAINT fk_fan_investments_fan_id 
    FOREIGN KEY (fan_id) REFERENCES users(id) ON DELETE CASCADE;
ALTER TABLE fan_investments ADD CONSTRAINT fk_fan_investments_venture_id 
    FOREIGN KEY (venture_id) REFERENCES artist_ventures(id) ON DELETE CASCADE;

-- venture_tiers
ALTER TABLE venture_tiers ADD CONSTRAINT fk_venture_tiers_venture_id 
    FOREIGN KEY (venture_id) REFERENCES artist_ventures(id) ON DELETE CASCADE;

-- venture_benefits
ALTER TABLE venture_benefits ADD CONSTRAINT fk_venture_benefits_venture_id 
    FOREIGN KEY (venture_id) REFERENCES artist_ventures(id) ON DELETE CASCADE;
ALTER TABLE venture_benefits ADD CONSTRAINT fk_venture_benefits_tier_id 
    FOREIGN KEY (tier_id) REFERENCES venture_tiers(id) ON DELETE CASCADE;

-- benefit_deliveries
ALTER TABLE benefit_deliveries ADD CONSTRAINT fk_benefit_deliveries_benefit_id 
    FOREIGN KEY (benefit_id) REFERENCES venture_benefits(id) ON DELETE CASCADE;
ALTER TABLE benefit_deliveries ADD CONSTRAINT fk_benefit_deliveries_venture_id 
    FOREIGN KEY (venture_id) REFERENCES artist_ventures(id) ON DELETE CASCADE;
ALTER TABLE benefit_deliveries ADD CONSTRAINT fk_benefit_deliveries_fan_id 
    FOREIGN KEY (fan_id) REFERENCES users(id) ON DELETE CASCADE;
ALTER TABLE benefit_deliveries ADD CONSTRAINT fk_benefit_deliveries_tier_id 
    FOREIGN KEY (tier_id) REFERENCES venture_tiers(id) ON DELETE SET NULL;

-- fan_preferences
ALTER TABLE fan_preferences ADD CONSTRAINT fk_fan_preferences_fan_id 
    FOREIGN KEY (fan_id) REFERENCES users(id) ON DELETE CASCADE;
```

### Foreign Keys Faltantes que Deben Agregarse

**⚠️ IMPORTANTE**: Las siguientes foreign keys NO están definidas en las migraciones actuales pero DEBEN agregarse:

1. **payments**:
   - `payer_id` → `users.id`
   - `payee_id` → `users.id`
   - `transaction_id` → `transactions.id` (opcional)

2. **royalty_distributions**:
   - `song_id` → `songs.id`
   - `artist_id` → `users.id`

3. **revenue_sharing_distributions**:
   - `contract_id` → `ownership_contracts.id`
   - `song_id` → `songs.id`

4. **shareholder_distributions**:
   - `shareholder_id` → `users.id`
   - `payment_id` → `payments.id` (opcional)

5. **payment_batch_items**:
   - `payment_id` → `payments.id`

6. **fraud_alerts**:
   - `payment_id` → `payments.id`
   - `user_id` → `users.id`
   - `reviewed_by` → `users.id` (opcional)

7. **ownership_contracts**:
   - `artist_id` → `users.id`

8. **share_transactions**:
   - `contract_id` → `ownership_contracts.id`
   - `buyer_id` → `users.id` (opcional)
   - `seller_id` → `users.id` (opcional)

9. **revenue_distributions**:
   - `contract_id` → `ownership_contracts.id`

10. **campaigns**:
    - `song_id` → `songs.id`
    - `artist_id` → `users.id`

11. **nft_purchases**:
    - `campaign_id` → `campaigns.id`
    - `user_id` → `users.id`

12. **campaign_analytics**:
    - `campaign_id` → `campaigns.id`

13. **listen_sessions**:
    - `user_id` → `users.id`
    - `song_id` → `songs.id`
    - `artist_id` → `artists.id`

14. **user_reward_history**:
    - `user_id` → `users.id`
    - `session_id` → `listen_sessions.id` (opcional)
    - `distribution_id` → `reward_distributions.id` (opcional)

15. **notifications**:
    - `user_id` → `users.id`

16. **fan_verifications**:
    - `fan_id` → `users.id`

17. **nft_wristbands**:
    - `fan_id` → `users.id`

### Script para Agregar Foreign Keys Faltantes

```sql
-- =====================================
-- MIGRATION: Add Missing Foreign Keys
-- =====================================

-- Payments
ALTER TABLE payments 
    ADD CONSTRAINT fk_payments_payer_id 
    FOREIGN KEY (payer_id) REFERENCES users(id) ON DELETE SET NULL;

ALTER TABLE payments 
    ADD CONSTRAINT fk_payments_payee_id 
    FOREIGN KEY (payee_id) REFERENCES users(id) ON DELETE SET NULL;

ALTER TABLE payments 
    ADD CONSTRAINT fk_payments_transaction_id 
    FOREIGN KEY (transaction_id) REFERENCES transactions(id) ON DELETE SET NULL;

-- Royalty Distributions
ALTER TABLE royalty_distributions 
    ADD CONSTRAINT fk_royalty_distributions_song_id 
    FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE RESTRICT;

ALTER TABLE royalty_distributions 
    ADD CONSTRAINT fk_royalty_distributions_artist_id 
    FOREIGN KEY (artist_id) REFERENCES users(id) ON DELETE RESTRICT;

-- Revenue Sharing Distributions
ALTER TABLE revenue_sharing_distributions 
    ADD CONSTRAINT fk_revenue_sharing_contract_id 
    FOREIGN KEY (contract_id) REFERENCES ownership_contracts(id) ON DELETE RESTRICT;

ALTER TABLE revenue_sharing_distributions 
    ADD CONSTRAINT fk_revenue_sharing_song_id 
    FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE RESTRICT;

-- Shareholder Distributions
ALTER TABLE shareholder_distributions 
    ADD CONSTRAINT fk_shareholder_distributions_shareholder_id 
    FOREIGN KEY (shareholder_id) REFERENCES users(id) ON DELETE RESTRICT;

ALTER TABLE shareholder_distributions 
    ADD CONSTRAINT fk_shareholder_distributions_payment_id 
    FOREIGN KEY (payment_id) REFERENCES payments(id) ON DELETE SET NULL;

-- Payment Batch Items
ALTER TABLE payment_batch_items 
    ADD CONSTRAINT fk_payment_batch_items_payment_id 
    FOREIGN KEY (payment_id) REFERENCES payments(id) ON DELETE RESTRICT;

-- Fraud Alerts
ALTER TABLE fraud_alerts 
    ADD CONSTRAINT fk_fraud_alerts_payment_id 
    FOREIGN KEY (payment_id) REFERENCES payments(id) ON DELETE RESTRICT;

ALTER TABLE fraud_alerts 
    ADD CONSTRAINT fk_fraud_alerts_user_id 
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT;

ALTER TABLE fraud_alerts 
    ADD CONSTRAINT fk_fraud_alerts_reviewed_by 
    FOREIGN KEY (reviewed_by) REFERENCES users(id) ON DELETE SET NULL;

-- Ownership Contracts
ALTER TABLE ownership_contracts 
    ADD CONSTRAINT fk_ownership_contracts_artist_id 
    FOREIGN KEY (artist_id) REFERENCES users(id) ON DELETE RESTRICT;

-- Share Transactions
ALTER TABLE share_transactions 
    ADD CONSTRAINT fk_share_transactions_contract_id 
    FOREIGN KEY (contract_id) REFERENCES ownership_contracts(id) ON DELETE RESTRICT;

ALTER TABLE share_transactions 
    ADD CONSTRAINT fk_share_transactions_buyer_id 
    FOREIGN KEY (buyer_id) REFERENCES users(id) ON DELETE SET NULL;

ALTER TABLE share_transactions 
    ADD CONSTRAINT fk_share_transactions_seller_id 
    FOREIGN KEY (seller_id) REFERENCES users(id) ON DELETE SET NULL;

-- Revenue Distributions
ALTER TABLE revenue_distributions 
    ADD CONSTRAINT fk_revenue_distributions_contract_id 
    FOREIGN KEY (contract_id) REFERENCES ownership_contracts(id) ON DELETE RESTRICT;

-- Campaigns
ALTER TABLE campaigns 
    ADD CONSTRAINT fk_campaigns_song_id 
    FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE RESTRICT;

ALTER TABLE campaigns 
    ADD CONSTRAINT fk_campaigns_artist_id 
    FOREIGN KEY (artist_id) REFERENCES users(id) ON DELETE RESTRICT;

-- NFT Purchases
ALTER TABLE nft_purchases 
    ADD CONSTRAINT fk_nft_purchases_campaign_id 
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE RESTRICT;

ALTER TABLE nft_purchases 
    ADD CONSTRAINT fk_nft_purchases_user_id 
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT;

-- Campaign Analytics
ALTER TABLE campaign_analytics 
    ADD CONSTRAINT fk_campaign_analytics_campaign_id 
    FOREIGN KEY (campaign_id) REFERENCES campaigns(id) ON DELETE CASCADE;

-- Listen Sessions
ALTER TABLE listen_sessions 
    ADD CONSTRAINT fk_listen_sessions_user_id 
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT;

ALTER TABLE listen_sessions 
    ADD CONSTRAINT fk_listen_sessions_song_id 
    FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE CASCADE;

ALTER TABLE listen_sessions 
    ADD CONSTRAINT fk_listen_sessions_artist_id 
    FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE RESTRICT;

-- User Reward History
ALTER TABLE user_reward_history 
    ADD CONSTRAINT fk_user_reward_history_user_id 
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT;

ALTER TABLE user_reward_history 
    ADD CONSTRAINT fk_user_reward_history_session_id 
    FOREIGN KEY (session_id) REFERENCES listen_sessions(id) ON DELETE SET NULL;

ALTER TABLE user_reward_history 
    ADD CONSTRAINT fk_user_reward_history_distribution_id 
    FOREIGN KEY (distribution_id) REFERENCES reward_distributions(id) ON DELETE SET NULL;

-- Notifications
ALTER TABLE notifications 
    ADD CONSTRAINT fk_notifications_user_id 
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE RESTRICT;

-- Fan Verifications
ALTER TABLE fan_verifications 
    ADD CONSTRAINT fk_fan_verifications_fan_id 
    FOREIGN KEY (fan_id) REFERENCES users(id) ON DELETE RESTRICT;

-- NFT Wristbands
ALTER TABLE nft_wristbands 
    ADD CONSTRAINT fk_nft_wristbands_fan_id 
    FOREIGN KEY (fan_id) REFERENCES users(id) ON DELETE RESTRICT;
```

## ✅ Checklist de Integridad

### Foreign Keys Verificados
- [x] Todas las tablas tienen primary keys definidos
- [ ] **Todas las foreign keys tienen referencias válidas** ⚠️ FALTAN 17 foreign keys
- [x] ON DELETE está definido para cada foreign key
- [x] Índices creados para todas las foreign keys

### Constraints Verificados
- [x] Unique constraints definidos donde es necesario
- [x] Check constraints para validación de datos
- [x] NOT NULL constraints en campos críticos

### Performance
- [x] Índices en foreign keys
- [x] Índices en campos de búsqueda frecuente
- [x] Índices compuestos para queries comunes

### Triggers
- [x] Triggers de `updated_at` en todas las tablas relevantes
- [x] Triggers de negocio (tier updates, play counts, etc.)

---

## 📝 Notas Importantes

1. **CASCADE vs SET NULL**: 
   - CASCADE se usa cuando la relación es fuerte (ej: artista → canciones)
   - SET NULL se usa cuando se quiere mantener historial (ej: payments → users)

2. **Junction Tables**: 
   - Siempre tienen UNIQUE constraint en la combinación de foreign keys
   - Ejemplos: `playlist_songs`, `user_shares`, `fan_ventures`

3. **Event Sourcing**: 
   - `payment_events` no usa foreign keys tradicionales
   - Usa `aggregate_id` para referenciar entidades

4. **Soft Deletes**: 
   - Algunas tablas usan `status` en lugar de DELETE físico
   - Ejemplo: `listen_sessions.status = 'deleted'`

---

> **Última actualización**: Diciembre 2024  
> **Próxima revisión**: Al agregar nuevas tablas o modificar relaciones

