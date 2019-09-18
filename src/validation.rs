use actix_web::{HttpRequest, HttpResponse };
use crate::models::{User, UserList, RegisterUser};
use actix_web::web;
use crate::db_connection::{ PgPool, PgPooledConnection };
use jsonwebtoken::{Validation, Algorithm};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Header, TokenData};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
}

pub fn validate_token(user_email: String, token: String) -> Result<TokenData<Claims>, jsonwebtoken::errors::ErrorKind> {
  let my_claims = Claims { sub: user_email };
  let client_secret = "iVdtK4V1vr4f_GrcrAZywnLZtyWQ8CS6VGBoKih5r62w8lRI-ea0Chk2t2V_blot";

  let token = match encode(&Header::default(), &my_claims, client_secret.as_ref()) {
      Ok(t) => t,
      Err(_) => panic!(), // in practice you would return the error
  };

  let validation = Validation::new(Algorithm::RS256);
  let mut validation = Validation { leeway: 60, ..Validation::default()};
  let mut validation = Validation { iss: Some("https://pkb.auth0.com".to_string()), ..Validation::default()};
  validation.set_audience(&"https://pkb/api");

  let token_data = match decode::<Claims>(&token, client_secret.as_ref(), &validation) {
    Ok(c) => c,
    Err(err) => match *err.kind() {
      ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
      ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
      _ => panic!("Some other errors")
    }
  };

  return Ok(token_data);
}


