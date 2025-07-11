use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{Macro, NewMacro};
use crate::schema::macros::dsl::*;
use crate::structs::CreateMacro;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use diesel::prelude::*;

pub fn macro_routes(db_pool: DbPool) -> Router {
    Router::new()
        .route(
            "/guilds/{guild_id}/macros",
            get(get_macros).post(create_macro),
        )
        .route(
            "/guilds/{guild_id}/macros/quick-access",
            get(get_quick_access_macros),
        )
        .route(
            "/guilds/{guild_id}/macros/{macro_name}",
            get(get_macro_by_name)
                .put(update_macro)
                .delete(delete_macro),
        )
        .with_state(db_pool)
}

async fn get_macros(
    State(pool): State<DbPool>,
    Path(guild_id_path): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;
    let results = macros
        .filter(guild_id.eq(guild_id_path))
        .order(name.asc())
        .select(Macro::as_select())
        .load(&mut conn)?;

    Ok((StatusCode::OK, Json(results)))
}

async fn get_quick_access_macros(
    State(pool): State<DbPool>,
    Path(guild_id_path): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;
    let results = macros
        .filter(quick_access.eq(true).and(guild_id.eq(guild_id_path)))
        .order(name.asc())
        .limit(3)
        .select(Macro::as_select())
        .load(&mut conn)?;

    Ok((StatusCode::OK, Json(results)))
}

async fn create_macro(
    State(pool): State<DbPool>,
    Path(guild_id_path): Path<String>,
    Json(payload): Json<CreateMacro>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;
    let quick_access_payload = payload.quick_access.unwrap_or(false);

    if quick_access_payload {
        let count: i64 = macros
            .filter(quick_access.eq(true).and(guild_id.eq(&guild_id_path)))
            .count()
            .get_result(&mut conn)?;

        if count >= 3 {
            return Err(AppError::Anyhow(anyhow::anyhow!(
                "Maximum of 3 quick access macros allowed"
            )));
        }
    }

    let new_macro = NewMacro {
        name: &payload.name,
        content: &payload.content,
        quick_access: Some(quick_access_payload),
        guild_id: &guild_id_path,
    };

    let result = diesel::insert_into(macros)
        .values(&new_macro)
        .returning(Macro::as_returning())
        .get_result(&mut conn)?;

    Ok((StatusCode::CREATED, Json(result)))
}

async fn get_macro_by_name(
    State(pool): State<DbPool>,
    Path((guild_id_path, macro_name)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;
    let result = macros
        .filter(name.eq(macro_name).and(guild_id.eq(guild_id_path)))
        .select(Macro::as_select())
        .first(&mut conn)
        .optional()?;

    if let Some(macro_data) = result {
        Ok((StatusCode::OK, Json(macro_data)))
    } else {
        Err(AppError::Anyhow(anyhow::anyhow!("Macro not found")))
    }
}

async fn delete_macro(
    State(pool): State<DbPool>,
    Path((guild_id_path, macro_name)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;
    let num_deleted =
        diesel::delete(macros.filter(name.eq(macro_name).and(guild_id.eq(guild_id_path))))
            .execute(&mut conn)?;

    if num_deleted > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::Anyhow(anyhow::anyhow!("Macro not found")))
    }
}

async fn update_macro(
    State(pool): State<DbPool>,
    Path((guild_id_path, macro_name)): Path<(String, String)>,
    Json(payload): Json<CreateMacro>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = pool.get()?;
    let quick_access_payload = payload.quick_access.unwrap_or(false);

    if quick_access_payload {
        let count: i64 = macros
            .filter(
                quick_access
                    .eq(true)
                    .and(name.ne(&macro_name))
                    .and(guild_id.eq(&guild_id_path)),
            )
            .count()
            .get_result(&mut conn)?;

        if count >= 3 {
            return Err(AppError::Anyhow(anyhow::anyhow!(
                "Maximum of 3 quick access macros allowed"
            )));
        }
    }

    let updated_macro =
        diesel::update(macros.filter(name.eq(macro_name).and(guild_id.eq(guild_id_path))))
            .set((
                content.eq(payload.content),
                quick_access.eq(quick_access_payload),
            ))
            .returning(Macro::as_returning())
            .get_result(&mut conn)?;

    Ok((StatusCode::OK, Json(updated_macro)))
}
