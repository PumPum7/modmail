# Discord Modmail Bot

A Discord modmail bot that integrates with a Rust backend to manage user support tickets through Discord DMs.

## Features

- **DM to Channel**: When users send DMs to the bot, it creates a dedicated channel in a specified category
- **Slash Commands** (Moderator only):
  - `/message <user> <message>` - Send a message to a specific user
  - `/close` - Close the current modmail thread
  - `/macro create <name> <content>` - Create a new macro
  - `/macro send <name>` - Send a macro in the current thread
  - `/macro delete <name>` - Delete an existing macro
- **Message Relay**: Messages sent in modmail channels are relayed to users via DM
- **Thread Management**: Automatic thread creation and closure with database persistence
- **Permission System**: Only moderators with specified roles can use commands

## Setup

### 1. Environment Variables

Create a `.env` file in the `discord/` directory with the following variables:

```env
# Discord Bot Configuration
DISCORD_BOT_TOKEN=your_bot_token_here
PUBLIC_DISCORD_CLIENT_ID=your_client_id_here
DISCORD_SERVER_ID=your_server_id_here

# Modmail Configuration
MODMAIL_CATEGORY_ID=your_category_id_here
MOD_ROLE_IDS=role_id_1,role_id_2,role_id_3

# Backend API Configuration
BACKEND_URL=http://localhost:8080
```

### 2. Discord Bot Setup

1. Go to the [Discord Developer Portal](https://discord.com/developers/applications)
2. Create a new application and bot
3. Copy the bot token to `DISCORD_BOT_TOKEN`
4. Copy the application ID to `PUBLIC_DISCORD_CLIENT_ID`
5. Enable the following bot permissions:
   - Send Messages
   - Manage Channels
   - Read Message History
   - Embed Links
   - Add Reactions
6. Enable the following intents:
   - Server Members Intent
   - Message Content Intent

### 3. Server Setup

1. Invite the bot to your Discord server with the required permissions
2. Create a category for modmail threads and copy its ID to `MODMAIL_CATEGORY_ID`
3. Get the role IDs for moderators and add them to `MOD_ROLE_IDS` (comma-separated)
4. Copy your server ID to `DISCORD_SERVER_ID`

### 4. Backend Setup

Make sure the Rust backend is running on the specified `BACKEND_URL`. The backend should have the following endpoints:

- `GET /threads` - Get all threads
- `POST /threads` - Create a new thread
- `POST /threads/{id}/close` - Close a thread
- `POST /threads/{id}/messages` - Add message to thread
- `GET /macros` - Get all macros
- `POST /macros` - Create a new macro
- `GET /macros/{name}` - Get macro by name
- `DELETE /macros/{name}` - Delete a macro

### 5. Deploy Commands

Run the following command to register slash commands:

```bash
bun run deploy
```

### 6. Start the Bot

```bash
bun run index.ts
```

## Usage

### For Users

1. Send a direct message to the bot
2. The bot will create a modmail thread and notify moderators
3. Continue the conversation through DMs
4. Receive responses from moderators via the bot

### For Moderators

#### Slash Commands

- **Message a user**: `/message <user_id> <message_content>`
  - Sends a direct message to the specified user
  - Creates a thread if one doesn't exist

- **Close thread**: `/close`
  - Closes the current modmail thread
  - Notifies the user that the thread has been closed
  - Can only be used in modmail channels

- **Create macro**: `/macro create <name> <content>`
  - Creates a reusable macro with the specified name and content
  - Useful for common responses

- **Send macro**: `/macro send <name>`
  - Sends a pre-created macro to the user
  - Can only be used in modmail channels

- **Delete macro**: `/macro delete <name>`
  - Removes an existing macro

#### Direct Messaging

- Type messages directly in modmail channels (non-slash commands)
- Messages will be automatically relayed to the user
- Bot will react with ✅ when message is sent successfully
- Bot will react with ❌ if there's an error

## Database Schema

The bot integrates with these database tables:

- `threads` - Stores modmail thread information
- `messages` - Stores all messages in the system
- `thread_messages` - Links messages to threads
- `macros` - Stores reusable message templates

## Permissions

Only users with roles specified in `MOD_ROLE_IDS` can:
- Use slash commands
- Have their messages relayed to users in modmail channels

Regular users can only initiate conversations through DMs.

## Error Handling

The bot includes comprehensive error handling:
- Failed API calls are logged and user-friendly errors are shown
- Permission errors are handled gracefully
- Database connection issues are logged
- Unknown users or channels are handled appropriately

## Development

To modify the bot:

1. Edit `index.ts` for bot logic
2. Edit `deploy-commands.ts` for slash command definitions
3. Run `bun run deploy` after changing commands
4. Restart the bot to apply changes
