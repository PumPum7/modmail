use crate::db;
use crate::structs::CreateMacro;
use actix_web::{delete, get, post, put, web, Responder, Result};
use sqlx::PgPool;

#[get("/macros")]
async fn get_macros(pool: web::Data<PgPool>) -> impl Responder {
    let macros = sqlx::query_as::<_, db::Macro>("SELECT * FROM macros ORDER BY name")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    web::Json(macros)
}

#[get("/macros/quick-access")]
async fn get_quick_access_macros(pool: web::Data<PgPool>) -> impl Responder {
    let macros = sqlx::query_as::<_, db::Macro>(
        "SELECT * FROM macros WHERE quick_access = TRUE ORDER BY name LIMIT 3",
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

    web::Json(macros)
}

#[post("/macros")]
async fn create_macro(
    pool: web::Data<PgPool>,
    macro_data: web::Json<CreateMacro>,
) -> Result<impl Responder> {
    let quick_access = macro_data.quick_access.unwrap_or(false);

    // Check if we already have 3 quick access macros
    if quick_access {
        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM macros WHERE quick_access = TRUE")
                .fetch_one(pool.get_ref())
                .await
                .unwrap();

        if count >= 3 {
            return Ok(web::Json(serde_json::json!({
                "error": "Maximum of 3 quick access macros allowed"
            })));
        }
    }

    let new_macro = sqlx::query_as::<_, db::Macro>(
        "INSERT INTO macros (name, content, quick_access) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&macro_data.name)
    .bind(&macro_data.content)
    .bind(quick_access)
    .fetch_one(pool.get_ref())
    .await
    .unwrap();

    Ok(web::Json(serde_json::json!(new_macro)))
}

#[get("/macros/{name}")]
async fn get_macro_by_name(pool: web::Data<PgPool>, name: web::Path<String>) -> impl Responder {
    let macro_result = sqlx::query_as::<_, db::Macro>("SELECT * FROM macros WHERE name = $1")
        .bind(name.as_str())
        .fetch_optional(pool.get_ref())
        .await
        .unwrap();

    match macro_result {
        Some(macro_data) => web::Json(serde_json::json!(macro_data)),
        None => web::Json(serde_json::json!(null)),
    }
}

#[delete("/macros/{name}")]
async fn delete_macro(pool: web::Data<PgPool>, name: web::Path<String>) -> impl Responder {
    let result = sqlx::query("DELETE FROM macros WHERE name = $1")
        .bind(name.as_str())
        .execute(pool.get_ref())
        .await
        .unwrap();

    if result.rows_affected() > 0 {
        web::Json(serde_json::json!({
            "success": true,
            "message": "Macro deleted successfully"
        }))
    } else {
        web::Json(serde_json::json!({
            "success": false,
            "message": "Macro not found"
        }))
    }
}

#[put("/macros/{name}")]
async fn update_macro(
    pool: web::Data<PgPool>,
    name: web::Path<String>,
    macro_data: web::Json<CreateMacro>,
) -> Result<impl Responder> {
    let quick_access = macro_data.quick_access.unwrap_or(false);

    // Check if we already have 3 quick access macros (excluding current one)
    if quick_access {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM macros WHERE quick_access = TRUE AND name != $1",
        )
        .bind(name.as_str())
        .fetch_one(pool.get_ref())
        .await
        .unwrap();

        if count >= 3 {
            return Ok(web::Json(serde_json::json!({
                "error": "Maximum of 3 quick access macros allowed"
            })));
        }
    }

    let updated_macro = sqlx::query_as::<_, db::Macro>(
        "UPDATE macros SET content = $1, quick_access = $2 WHERE name = $3 RETURNING *",
    )
    .bind(&macro_data.content)
    .bind(quick_access)
    .bind(name.as_str())
    .fetch_one(pool.get_ref())
    .await
    .unwrap();

    Ok(web::Json(serde_json::json!(updated_macro)))
}
