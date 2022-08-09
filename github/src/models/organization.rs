use serde::{ Deserialize, Serialize };

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Organization {
    pub id: i64,
    pub login: String,
    pub gravatar_id: String,
    pub url: String,
    pub avatar_url: String,
}
