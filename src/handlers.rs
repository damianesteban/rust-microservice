use actix_web::{HttpRequest, HttpResponse };
use crate::models::{User, UserList, RegisterUser};
use actix_web::web;
use crate::db_connection::{ PgPool, PgPooledConnection };
use crate::models::AuthUser;
use crate::errors::{MyStoreError};

/**
 * Provides the pooled postgres connection for other handlers
 */
pub fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
    pool
    .get()
    .map_err(|e| {
        HttpResponse::InternalServerError().json(e.to_string())
    })
}

pub fn user_list_handler(_req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
  let pg_pool = pg_pool_handler(pool)?;

  UserList::list(&pg_pool)
    .map(|list| HttpResponse::Ok().json(list))
    .map_err(|e| {
      HttpResponse::InternalServerError().json(e.to_string())
    })
}

/**
 * Registers a user
 */
pub fn register(new_user: web::Json<RegisterUser>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
  let pg_pool = pg_pool_handler(pool)?;

  let register_user = new_user
    .into_inner()
    .validates()
    .map_err(|e| {
      HttpResponse::InternalServerError().json(e.to_string())
    })?;

  User::create(register_user, &pg_pool)
    .map(|user| HttpResponse::Ok().json(user))
    .map_err(|e| {
      HttpResponse::InternalServerError().json(e.to_string())
    })
}

pub fn login(auth_user: web::Json<AuthUser>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
  let pg_pool = pg_pool_handler(pool)?;
  let user = auth_user
    .login(&pg_pool)
    .map_err(|e| {
      match e {
        MyStoreError::DBError(diesel::result::Error::NotFound) =>
          HttpResponse::NotFound().json(e.to_string()),
        _ =>
          HttpResponse::InternalServerError().json(e.to_string())
      }
    })?;

    let response =
        HttpResponse::Ok().json(user);
    Ok(response)
}
