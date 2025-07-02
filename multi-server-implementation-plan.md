# Multi-Server Modmail Implementation Plan

## Phase 1: Database Schema Updates

### 1.1 Add guild_id to all tables
```sql
-- Add guild_id column to all main tables
ALTER TABLE threads ADD COLUMN guild_id VARCHAR(255) NOT NULL DEFAULT 'default';
ALTER TABLE messages ADD COLUMN guild_id VARCHAR(255) NOT NULL DEFAULT 'default';
ALTER TABLE macros ADD COLUMN guild_id VARCHAR(255) NOT NULL DEFAULT 'default';
ALTER TABLE notes ADD COLUMN guild_id VARCHAR(255) NOT NULL DEFAULT 'default';
ALTER TABLE blocked_users ADD COLUMN guild_id VARCHAR(255) NOT NULL DEFAULT 'default';
```

### 1.2 Update constraints and indexes
```sql
-- Update unique constraints to include guild_id
ALTER TABLE threads DROP CONSTRAINT IF EXISTS idx_threads_user_open;
CREATE UNIQUE INDEX idx_threads_user_guild_open ON threads (user_id, guild_id) WHERE is_open = true;

-- Add guild-specific indexes for performance
CREATE INDEX idx_threads_guild_id ON threads (guild_id);
CREATE INDEX idx_messages_guild_id ON messages (guild_id);
CREATE INDEX idx_macros_guild_id ON macros (guild_id);
```

### 1.3 Create servers table
```sql
CREATE TABLE servers (
    id SERIAL PRIMARY KEY,
    guild_id VARCHAR(255) NOT NULL UNIQUE,
    guild_name VARCHAR(255) NOT NULL,
    is_premium BOOLEAN NOT NULL DEFAULT FALSE,
    max_threads INTEGER DEFAULT 50,
    max_macros INTEGER DEFAULT 10,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### 1.4 Create guild configuration table
```sql
CREATE TABLE guild_configs (
    id SERIAL PRIMARY KEY,
    guild_id VARCHAR(255) NOT NULL UNIQUE,
    modmail_category_id VARCHAR(255),
    log_channel_id VARCHAR(255),
    randomize_names BOOLEAN DEFAULT FALSE,
    auto_close_hours INTEGER DEFAULT NULL,
    welcome_message TEXT DEFAULT NULL,
    moderator_role_ids TEXT[] DEFAULT '{}',
    blocked_words TEXT[] DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## Phase 2: Backend API Updates

### 2.1 Update all API endpoints to filter by guild_id
- Modify all queries in `threads.rs`, `messages.rs`, `macros.rs`, `notes.rs`, `blocked_users.rs`
- Add guild_id parameter to all create/update operations
- Update analytics to be guild-specific

### 2.2 Add server management endpoints
```rust
// New endpoints in servers.rs
GET /servers - List user's accessible servers
POST /servers - Register new server
GET /servers/{guild_id} - Get server details
PUT /servers/{guild_id} - Update server settings
DELETE /servers/{guild_id} - Remove server
```

### 2.3 Add guild configuration endpoints
```rust
// New endpoints in guild_configs.rs
GET /guilds/{guild_id}/config - Get guild configuration
PUT /guilds/{guild_id}/config - Update guild configuration
POST /guilds/{guild_id}/config - Create initial guild configuration
```

### 2.4 Update structs to include guild_id
```rust
// Update all Create* structs in structs.rs
pub struct CreateThread {
    pub user_id: String,
    pub thread_id: String,
    pub guild_id: String,
    pub urgency: Option<String>,
}

// Add new config struct
pub struct GuildConfig {
    pub guild_id: String,
    pub modmail_category_id: Option<String>,
    pub log_channel_id: Option<String>,
    pub randomize_names: bool,
    pub auto_close_hours: Option<i32>,
    pub welcome_message: Option<String>,
    pub moderator_role_ids: Vec<String>,
    pub blocked_words: Vec<String>,
}
```

## Phase 3: Discord Bot Multi-Server Support

### 3.1 Update bot to handle multiple servers
- Modify `dmHandler.ts` to show server selection dropdown for users in multiple servers
- Update all API calls to include guild_id
- Replace environment variable reads with database config lookups

### 3.2 Server selection in DMs
```typescript
// Add server selection before intro modal
if (userServers.length > 1) {
    const serverSelectMenu = new StringSelectMenuBuilder()
        .setCustomId(`select_server_${userId}`)
        .setPlaceholder('Choose a server to contact')
        .addOptions(userServers.map(server => ({
            label: server.name,
            value: server.id,
            description: `Contact ${server.name} moderators`
        })));
}
```

### 3.3 Add configuration slash commands
```typescript
// New /config command with subcommands
/config set modmail-category <channel>
/config set log-channel <channel>
/config set randomize-names <true/false>
/config set auto-close-hours <number>
/config set welcome-message <text>
/config add moderator-role <role>
/config remove moderator-role <role>
/config add blocked-word <word>
/config remove blocked-word <word>
/config show - Display current configuration
/config reset - Reset to defaults
```

### 3.4 Replace environment variables with database lookups
- Create `getGuildConfig(guildId)` function to fetch configuration
- Update all hardcoded environment variable references
- Add configuration caching for performance

## Phase 4: Frontend Multi-Server Interface

### 4.1 Update authentication to handle multiple servers
- Modify auth flow to check user's servers via Discord API
- Store accessible servers in JWT/session
- Add server switching in navigation

### 4.2 Add server selector component
```svelte
<!-- ServerSelector.svelte -->
<select bind:value={selectedServer}>
    {#each userServers as server}
        <option value={server.guild_id}>{server.name}</option>
    {/each}
</select>
```

### 4.3 Update all API calls to include guild_id
- Modify `lib/api.ts` to accept guild_id parameter
- Update all route handlers to filter by selected server
- Add server context to all pages

### 4.4 Add guild configuration management interface
- Server settings page with configuration options
- Real-time validation of Discord IDs (channels, roles)
- Configuration history and audit log

## Phase 5: Premium Features Implementation

### 5.1 Create subscription system
```sql
CREATE TABLE subscriptions (
    id SERIAL PRIMARY KEY,
    guild_id VARCHAR(255) NOT NULL UNIQUE,
    plan_type VARCHAR(50) NOT NULL DEFAULT 'free',
    max_servers INTEGER DEFAULT 1,
    max_threads_per_server INTEGER DEFAULT 50,
    max_macros_per_server INTEGER DEFAULT 10,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### 5.2 Premium feature gates
- Server limit enforcement (free = 1 server, premium = unlimited)
- Advanced analytics (retention, custom reports)
- Priority support channels
- Custom branding/themes
- Advanced automation rules

### 5.3 Billing integration
- Stripe/payment processor integration
- Subscription management interface
- Usage tracking and limits enforcement

## Phase 6: Migration Strategy

### 6.1 Data migration for existing installations
```sql
-- Migration script to set default guild_id for existing data
UPDATE threads SET guild_id = '330300161895038987' WHERE guild_id = 'default';
UPDATE messages SET guild_id = '330300161895038987' WHERE guild_id = 'default';
-- ... repeat for all tables

-- Create initial guild configuration from environment variables
INSERT INTO guild_configs (guild_id, modmail_category_id, randomize_names)
VALUES ('330300161895038987', '482421820734177290', false);
```

### 6.2 Configuration migration
- Migrate existing environment variables to database
- Update Discord bot to use database configuration
- Provide migration script for existing installations

## Phase 7: Additional Features

### 7.1 Server management dashboard
- Server settings page
- Moderator role management per server
- Server-specific customization options

### 7.2 Cross-server features (Premium)
- Global user blocking across servers
- Shared macro libraries
- Cross-server analytics

### 7.3 Advanced automation
- Server-specific auto-responders
- Custom workflow rules per server
- Integration webhooks per server

### 7.4 Configuration management enhancements
- Configuration templates for quick setup
- Bulk configuration changes across servers
- Configuration backup and restore
- Real-time configuration validation
- Configuration change notifications

## Implementation Order

1. **Database migrations** (Phase 1) - Foundation for everything
2. **Backend API updates** (Phase 2) - Core functionality
3. **Discord bot updates** (Phase 3) - User-facing bot features
4. **Frontend updates** (Phase 4) - Dashboard interface
5. **Premium features** (Phase 5) - Monetization
6. **Migration tools** (Phase 6) - Existing user support
7. **Advanced features** (Phase 7) - Enhancement and differentiation

## Key Considerations

- **Backward compatibility**: Ensure existing single-server setups continue working
- **Performance**: Guild-specific indexes and query optimization
- **Security**: Proper authorization checks for guild access
- **User experience**: Smooth server selection and switching
- **Scalability**: Design for hundreds of servers per installation
- **Configuration management**: Easy setup and maintenance via bot commands
- **Validation**: Ensure Discord IDs are valid and accessible
- **Caching**: Cache frequently accessed configuration for performance