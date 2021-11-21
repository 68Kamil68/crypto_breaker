use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserInfo {
    pub email: String,
    pub hash: String,
}
