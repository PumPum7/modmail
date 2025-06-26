# Discord Bot - Modmail Bot

A Discord bot that handles direct messages from users and facilitates communication between users and server moderators through a modmail system.

## Technologies Used

- **Bun** - Fast JavaScript runtime and package manager
- **TypeScript** - Type-safe JavaScript development
- **discord.js** - Primary Discord API library
- **@discordjs/builders** - Discord slash command builders
- **@discordjs/rest** - Discord REST API client
- **pg** - PostgreSQL client for database operations
- **dotenv** - Environment variable management
- **UUID** - Unique identifier generation

## Architecture

The bot operates as a bridge between Discord users and the modmail system:

### Core Functionality

1. **DM Handling** - Receives direct messages from users and creates modmail threads
2. **Thread Management** - Creates dedicated channels for each modmail conversation
3. **Message Relay** - Forwards messages between users and moderators bidirectionally
4. **Slash Commands** - Provides moderator commands for thread management

### Slash Commands

- `/message <user> <content>` - Send a message to a user (creates thread if needed)
- `/close [reason]` - Close the current modmail thread
- `/macro <name> [user]` - Send a predefined macro response
- `/delete <count>` - Delete recent messages from the thread

### Data Flow

1. User sends DM to bot → Bot creates thread in backend → Bot creates Discord channel
2. User sends follow-up DM → Bot adds message to existing thread → Bot forwards to Discord channel
3. Moderator responds in channel → Bot sends message to user → Bot stores in backend
4. Moderator uses `/close` → Bot closes thread in backend → Bot archives Discord channel

### Integration

- Communicates with **Backend API** for persistent storage
- Manages Discord channels and permissions automatically
- Handles user authentication and moderator role verification

## Setup

1. Install Bun
2. Create Discord application and bot
3. Configure environment variables
4. Deploy slash commands: `bun run deploy-commands.ts`
5. Start bot: `bun run index.ts`

## Environment Variables

- `DISCORD_TOKEN` - Bot token from Discord Developer Portal
- `DISCORD_CLIENT_ID` - Application ID from Discord Developer Portal
- `GUILD_ID` - Discord server ID where bot operates
- `BACKEND_URL` - URL of the Rust backend API
- Database connection variables for PostgreSQL
