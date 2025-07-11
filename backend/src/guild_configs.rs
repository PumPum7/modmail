use crate::errors::AppError;
use crate::structs::{CreateGuildConfig, GuildConfig, UpdateGuildConfig};
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use sqlx::PgPool;

// Route builder
pub fn guild_config_routes(db_pool: PgPool) -> Router {
    Router::new()
        .route(
            "/guilds/:guild_id/config",
            get(get_guild_config)
                .put(update_guild_config)
                .post(create_guild_config),
        )
        .with_state(db_pool)
}

// Get guild configuration
async fn get_guild_config(
    State(db_pool): State<PgPool>,
    Path(guild_id): Path<String>,
) -> Result<Json<GuildConfig>, AppError> {
    let config =
        sqlx::query_as::<_, GuildConfig>("SELECT * FROM guild_configs WHERE guild_id = $1")
            .bind(guild_id)
            .fetch_optional(&db_pool)
            .await?;
    if let Some(config) = config {
        Ok(Json(config))
    } else {
        Err(AppError::Anyhow(anyhow::anyhow!("Config not found")))
    }
}

// Create initial guild configuration
async fn create_guild_config(
    State(db_pool): State<PgPool>,
    Path(guild_id): Path<String>,
    Json(payload): Json<CreateGuildConfig>,
) -> Result<Json<GuildConfig>, AppError> {
    let config = sqlx::query_as::<_, GuildConfig>(
        "INSERT INTO guild_configs (guild_id, modmail_category_id, log_channel_id, randomize_names, auto_close_hours, welcome_message, moderator_role_ids, blocked_words) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *"
    )
    .bind(guild_id)
    .bind(payload.modmail_category_id)
    .bind(payload.log_channel_id)
    .bind(payload.randomize_names.unwrap_or(false))
    .bind(payload.auto_close_hours)
    .bind(payload.welcome_message)
    .bind(payload.moderator_role_ids.unwrap_or_default())
    .bind(payload.blocked_words.unwrap_or_default())
    .fetch_one(&db_pool)
    .await?;
    Ok(Json(config))
}

// Update guild configuration
async fn update_guild_config(
    State(db_pool): State<PgPool>,
    Path(guild_id): Path<String>,
    Json(payload): Json<UpdateGuildConfig>,
) -> Result<Json<GuildConfig>, AppError> {
    let current_config =
        match get_guild_config(State(db_pool.clone()), Path(guild_id.clone())).await {
            Ok(Json(config)) => config,
            Err(e) => return Err(e),
        };

    let modmail_category_id = payload
        .modmail_category_id
        .or(current_config.modmail_category_id);
    let log_channel_id = payload.log_channel_id.or(current_config.log_channel_id);
    let randomize_names = payload
        .randomize_names
        .unwrap_or(current_config.randomize_names);
    let auto_close_hours = payload.auto_close_hours.or(current_config.auto_close_hours);
    let welcome_message = payload.welcome_message.or(current_config.welcome_message);
    let moderator_role_ids = payload
        .moderator_role_ids
        .unwrap_or(current_config.moderator_role_ids);
    let blocked_words = payload
        .blocked_words
        .unwrap_or(current_config.blocked_words);

    let config = sqlx::query_as::<_, GuildConfig>(
        "UPDATE guild_configs SET modmail_category_id = $1, log_channel_id = $2, randomize_names = $3, auto_close_hours = $4, welcome_message = $5, moderator_role_ids = $6, blocked_words = $7, updated_at = NOW() WHERE guild_id = $8 RETURNING *"
    )
    .bind(modmail_category_id)
    .bind(log_channel_id)
    .bind(randomize_names)
    .bind(auto_close_hours)
    .bind(welcome_message)
    .bind(moderator_role_ids)
    .bind(blocked_words)
    .bind(guild_id)
    .fetch_one(&db_pool)
    .await?;
    Ok(Json(config))
}
