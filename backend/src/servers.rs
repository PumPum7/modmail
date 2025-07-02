use crate::structs::{CreateServer, Server, UpdateServer};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

// Route builder
pub fn server_routes(db_pool: PgPool) -> Router {
    Router::new()
        .route("/servers", post(create_server).get(get_servers))
        .route(
            "/servers/:guild_id",
            get(get_server).put(update_server).delete(delete_server),
        )
        .with_state(db_pool)
}

// Get all servers (for a user, you might add authentication and filtering here)
async fn get_servers(State(db_pool): State<PgPool>) -> Result<Json<Vec<Server>>, StatusCode> {
    match sqlx::query_as::<_, Server>("SELECT * FROM servers")
        .fetch_all(&db_pool)
        .await
    {
        Ok(servers) => Ok(Json(servers)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Create a new server
async fn create_server(
    State(db_pool): State<PgPool>,
    Json(payload): Json<CreateServer>,
) -> Result<Json<Server>, StatusCode> {
    match sqlx::query_as::<_, Server>(
        "INSERT INTO servers (guild_id, guild_name) VALUES ($1, $2) RETURNING *",
    )
    .bind(payload.guild_id)
    .bind(payload.guild_name)
    .fetch_one(&db_pool)
    .await
    {
        Ok(server) => Ok(Json(server)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Get a specific server by guild_id
async fn get_server(
    State(db_pool): State<PgPool>,
    Path(guild_id): Path<String>,
) -> Result<Json<Server>, StatusCode> {
    match sqlx::query_as::<_, Server>("SELECT * FROM servers WHERE guild_id = $1")
        .bind(guild_id)
        .fetch_optional(&db_pool)
        .await
    {
        Ok(Some(server)) => Ok(Json(server)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Update a server's settings
async fn update_server(
    State(db_pool): State<PgPool>,
    Path(guild_id): Path<String>,
    Json(payload): Json<UpdateServer>,
) -> Result<Json<Server>, StatusCode> {
    // Fetch the current server details to have a complete object to return
    let current_server = match get_server(State(db_pool.clone()), Path(guild_id.clone())).await {
        Ok(Json(server)) => server,
        Err(status) => return Err(status),
    };

    let guild_name = payload.guild_name.unwrap_or(current_server.guild_name);
    let is_premium = payload.is_premium.unwrap_or(current_server.is_premium);
    let max_threads = payload.max_threads.unwrap_or(current_server.max_threads);
    let max_macros = payload.max_macros.unwrap_or(current_server.max_macros);

    match sqlx::query_as::<_, Server>(
        "UPDATE servers SET guild_name = $1, is_premium = $2, max_threads = $3, max_macros = $4, updated_at = NOW() WHERE guild_id = $5 RETURNING *"
    )
    .bind(guild_name)
    .bind(is_premium)
    .bind(max_threads)
    .bind(max_macros)
    .bind(guild_id)
    .fetch_one(&db_pool)
    .await {
        Ok(server) => Ok(Json(server)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Delete a server
async fn delete_server(
    State(db_pool): State<PgPool>,
    Path(guild_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    match sqlx::query("DELETE FROM servers WHERE guild_id = $1")
        .bind(guild_id)
        .execute(&db_pool)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => Ok(StatusCode::NO_CONTENT),
        Ok(_) => Err(StatusCode::NOT_FOUND), // No rows affected
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
