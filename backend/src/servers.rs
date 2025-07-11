use crate::errors::AppError;
use crate::structs::{
    CreateServer, GuildConfig, Server, UpdateServer, ValidateGuildRequest, ValidatedGuild,
};
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
        .route("/validate-guilds", post(validate_user_guilds))
        .with_state(db_pool)
}

// Get all servers (for a user, you might add authentication and filtering here)
async fn get_servers(State(db_pool): State<PgPool>) -> Result<Json<Vec<Server>>, AppError> {
    let servers = sqlx::query_as::<_, Server>("SELECT * FROM servers")
        .fetch_all(&db_pool)
        .await?;
    Ok(Json(servers))
}

// Create a new server
async fn create_server(
    State(db_pool): State<PgPool>,
    Json(payload): Json<CreateServer>,
) -> Result<Json<Server>, AppError> {
    let server = sqlx::query_as::<_, Server>(
        "INSERT INTO servers (guild_id, guild_name) VALUES ($1, $2) RETURNING *",
    )
    .bind(payload.guild_id)
    .bind(payload.guild_name)
    .fetch_one(&db_pool)
    .await?;
    Ok(Json(server))
}

// Get a specific server by guild_id
async fn get_server(
    State(db_pool): State<PgPool>,
    Path(guild_id): Path<String>,
) -> Result<Json<Server>, AppError> {
    let server = sqlx::query_as::<_, Server>("SELECT * FROM servers WHERE guild_id = $1")
        .bind(guild_id)
        .fetch_optional(&db_pool)
        .await?;
    if let Some(server) = server {
        Ok(Json(server))
    } else {
        Err(AppError::Anyhow(anyhow::anyhow!("Server not found")))
    }
}

// Update a server's settings
async fn update_server(
    State(db_pool): State<PgPool>,
    Path(guild_id): Path<String>,
    Json(payload): Json<UpdateServer>,
) -> Result<Json<Server>, AppError> {
    // Fetch the current server details to have a complete object to return
    let current_server = match get_server(State(db_pool.clone()), Path(guild_id.clone())).await {
        Ok(Json(server)) => server,
        Err(e) => return Err(e),
    };

    let guild_name = payload.guild_name.unwrap_or(current_server.guild_name);
    let is_premium = payload.is_premium.unwrap_or(current_server.is_premium);
    let max_threads = payload.max_threads.unwrap_or(current_server.max_threads);
    let max_macros = payload.max_macros.unwrap_or(current_server.max_macros);

    let server = sqlx::query_as::<_, Server>(
        "UPDATE servers SET guild_name = $1, is_premium = $2, max_threads = $3, max_macros = $4, updated_at = NOW() WHERE guild_id = $5 RETURNING *"
    )
    .bind(guild_name)
    .bind(is_premium)
    .bind(max_threads)
    .bind(max_macros)
    .bind(guild_id)
    .fetch_one(&db_pool)
    .await?;
    Ok(Json(server))
}

// Delete a server
async fn delete_server(
    State(db_pool): State<PgPool>,
    Path(guild_id): Path<String>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM servers WHERE guild_id = $1")
        .bind(guild_id)
        .execute(&db_pool)
        .await?;

    if result.rows_affected() > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::Anyhow(anyhow::anyhow!("Server not found")))
    }
}

// Validate which guilds a user can access (bot is present + user has permissions)
async fn validate_user_guilds(
    State(db_pool): State<PgPool>,
    Json(payload): Json<Vec<ValidateGuildRequest>>,
) -> Result<Json<Vec<ValidatedGuild>>, AppError> {
    // Collect all guild_ids from the user
    let user_guild_ids: Vec<String> = payload.iter().map(|g| g.guild_id.clone()).collect();
    if user_guild_ids.is_empty() {
        return Ok(Json(vec![]));
    }

    // Query all servers in one go
    let rows = sqlx::query_as::<_, Server>("SELECT guild_id FROM servers WHERE guild_id = ANY($1)")
        .bind(&user_guild_ids)
        .fetch_all(&db_pool)
        .await?;
    let bot_guild_ids: std::collections::HashSet<String> =
        rows.iter().map(|r| r.guild_id.clone()).collect();

    // Query configs for those guilds
    let config_rows = sqlx::query_as::<_, GuildConfig>(
        "SELECT guild_id FROM guild_configs WHERE guild_id = ANY($1)",
    )
    .bind(&user_guild_ids)
    .fetch_all(&db_pool)
    .await?;
    let config_guild_ids: std::collections::HashSet<String> =
        config_rows.into_iter().map(|r| r.guild_id).collect();

    // Build validated list
    let validated_guilds: Vec<ValidatedGuild> = payload
        .into_iter()
        .filter(|g| bot_guild_ids.contains(&g.guild_id))
        .map(|g| ValidatedGuild {
            guild_id: g.guild_id.clone(),
            guild_name: g.guild_name,
            guild_icon: g.guild_icon,
            has_bot: true,
            has_config: config_guild_ids.contains(&g.guild_id),
            user_has_permissions: g.user_has_permissions,
        })
        .collect();

    Ok(Json(validated_guilds))
}
