use crate::db;
use crate::structs::CreateMacro;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
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

async fn get_macros(State(pool): State<PgPool>, Path(guild_id): Path<String>) -> Response {
    let macros_result =
        sqlx::query_as::<_, db::Macro>("SELECT * FROM macros WHERE guild_id = $1 ORDER BY name")
            .bind(guild_id)
            .fetch_all(&pool)
            .await;

    match macros_result {
        Ok(macros) => (StatusCode::OK, Json(macros)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Failed to fetch macros" })),
        )
            .into_response(),
    }
}

async fn get_quick_access_macros(
    State(pool): State<PgPool>,
    Path(guild_id): Path<String>,
) -> Response {
    let macros_result = sqlx::query_as::<_, db::Macro>(
        "SELECT * FROM macros WHERE quick_access = TRUE AND guild_id = $1 ORDER BY name LIMIT 3",
    )
    .bind(guild_id)
    .fetch_all(&pool)
    .await;

    match macros_result {
        Ok(macros) => (StatusCode::OK, Json(macros)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Failed to fetch quick access macros" })),
        )
            .into_response(),
    }
}

async fn create_macro(
    State(pool): State<PgPool>,
    Path(guild_id): Path<String>,
    Json(payload): Json<CreateMacro>,
) -> Response {
    let quick_access = payload.quick_access.unwrap_or(false);

    if quick_access {
        let count_result: Result<i64, sqlx::Error> = sqlx::query_scalar(
            "SELECT COUNT(*) FROM macros WHERE quick_access = TRUE AND guild_id = $1",
        )
        .bind(guild_id.clone())
        .fetch_one(&pool)
        .await;

        if let Ok(count) = count_result {
            if count >= 3 {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(
                        serde_json::json!({ "error": "Maximum of 3 quick access macros allowed" }),
                    ),
                )
                    .into_response();
            }
        }
    }

    let new_macro_result = sqlx::query_as::<_, db::Macro>(
        "INSERT INTO macros (name, content, quick_access, guild_id) VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(&payload.name)
    .bind(&payload.content)
    .bind(quick_access)
    .bind(guild_id)
    .fetch_one(&pool)
    .await;

    match new_macro_result {
        Ok(new_macro) => (StatusCode::CREATED, Json(new_macro)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Failed to create macro" })),
        )
            .into_response(),
    }
}

async fn get_macro_by_name(
    State(pool): State<PgPool>,
    Path((guild_id, name)): Path<(String, String)>,
) -> Response {
    let macro_result =
        sqlx::query_as::<_, db::Macro>("SELECT * FROM macros WHERE name = $1 AND guild_id = $2")
            .bind(name)
            .bind(guild_id)
            .fetch_optional(&pool)
            .await;

    match macro_result {
        Ok(Some(macro_data)) => (StatusCode::OK, Json(macro_data)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Macro not found" })),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Failed to fetch macro" })),
        )
            .into_response(),
    }
}

async fn delete_macro(
    State(pool): State<PgPool>,
    Path((guild_id, name)): Path<(String, String)>,
) -> Response {
    let delete_result = sqlx::query("DELETE FROM macros WHERE name = $1 AND guild_id = $2")
        .bind(name)
        .bind(guild_id)
        .execute(&pool)
        .await;

    match delete_result {
        Ok(result) if result.rows_affected() > 0 => (StatusCode::NO_CONTENT).into_response(),
        Ok(_) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Macro not found" })),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Failed to delete macro" })),
        )
            .into_response(),
    }
}

async fn update_macro(
    State(pool): State<PgPool>,
    Path((guild_id, name)): Path<(String, String)>,
    Json(payload): Json<CreateMacro>,
) -> Response {
    let quick_access = payload.quick_access.unwrap_or(false);

    if quick_access {
        let count_result: Result<i64, sqlx::Error> = sqlx::query_scalar(
            "SELECT COUNT(*) FROM macros WHERE quick_access = TRUE AND name != $1 AND guild_id = $2",
        )
        .bind(name.clone())
        .bind(guild_id.clone())
        .fetch_one(&pool)
        .await;

        if let Ok(count) = count_result {
            if count >= 3 {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(
                        serde_json::json!({ "error": "Maximum of 3 quick access macros allowed" }),
                    ),
                )
                    .into_response();
            }
        }
    }

    let updated_macro_result = sqlx::query_as::<_, db::Macro>(
        "UPDATE macros SET content = $1, quick_access = $2 WHERE name = $3 AND guild_id = $4 RETURNING *",
    )
    .bind(&payload.content)
    .bind(quick_access)
    .bind(name)
    .bind(guild_id)
    .fetch_one(&pool)
    .await;

    match updated_macro_result {
        Ok(updated_macro) => (StatusCode::OK, Json(updated_macro)).into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Macro not found" })),
        )
            .into_response(),
    }
}
