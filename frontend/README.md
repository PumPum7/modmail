# Modmail Frontend Dashboard

A web dashboard for moderators to manage Discord modmail threads with Discord OAuth authentication.

## Features

- **Discord OAuth Authentication**: Moderators log in with their Discord accounts
- **Role-based Access**: Only users with predefined moderator roles can access the dashboard
- **Thread Management**: View all modmail threads, see their status, and close them
- **Message History**: View complete conversation history for each thread
- **Macro Management**: Create, edit, and delete response macros
- **Real-time Data**: Live updates of thread status and messages
- **Responsive Design**: Works on desktop and mobile devices

## Setup

### 1. Install Dependencies

```bash
cd web
bun install
```

### 2. Environment Configuration

Create a `.env` file in the `web/` directory:

```env
# Discord OAuth Configuration
PUBLIC_DISCORD_CLIENT_ID=your_client_id_here
DISCORD_CLIENT_SECRET=your_client_secret_here
PUBLIC_DISCORD_REDIRECT_URI=http://localhost:3000/auth/callback

# Backend API
PUBLIC_BACKEND_URL=http://localhost:8080

# Moderator Role IDs (comma-separated)
PUBLIC_MOD_ROLE_IDS=role_id_1,role_id_2,role_id_3

# Server ID
PUBLIC_DISCORD_SERVER_ID=your_server_id_here
```

### 3. Discord Application Setup

1. Go to the [Discord Developer Portal](https://discord.com/developers/applications)
2. Select your existing application (the same one used for the bot)
3. In the "OAuth2" section, add redirect URI: `http://localhost:3000/auth/callback`
4. Copy the client secret to `DISCORD_CLIENT_SECRET`
5. Ensure the application has the `identify`, `email`, and `guilds.members.read` scopes

### 4. Start the Development Server

```bash
bun run dev
```

The frontend will be available at `http://localhost:3000`

## Production Deployment

### 1. Build the Application

```bash
bun run build
```

### 2. Environment Variables

Update your production environment variables:

- Set `PUBLIC_DISCORD_REDIRECT_URI` to your production domain
- Update `PUBLIC_BACKEND_URL` to your production backend URL
- Ensure HTTPS is enabled for production

### 3. Start Production Server

```bash
bun run preview
```

## Usage

### Authentication Flow

1. Visit the dashboard URL
2. Click "Continue with Discord"
3. Authorize the application with Discord
4. If you have the required moderator role, you'll be redirected to the dashboard
5. If not, you'll see an error message

### Dashboard Features

#### Threads Page (`/`)

- View all modmail threads with their status
- See user information and thread details
- Close open threads directly from the list
- Click "View Messages" to see the full conversation

#### Thread Details (`/thread/{id}`)

- View complete message history
- See timestamps and author information
- Send new messages to the user
- Close the thread

#### Messages Page (`/messages`)

- View all messages across all threads
- Filter and search functionality
- Export message history

#### Macros Page (`/macros`)

- Create new response macros
- Edit existing macros
- Delete unused macros
- Preview macro content

## API Integration

The frontend communicates with the Rust backend API:

- `GET /threads` - Fetch all threads
- `GET /threads/{id}` - Fetch specific thread with messages
- `POST /threads/{id}/close` - Close a thread
- `POST /threads/{id}/messages` - Add message to thread
- `GET /messages` - Fetch all messages
- `GET /macros` - Fetch all macros
- `POST /macros` - Create new macro
- `DELETE /macros/{name}` - Delete macro

## Security

- All API requests are authenticated
- Session cookies are HTTP-only and secure
- CSRF protection via SameSite cookies
- Role verification on every protected route
- No sensitive data stored in client-side storage

## Development

### File Structure

```
web/
├── src/
│   ├── lib/
│   │   ├── auth.ts         # Discord OAuth utilities
│   │   └── api.ts          # Backend API client
│   ├── routes/
│   │   ├── +layout.svelte  # Main layout with navigation
│   │   ├── +page.svelte    # Threads dashboard
│   │   ├── auth/           # Authentication routes
│   │   ├── thread/         # Thread detail pages
│   │   ├── messages/       # Messages page
│   │   └── macros/         # Macros management
│   ├── app.html           # HTML template
│   └── app.d.ts           # Type definitions
├── package.json
├── svelte.config.js
├── vite.config.js
└── tsconfig.json
```

### Adding New Features

1. Create new routes in `src/routes/`
2. Add API methods to `src/lib/api.ts`
3. Update types in `src/app.d.ts`
4. Add navigation links in `src/routes/+layout.svelte`

### Styling

- Uses vanilla CSS with CSS custom properties
- Follows Discord's design language
- Responsive design with CSS Grid and Flexbox
- Dark/light theme ready (currently light theme)

### Debug Mode

Add to your `.env` file:

```env
NODE_ENV=development
```

This enables additional logging and error details.
