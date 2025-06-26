# Backend - Modmail API Server

A Rust-based REST API server that provides the core data layer for the modmail system.

## Technologies Used

- **Rust** - Systems programming language for performance and safety
- **Actix Web** - High-performance web framework for building HTTP APIs
- **SQLx** - Async SQL toolkit with compile-time checked queries
- **PostgreSQL** - Primary database for persistent storage
- **Serde** - Serialization/deserialization framework
- **Chrono** - Date and time handling
- **UUID** - Unique identifier generation
- **dotenv** - Environment variable management

## Architecture

The backend serves as the central data layer, exposing RESTful endpoints for:

- **Threads** - Modmail conversation management
- **Messages** - Individual message storage and retrieval
- **Macros** - Predefined response templates

### Database Schema

- `messages` - All messages (both user DMs and moderator responses)
- `threads` - Modmail conversation threads
- `macros` - Reusable message templates
- `thread_messages` - Junction table linking messages to threads

### API Endpoints

- `GET /messages` - Retrieve all messages
- `GET /threads` - List all threads
- `GET /threads/{id}` - Get specific thread with messages
- `POST /threads/{id}/messages` - Add message to thread
- `POST /threads/{id}/close` - Close a thread
- `GET /macros` - List all macros
- `POST /macros` - Create new macro
- `PUT /macros/{name}` - Update existing macro
- `DELETE /macros/{name}` - Delete macro

## Setup

1. Install Rust and Cargo
2. Set up PostgreSQL database
3. Configure environment variables in `.env`
4. Run migrations: `sqlx migrate run`
5. Start server: `cargo run`

The server runs on port 8080 by default and provides CORS support for the frontend dashboard.
