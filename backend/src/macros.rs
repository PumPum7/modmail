use crate::db;
use crate::structs::CreateMacro;
use actix_web::{delete, get, post, put, web, Responder, Result};
use sqlx::PgPool;

#[get("/macros")]
async fn get_macros(pool: web::Data<PgPool>) -> impl Responder {
    let macros = sqlx::query_as::<_, db::Macro>("SELECT * FROM macros")
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
    let new_macro = sqlx::query_as::<_, db::Macro>(
        "INSERT INTO macros (name, content) VALUES ($1, $2) RETURNING *",
    )
    .bind(&macro_data.name)
    .bind(&macro_data.content)
    .fetch_one(pool.get_ref())
    .await
    .unwrap();

    Ok(web::Json(new_macro))
}

#[get("/macros/{name}")]
async fn get_macro_by_name(pool: web::Data<PgPool>, name: web::Path<String>) -> impl Responder {
    let macro_result = sqlx::query_as::<_, db::Macro>("SELECT * FROM macros WHERE name = $1")
        .bind(name.into_inner())
        .fetch_optional(pool.get_ref())
        .await
        .unwrap();

    match macro_result {
        Some(macro_data) => web::Json(serde_json::json!(macro_data)),
        None => web::Json(serde_json::Value::Null),
    }
}

#[delete("/macros/{name}")]
async fn delete_macro(pool: web::Data<PgPool>, name: web::Path<String>) -> Result<impl Responder> {
    let rows_affected = sqlx::query("DELETE FROM macros WHERE name = $1")
        .bind(name.into_inner())
        .execute(pool.get_ref())
        .await
        .unwrap()
        .rows_affected();

    if rows_affected > 0 {
        Ok(web::Json(
            serde_json::json!({"success": true, "message": "Macro deleted"}),
        ))
    } else {
        Ok(web::Json(
            serde_json::json!({"success": false, "message": "Macro not found"}),
        ))
    }
}

#[put("/macros/{name}")]
async fn update_macro(
    pool: web::Data<PgPool>,
    name: web::Path<String>,
    macro_data: web::Json<CreateMacro>,
) -> Result<impl Responder> {
    let updated_macro = sqlx::query_as::<_, db::Macro>(
        "UPDATE macros SET content = $1 WHERE name = $2 RETURNING *",
    )
    .bind(&macro_data.content)
    .bind(name.into_inner())
    .fetch_one(pool.get_ref())
    .await
    .unwrap();

    Ok(web::Json(updated_macro))
}
