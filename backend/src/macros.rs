use crate::db;
use crate::structs::CreateMacro;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use sqlx::PgPool;

#[get("/macros")]
async fn get_macros(pool: web::Data<PgPool>) -> impl Responder {
    let macros_result = sqlx::query_as::<_, db::Macro>("SELECT * FROM macros ORDER BY name")
        .fetch_all(pool.get_ref())
        .await;

    match macros_result {
        Ok(macros) => HttpResponse::Ok().json(macros),
        Err(e) => {
            eprintln!("Database error fetching macros: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch macros"
            }))
        }
    }
}

#[get("/macros/quick-access")]
async fn get_quick_access_macros(pool: web::Data<PgPool>) -> impl Responder {
    let macros_result = sqlx::query_as::<_, db::Macro>(
        "SELECT * FROM macros WHERE quick_access = TRUE ORDER BY name LIMIT 3",
    )
    .fetch_all(pool.get_ref())
    .await;

    match macros_result {
        Ok(macros) => HttpResponse::Ok().json(macros),
        Err(e) => {
            eprintln!("Database error fetching quick access macros: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch quick access macros"
            }))
        }
    }
}

#[post("/macros")]
async fn create_macro(
    pool: web::Data<PgPool>,
    macro_data: web::Json<CreateMacro>,
) -> Result<impl Responder> {
    let quick_access = macro_data.quick_access.unwrap_or(false);

    // Check if we already have 3 quick access macros
    if quick_access {
        let count_result: Result<i64, sqlx::Error> =
            sqlx::query_scalar("SELECT COUNT(*) FROM macros WHERE quick_access = TRUE")
                .fetch_one(pool.get_ref())
                .await;

        let count = match count_result {
            Ok(count) => count,
            Err(e) => {
                eprintln!("Database error counting quick access macros: {}", e);
                return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to check quick access macro count"
                })));
            }
        };

        if count >= 3 {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Maximum of 3 quick access macros allowed"
            })));
        }
    }

    let new_macro_result = sqlx::query_as::<_, db::Macro>(
        "INSERT INTO macros (name, content, quick_access) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&macro_data.name)
    .bind(&macro_data.content)
    .bind(quick_access)
    .fetch_one(pool.get_ref())
    .await;

    match new_macro_result {
        Ok(new_macro) => Ok(HttpResponse::Ok().json(new_macro)),
        Err(e) => {
            eprintln!("Database error creating macro: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create macro"
            })))
        }
    }
}

#[get("/macros/{name}")]
async fn get_macro_by_name(pool: web::Data<PgPool>, name: web::Path<String>) -> impl Responder {
    let macro_result = sqlx::query_as::<_, db::Macro>("SELECT * FROM macros WHERE name = $1")
        .bind(name.as_str())
        .fetch_optional(pool.get_ref())
        .await;

    match macro_result {
        Ok(Some(macro_data)) => HttpResponse::Ok().json(macro_data),
        Ok(None) => HttpResponse::Ok().json(serde_json::json!(null)),
        Err(e) => {
            eprintln!("Database error fetching macro: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch macro"
            }))
        }
    }
}

#[delete("/macros/{name}")]
async fn delete_macro(pool: web::Data<PgPool>, name: web::Path<String>) -> impl Responder {
    let delete_result = sqlx::query("DELETE FROM macros WHERE name = $1")
        .bind(name.as_str())
        .execute(pool.get_ref())
        .await;

    match delete_result {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().json(serde_json::json!({
                    "success": true,
                    "message": "Macro deleted successfully"
                }))
            } else {
                HttpResponse::NotFound().json(serde_json::json!({
                    "success": false,
                    "message": "Macro not found"
                }))
            }
        }
        Err(e) => {
            eprintln!("Database error deleting macro: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete macro"
            }))
        }
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
        let count_result: Result<i64, sqlx::Error> = sqlx::query_scalar(
            "SELECT COUNT(*) FROM macros WHERE quick_access = TRUE AND name != $1",
        )
        .bind(name.as_str())
        .fetch_one(pool.get_ref())
        .await;

        let count = match count_result {
            Ok(count) => count,
            Err(e) => {
                eprintln!("Database error counting quick access macros: {}", e);
                return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to check quick access macro count"
                })));
            }
        };

        if count >= 3 {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Maximum of 3 quick access macros allowed"
            })));
        }
    }

    let updated_macro_result = sqlx::query_as::<_, db::Macro>(
        "UPDATE macros SET content = $1, quick_access = $2 WHERE name = $3 RETURNING *",
    )
    .bind(&macro_data.content)
    .bind(quick_access)
    .bind(name.as_str())
    .fetch_one(pool.get_ref())
    .await;

    match updated_macro_result {
        Ok(updated_macro) => Ok(HttpResponse::Ok().json(updated_macro)),
        Err(sqlx::Error::RowNotFound) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Macro not found"
        }))),
        Err(e) => {
            eprintln!("Database error updating macro: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update macro"
            })))
        }
    }
}
