use crate::db;
use crate::errors::AppError;
use crate::structs::CreateMacro;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use sqlx::PgPool;

pub fn macro_routes(db_pool: PgPool) -> Router {
    Router::new()
        .route(
            "/guilds/:guild_id/macros",
            get(get_macros).post(create_macro),
        )
        .route(
            "/guilds/:guild_id/macros/quick-access",
            get(get_quick_access_macros),
        )
        .route(
            "/guilds/:guild_id/macros/:name",
            get(get_macro_by_name)
                .put(update_macro)
                .delete(delete_macro),
        )
        .with_state(db_pool)
}

async fn get_macros(
    State(pool): State<PgPool>,
    Path(guild_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let macros =
        sqlx::query_as::<_, db::Macro>("SELECT * FROM macros WHERE guild_id = $1 ORDER BY name")
            .bind(guild_id)
            .fetch_all(&pool)
            .await?;

    Ok((StatusCode::OK, Json(macros)))
}

async fn get_quick_access_macros(
    State(pool): State<PgPool>,
    Path(guild_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let macros = sqlx::query_as::<_, db::Macro>(
        "SELECT * FROM macros WHERE quick_access = TRUE AND guild_id = $1 ORDER BY name LIMIT 3",
    )
    .bind(guild_id)
    .fetch_all(&pool)
    .await?;

    Ok((StatusCode::OK, Json(macros)))
}

async fn create_macro(
    State(pool): State<PgPool>,
    Path(guild_id): Path<String>,
    Json(payload): Json<CreateMacro>,
) -> Result<impl IntoResponse, AppError> {
    let quick_access = payload.quick_access.unwrap_or(false);

    if quick_access {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM macros WHERE quick_access = TRUE AND guild_id = $1",
        )
        .bind(guild_id.clone())
        .fetch_one(&pool)
        .await?;

        if count >= 3 {
            return Err(AppError::Anyhow(anyhow::anyhow!(
                "Maximum of 3 quick access macros allowed"
            )));
        }
    }

    let new_macro = sqlx::query_as::<_, db::Macro>(
        "INSERT INTO macros (name, content, quick_access, guild_id) VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(&payload.name)
    .bind(&payload.content)
    .bind(quick_access)
    .bind(guild_id)
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(new_macro)))
}

async fn get_macro_by_name(
    State(pool): State<PgPool>,
    Path((guild_id, name)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let macro_data =
        sqlx::query_as::<_, db::Macro>("SELECT * FROM macros WHERE name = $1 AND guild_id = $2")
            .bind(name)
            .bind(guild_id)
            .fetch_optional(&pool)
            .await?;

    if let Some(macro_data) = macro_data {
        Ok((StatusCode::OK, Json(macro_data)))
    } else {
        Err(AppError::Anyhow(anyhow::anyhow!("Macro not found")))
    }
}

async fn delete_macro(
    State(pool): State<PgPool>,
    Path((guild_id, name)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let result = sqlx::query("DELETE FROM macros WHERE name = $1 AND guild_id = $2")
        .bind(name)
        .bind(guild_id)
        .execute(&pool)
        .await?;

    if result.rows_affected() > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::Anyhow(anyhow::anyhow!("Macro not found")))
    }
}

async fn update_macro(
    State(pool): State<PgPool>,
    Path((guild_id, name)): Path<(String, String)>,
    Json(payload): Json<CreateMacro>,
) -> Result<impl IntoResponse, AppError> {
    let quick_access = payload.quick_access.unwrap_or(false);

    if quick_access {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM macros WHERE quick_access = TRUE AND name != $1 AND guild_id = $2",
        )
        .bind(name.clone())
        .bind(guild_id.clone())
        .fetch_one(&pool)
        .await?;

        if count >= 3 {
            return Err(AppError::Anyhow(anyhow::anyhow!(
                "Maximum of 3 quick access macros allowed"
            )));
        }
    }

    let updated_macro = sqlx::query_as::<_, db::Macro>(
        "UPDATE macros SET content = $1, quick_access = $2 WHERE name = $3 AND guild_id = $4 RETURNING *",
    )
    .bind(&payload.content)
    .bind(quick_access)
    .bind(name)
    .bind(guild_id)
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::OK, Json(updated_macro)))
}
