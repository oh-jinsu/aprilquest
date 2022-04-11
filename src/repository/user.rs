use std::sync::Arc;
use mysql::*;
use mysql::prelude::*;

use super::super::model::UserModel;

#[derive(Clone)]
pub struct UserRepository {
  pool: Arc<Pool>,
}

pub fn new(pool: Arc<Pool>) -> UserRepository {
  UserRepository {
    pool,
  }
}

impl UserRepository {
  pub fn find_by_email(&self, email: &str) -> Option<UserModel> {
    let mut conn = self.pool.get_conn().unwrap();

    let users = conn.query_map(
      format!("SELECT id, email, password FROM users WHERE email='{}'", email),
      |(id, email, password)| {
        UserModel { id, email, password }
      },
    ).unwrap();

    let first = users.get(0)?;

    let result = first.clone();

    Some(result)
  }

  pub fn save(&self, email: &str, password: &str) -> UserModel {
    let mut conn = self.pool.get_conn().unwrap();

    conn.exec_drop(
      r"INSERT INTO users (email, password)
      VALUES (:email, :password)"
      , params! {
        "email" => email,
        "password" => password,
      }).unwrap();
  
    let results = conn.query_map(
      format!("SELECT id, email, password FROM users WHERE email='{}'", email),
      |(id, email, password)| {
        UserModel { id, email, password }
      },
    ).unwrap();

    let result = results.get(0).unwrap();

    return result.clone();
  }
}