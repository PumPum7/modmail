# Frontend - Modmail Dashboard

A web-based dashboard for moderators to manage modmail threads, view message history, and configure macros.

## Technologies Used

- **SvelteKit** - Full-stack web framework with SSR/SPA capabilities
- **TypeScript** - Type-safe JavaScript development
- **Vite** - Fast build tool and development server
- **Tailwind CSS** - Utility-first CSS framework (implied from styling)
- **Lucide Svelte** - Icon library for UI components
- **Discord API Types** - TypeScript definitions for Discord API
- **Cookie** - Cookie parsing utilities

## Architecture

The dashboard provides a web interface for moderator workflow management:

### Core Features

1. **Authentication** - Discord OAuth integration for moderator login
2. **Thread Management** - View active and closed modmail conversations
3. **Message History** - Browse complete conversation threads
4. **Macro Management** - Create, edit, and delete response templates
5. **Real-time Updates** - Live view of ongoing conversations

### Page Structure

- `/` - Dashboard home with thread overview
- `/messages` - All messages across threads
- `/thread/{id}` - Individual thread view with full message history
- `/macros` - Macro management interface
- `/login` - Discord OAuth authentication

### API Integration

The frontend communicates with the **Backend API** through a centralized API client (`lib/api.ts`):

- Thread operations (list, view, close, add messages)
- Message retrieval and management
- Macro CRUD operations
- Authentication state management

### Authentication Flow

1. User visits dashboard → Redirected to Discord OAuth
2. Discord callback → Exchange code for tokens
3. Fetch user info → Verify moderator permissions
4. Create JWT session → Store in HTTP-only cookie
5. Subsequent requests → Validate JWT for API access

### Server-Side Rendering

- Uses SvelteKit's SSR capabilities for initial page loads
- Server-side data fetching in `+page.server.ts` files
- Client-side hydration for interactive features
- Handles authentication state on both server and client

## Setup

1. Install Bun (or Node.js)
2. Install dependencies: `bun install`
3. Configure environment variables
4. Start development server: `bun run dev`
5. Build for production: `bun run build`

## Environment Variables

- `DISCORD_CLIENT_ID` - Discord application client ID
- `DISCORD_CLIENT_SECRET` - Discord application client secret
- `DISCORD_REDIRECT_URI` - OAuth callback URL
- `JWT_SECRET` - Secret key for JWT token signing
- `BACKEND_URL` - URL of the Rust backend API
- `GUILD_ID` - Discord server ID for permission checks

## Development

The dashboard runs on port 5173 in development mode with hot module replacement. It proxies API requests to the backend server and handles CORS appropriately.
