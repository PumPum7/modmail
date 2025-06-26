# Modmail System

A comprehensive modmail system for Discord servers, enabling users to communicate privately with server moderators through direct messages.

## Architecture Overview

The system consists of three main components:

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Discord Bot   │    │   Backend API    │    │  Web Dashboard  │
│   (TypeScript)  │◄──►│     (Rust)       │◄──►│  (SvelteKit)    │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                        │                        │
         ▼                        ▼                        ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│  Discord API    │    │   PostgreSQL     │    │ Discord OAuth   │
│                 │    │    Database      │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

### Components

1. **[Discord Bot](./discord/README.md)** - Handles DMs, creates threads, manages Discord channels
2. **[Backend API](./backend/README.md)** - REST API server for data persistence and management
3. **[Web Dashboard](./frontend/README.md)** - Moderator interface for thread and macro management

### Data Flow

1. **User Interaction**: User sends DM to Discord bot
2. **Thread Creation**: Bot creates thread in backend and Discord channel
3. **Message Relay**: Messages flow bidirectionally between user and moderators
4. **Web Management**: Moderators use dashboard to view history and manage macros
5. **Thread Closure**: Moderators close threads via bot commands or web interface

## Quick Start

1. **Database Setup**:
   ```bash
   # Start PostgreSQL with Docker
   docker-compose up -d postgres
   ```

2. **Backend**:
   ```bash
   cd backend
   cargo run
   ```

3. **Discord Bot**:
   ```bash
   cd discord
   bun install
   bun run deploy-commands.ts
   bun run index.ts
   ```

4. **Frontend**:
   ```bash
   cd frontend
   bun install
   bun run dev
   ```

## Environment Configuration

Each component requires specific environment variables. See individual package READMEs for detailed configuration:

- [Backend Environment Variables](./backend/README.md#setup)
- [Discord Bot Environment Variables](./discord/README.md#environment-variables)
- [Frontend Environment Variables](./frontend/README.md#environment-variables)

## Docker Deployment

The system includes Docker configurations for containerized deployment:

```bash
docker-compose up -d
```

This starts all services with proper networking and database connections.

## Features

- **Seamless Communication**: Users DM the bot, moderators respond in dedicated channels
- **Thread Management**: Automatic thread creation and organization
- **Message History**: Complete conversation logs accessible via web dashboard
- **Macro System**: Predefined responses for common inquiries
- **Role-based Access**: Discord role integration for moderator permissions
- **Real-time Updates**: Live message relay between Discord and web interface

## Technology Stack

- **Backend**: Rust, Actix Web, SQLx, PostgreSQL
- **Discord Bot**: TypeScript, discord.js, Bun
- **Frontend**: SvelteKit, TypeScript, Vite
- **Database**: PostgreSQL with SQLx migrations
- **Deployment**: Docker, Docker Compose