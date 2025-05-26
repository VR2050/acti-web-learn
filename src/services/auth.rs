use bcrypt::{DEFAULT_COST, hash, verify};
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
    // 查询用户是否存在
    let row = sqlx::query!(
        "SELECT username, password_hash FROM blog_admin WHERE username = ?",
        user.uname
    )
    .fetch_optional(pool)
    .await?;

    if let Some(row) = row {
        // 验证明文密码是否匹配数据库中的 hash
        if verify(user.passwd, &row.password_hash).unwrap() {
            println!("{}",row.password_hash);
            return Ok(true);
        }
    }
    Ok(false)
}
