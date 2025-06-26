# Discord Modmail System

A complete Discord modmail system with a Rust backend, Discord bot, and web dashboard for moderators.

## Overview

This system consists of three main components:

1. **Backend (Rust)** - API server with PostgreSQL database
2. **Discord Bot (TypeScript)** - Handles DM conversations and slash commands  
3. **Web Dashboard (SvelteKit)** - Frontend for moderators with Discord OAuth

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Discord Bot   │    │   Rust Backend   │    │  Web Dashboard  │
│   (TypeScript)  │◄──►│     (API)        │◄──►│   (SvelteKit)   │
└─────────────────┘    └──────────────────┘    └─────────────────┘
        │                        │                        │
        ▼                        ▼                        ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│  Discord API    │    │   PostgreSQL     │    │ Discord OAuth   │
│                 │    │   Database       │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

## Features

### Discord Bot
- **DM to Channel Creation**: Automatically creates channels for new conversations
- **Message Relay**: Forwards messages between users and moderators
- **Slash Commands**: `/message`, `/close`, `/macro` (moderator-only)
- **Thread Management**: Creates and manages modmail threads
- **Macro System**: Quick response templates

### Web Dashboard
- **Discord OAuth Authentication**: Role-based access control
- **Thread Overview**: View all active and closed threads
- **Message History**: Complete conversation logs
- **Macro Management**: Create, edit, delete response templates
- **Real-time Updates**: Live data from the backend API

### Backend API
- **RESTful API**: Full CRUD operations for threads, messages, macros
- **Database Persistence**: PostgreSQL with migrations
- **Thread Management**: Track conversation state and history
- **Message Storage**: Complete audit trail of all interactions

## Quick Start

### Prerequisites

- PostgreSQL database
- Discord bot application
- Bun runtime
- Rust toolchain

### 1. Database Setup

```sql
-- Create database
CREATE DATABASE modmail;

-- Run migrations (automatically handled by the backend)
```

### 2. Backend Setup

```bash
cd backend
cp .env.example .env
# Configure DATABASE_URL in .env
cargo run
```

The backend will run on `http://localhost:8080`

### 3. Discord Bot Setup

```bash
cd discord
cp .env.example .env
# Configure Discord tokens and settings in .env
bun install
bun run deploy  # Deploy slash commands
bun run index.ts
```

### 4. Web Dashboard Setup

```bash
cd web
cp .env.example .env
# Configure Discord OAuth and API settings in .env
chmod +x setup.sh
./setup.sh
bun run dev
```

The dashboard will be available at `http://localhost:3000`

## Configuration

### Environment Variables

#### Backend (`backend/.env`)
```env
DATABASE_URL=postgresql://user:password@localhost/modmail
```

#### Discord Bot (`discord/.env`)
```env
DISCORD_BOT_TOKEN=your_bot_token
PUBLIC_DISCORD_CLIENT_ID=your_client_id
DISCORD_SERVER_ID=your_server_id
MODMAIL_CATEGORY_ID=your_category_id
MOD_ROLE_IDS=role1,role2,role3
BACKEND_URL=http://localhost:8080
```

#### Web Dashboard (`web/.env`)
```env
PUBLIC_DISCORD_CLIENT_ID=your_client_id
DISCORD_CLIENT_SECRET=your_client_secret
PUBLIC_DISCORD_REDIRECT_URI=http://localhost:3000/auth/callback
PUBLIC_BACKEND_URL=http://localhost:8080
PUBLIC_MOD_ROLE_IDS=role1,role2,role3
PUBLIC_DISCORD_SERVER_ID=your_server_id
```

## Usage

### For Users
1. Send a DM to the Discord bot
2. Your message creates a new modmail thread
3. Continue the conversation through DMs
4. Receive responses from moderators

### For Moderators

#### Discord Commands
- `/message <user_id> <content>` - Send message to user
- `/close` - Close current thread
- `/macro create <name> <content>` - Create macro
- `/macro send <name>` - Send macro
- `/macro delete <name>` - Delete macro

#### Web Dashboard
- **Login**: Authenticate with Discord
- **Threads**: View all conversations and their status
- **Messages**: Search through all message history
- **Macros**: Manage response templates
- **Real-time**: Live updates of new messages and threads

## Development

### Project Structure

```
modmail/
├── backend/          # Rust API server
│   ├── src/
│   ├── migrations/
│   └── Cargo.toml
├── discord/          # Discord bot
│   ├── index.ts
│   ├── deploy-commands.ts
│   └── package.json
├── web/              # Frontend dashboard
│   ├── src/
│   ├── static/
│   └── package.json
└── docker-compose.yml
```

### API Endpoints

- `GET /messages` - All messages
- `POST /messages` - Create message
- `GET /threads` - All threads  
- `POST /threads` - Create thread
- `GET /threads/{id}` - Thread with messages
- `POST /threads/{id}/close` - Close thread
- `POST /threads/{id}/messages` - Add message to thread
- `GET /macros` - All macros
- `POST /macros` - Create macro
- `GET /macros/{name}` - Get macro
- `DELETE /macros/{name}` - Delete macro

### Database Schema

```sql
-- Threads table
CREATE TABLE threads (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    thread_id VARCHAR(255) NOT NULL,
    is_open BOOLEAN NOT NULL DEFAULT TRUE
);

-- Messages table  
CREATE TABLE messages (
    id UUID PRIMARY KEY,
    author_id VARCHAR(255) NOT NULL,
    author_tag VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

-- Thread-Message relationship
CREATE TABLE thread_messages (
    thread_id INTEGER REFERENCES threads(id),
    message_id UUID REFERENCES messages(id),
    PRIMARY KEY (thread_id, message_id)
);

-- Macros table
CREATE TABLE macros (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    content TEXT NOT NULL
);
```

## Deployment

### Docker Compose

```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

### Production Considerations

1. **Environment Variables**: Use production values
2. **Database**: Configure PostgreSQL with proper credentials
3. **Discord OAuth**: Update redirect URIs for production domain
4. **HTTPS**: Enable SSL for web dashboard
5. **Monitoring**: Add logging and health checks

## Security

- **Authentication**: Discord OAuth with role verification
- **Authorization**: Role-based access control
- **Data Protection**: HTTP-only cookies, CSRF protection
- **API Security**: Input validation and sanitization
- **Database**: Prepared statements, connection pooling

## Troubleshooting

### Common Issues

1. **Bot not responding**: Check bot token and permissions
2. **Dashboard login fails**: Verify OAuth configuration
3. **API errors**: Ensure backend is running and accessible
4. **Database connection**: Check PostgreSQL credentials and connectivity

### Debug Mode

Enable debug logging by setting:
```env
RUST_LOG=debug          # Backend
NODE_ENV=development    # Frontend
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License.

## Support

For issues and questions:
1. Check the documentation in each component's README
2. Review the troubleshooting section
3. Create an issue on GitHub with relevant logs and configuration 