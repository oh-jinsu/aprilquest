#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserModel {
  pub id: i32,
  pub email: String,
  pub password: String,
}