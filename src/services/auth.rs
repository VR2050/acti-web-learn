use bcrypt::verify;
use sqlx::{MySqlPool, query};

use crate::models::user::User;
// use crate::models::user::Register_User;
pub async fn user_auth(pool: &MySqlPool, uname: String) -> Result<bool, sqlx::Error> {
    let query_result = query("select passwd from users where uname= ?")
        .bind(uname)
        .fetch_optional(pool)
        .await?;

    Ok(query_result.is_some())
}

pub async fn user_auth2(pool: &MySqlPool, user: User) -> Result<bool, sqlx::Error> {
    let passwd_hash = user.crypt();

    let query_result = query!(
        r#"select username,password_hash from blog_admin where username=? and password_hash=? "#,
        user.uname,
        passwd_hash
    )
    .fetch_optional(pool)
    .await?;
    if let Some(row) = query_result {
        if verify(passwd_hash, &row.password_hash).unwrap(){
            return Ok(true);
        }
    }

    Ok(false)
}
