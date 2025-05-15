use sqlx::{MySqlPool, query};
// use crate::models::user::Register_User;
pub async fn user_auth(pool: &MySqlPool, uname: String) -> Result<bool, sqlx::Error> {
    let query_result = query("select passwd from users where uname= ?")
        .bind(uname)
        .fetch_optional(pool)
        .await?;

    Ok(query_result.is_some())
}

