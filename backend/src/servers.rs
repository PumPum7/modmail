use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{NewServer, Server};
use crate::schema::{guild_configs, servers};
use crate::structs::{CreateServer, UpdateServer, ValidateGuildRequest, ValidatedGuild};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::Utc;
use diesel::prelude::*;

// Route builder
pub fn server_routes(db_pool: DbPool) -> Router {
    Router::new()
        .route("/servers", post(create_server).get(get_servers))
        .route(
            "/servers/{guild_id}",
            get(get_server).put(update_server).delete(delete_server),
        )
        .route("/validate-guilds", post(validate_user_guilds))
        .with_state(db_pool)
}

// Get all servers
async fn get_servers(State(pool): State<DbPool>) -> Result<Json<Vec<Server>>, AppError> {
    let mut conn = pool.get()?;
    let results = servers::table.select(Server::as_select()).load(&mut conn)?;
    Ok(Json(results))
}

// Create a new server
async fn create_server(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateServer>,
) -> Result<Json<Server>, AppError> {
    let mut conn = pool.get()?;
    let now = Utc::now();
    let new_server = NewServer {
        guild_id: &payload.guild_id,
        guild_name: &payload.guild_name,
        created_at: now,
        updated_at: now,
    };

    let result = diesel::insert_into(servers::table)
        .values(&new_server)
        .returning(Server::as_returning())
        .get_result(&mut conn)?;

    Ok(Json(result))
}

// Get a specific server by guild_id
async fn get_server(
    State(pool): State<DbPool>,
    Path(guild_id_path): Path<String>,
) -> Result<Json<Server>, AppError> {
    let mut conn = pool.get()?;
    let result = servers::table
        .filter(servers::guild_id.eq(guild_id_path))
        .select(Server::as_select())
        .first(&mut conn)
        .optional()?;

    if let Some(server) = result {
        Ok(Json(server))
    } else {
        Err(AppError::Anyhow(anyhow::anyhow!("Server not found")))
    }
}

// Update a server's settings
async fn update_server(
    State(pool): State<DbPool>,
    Path(guild_id_path): Path<String>,
    Json(payload): Json<UpdateServer>,
) -> Result<Json<Server>, AppError> {
    let mut conn = pool.get()?;
    let target = servers::table.filter(servers::guild_id.eq(guild_id_path));

    let updated_server = diesel::update(target)
        .set((
            payload.guild_name.map(|name| servers::guild_name.eq(name)),
            payload.is_premium.map(|p| servers::is_premium.eq(p)),
            payload.max_threads.map(|mt| servers::max_threads.eq(mt)),
            payload.max_macros.map(|mm| servers::max_macros.eq(mm)),
            servers::updated_at.eq(Utc::now()),
        ))
        .returning(Server::as_returning())
        .get_result(&mut conn)?;

    Ok(Json(updated_server))
}

// Delete a server
async fn delete_server(
    State(pool): State<DbPool>,
    Path(guild_id_path): Path<String>,
) -> Result<StatusCode, AppError> {
    let mut conn = pool.get()?;
    let num_deleted = diesel::delete(servers::table.filter(servers::guild_id.eq(guild_id_path)))
        .execute(&mut conn)?;

    if num_deleted > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::Anyhow(anyhow::anyhow!("Server not found")))
    }
}

// Validate which guilds a user can access
async fn validate_user_guilds(
    State(pool): State<DbPool>,
    Json(payload): Json<Vec<ValidateGuildRequest>>,
) -> Result<Json<Vec<ValidatedGuild>>, AppError> {
    let mut conn = pool.get()?;
    let user_guild_ids: Vec<String> = payload.iter().map(|g| g.guild_id.clone()).collect();
    if user_guild_ids.is_empty() {
        return Ok(Json(vec![]));
    }

    let bot_guild_ids: std::collections::HashSet<String> = servers::table
        .filter(servers::guild_id.eq_any(&user_guild_ids))
        .select(servers::guild_id)
        .load::<String>(&mut conn)?
        .into_iter()
        .collect();

    let config_guild_ids: std::collections::HashSet<String> = guild_configs::table
        .filter(guild_configs::guild_id.eq_any(&user_guild_ids))
        .select(guild_configs::guild_id)
        .load::<String>(&mut conn)?
        .into_iter()
        .collect();

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
