use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{GuildConfig, NewGuildConfig};
use crate::schema::guild_configs::dsl::*;
use crate::structs::{CreateGuildConfig, UpdateGuildConfig};
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use chrono::Utc;
use diesel::prelude::*;

pub fn guild_config_routes(db_pool: DbPool) -> Router {
    Router::new()
        .route(
            "/guilds/:guild_id/config",
            get(get_guild_config)
                .put(update_guild_config)
                .post(create_guild_config),
        )
        .with_state(db_pool)
}

async fn get_guild_config(
    State(pool): State<DbPool>,
    Path(guild_id_path): Path<String>,
) -> Result<Json<GuildConfig>, AppError> {
    let mut conn = pool.get()?;
    let config = guild_configs
        .filter(guild_id.eq(guild_id_path))
        .select(GuildConfig::as_select())
        .first(&mut conn)
        .optional()?;

    if let Some(config) = config {
        Ok(Json(config))
    } else {
        Err(AppError::Anyhow(anyhow::anyhow!("Config not found")))
    }
}

async fn create_guild_config(
    State(pool): State<DbPool>,
    Path(guild_id_path): Path<String>,
    Json(payload): Json<CreateGuildConfig>,
) -> Result<Json<GuildConfig>, AppError> {
    let mut conn = pool.get()?;
    let now = Utc::now();
    let new_config = NewGuildConfig {
        guild_id: &guild_id_path,
        modmail_category_id: payload.modmail_category_id,
        log_channel_id: payload.log_channel_id,
        randomize_names: payload.randomize_names,
        auto_close_hours: payload.auto_close_hours,
        welcome_message: payload.welcome_message,
        moderator_role_ids: payload.moderator_role_ids,
        blocked_words: payload.blocked_words,
        created_at: now,
        updated_at: now,
    };

    let result = diesel::insert_into(guild_configs)
        .values(&new_config)
        .returning(GuildConfig::as_returning())
        .get_result(&mut conn)?;

    Ok(Json(result))
}

async fn update_guild_config(
    State(pool): State<DbPool>,
    Path(guild_id_path): Path<String>,
    Json(payload): Json<UpdateGuildConfig>,
) -> Result<Json<GuildConfig>, AppError> {
    let mut conn = pool.get()?;
    let target = guild_configs.filter(guild_id.eq(guild_id_path));

    let updated_config = diesel::update(target)
        .set(payload)
        .returning(GuildConfig::as_returning())
        .get_result(&mut conn)?;

    Ok(Json(updated_config))
}
