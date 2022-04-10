use super::{status, message};
use super::super::model::UserModel;

use actix_web::{post, web, Responder};
use serde::{Serialize, Deserialize};
use regex::Regex;
use mysql::*;
use mysql::prelude::*;

#[derive(Deserialize)]
pub struct SignUpRequestBody {
  pub email: String,
  pub password: String,
}

#[derive(Serialize)]
pub struct SignUpResponseBody {
  pub id: i32,
  pub email: String,
  pub password: String,
}

#[post("signup")]
pub async fn signup(pool: web::Data<mysql::Pool>, body: web::Json<SignUpRequestBody>) -> impl Responder {
  if is_not_email(&body.email) {
    return status(400).json(message("Invalid email"))
  }

  if body.password.chars().count() < 8 {
    return status(400).json(message("Too short password"))
  }
  
  if body.password.chars().count() > 32 {
    return status(400).json(message("Too long password"))
  }

  if has_space(&body.password) {
    return status(400).json(message("Space in password"))
  }

  let mut conn = pool.get_ref().get_conn().unwrap();

  let users = conn.query_map(
    format!("SELECT id, email, password FROM users WHERE email='{}'", body.email),
    |(id, email, password)| {
      UserModel { id, email, password }
    },
  ).unwrap();

  if let Some(_) = users.get(0) {
    return status(409).json(message("Email exists"))
  };

  conn.exec_drop(
    r"INSERT INTO users (email, password)
    VALUES (:email, :password)"
    , params! {
      "email" => &body.email,
      "password" => &body.password,
    }).unwrap();

  let results = conn.query_map(
    format!("SELECT id, email, password FROM users WHERE email='{}'", body.email),
    |(id, email, password)| {
      UserModel { id, email, password }
    },
  ).unwrap();

  let result = results.get(0).unwrap();

  status(200).json(SignUpResponseBody {
    id: result.id,
    email: result.email.clone(),
    password: result.password.clone(),
  })
}

fn is_not_email(value: &str) -> bool {
  let regex = Regex::new(r"(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$)").unwrap();
  
  !regex.is_match(value)
}

fn has_space(value: &str) -> bool {
  let regex = Regex::new(r"(\s)").unwrap();

  regex.is_match(value)
}