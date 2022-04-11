use super::{
  status,
  message,
  super::{
    repository,
  }
};

use actix_web::{post, web, Responder};
use serde::{Serialize, Deserialize};
use regex::Regex;

#[derive(Serialize, Deserialize)]
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
pub async fn signup(
  user_repository: web::Data<repository::user::UserRepository>,
  body: web::Json<SignUpRequestBody>,
) -> impl Responder {
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

  if let Some(_) = user_repository.as_ref().find_by_email(&body.email) {
    return status(409).json(message("Email exists"))
  };

  actix_web::rt::time::sleep(std::time::Duration::from_millis(10)).await;

  let result = user_repository.as_ref().save(&body.email, &body.password);

  status(201).json(SignUpResponseBody {
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