use crate::models::user::*;
use sqlx::{MySqlPool, query};

pub async fn register(pool: &MySqlPool, register_user: Register_User) -> Result<bool, sqlx::Error> {
    let query_result = query("select uname from users where uname=?")
        .bind(&register_user.email)
        .fetch_optional(pool)
        .await?;

    if query_result.is_some() {
        Ok(false)
    } else {
        // 用户不存在，执行插入操作
        let insert_result = sqlx::query("INSERT INTO users (uname, password) VALUES (?, ?)")
            .bind(&register_user.email) // 绑定用户名
            .bind(&register_user.passwd) // 绑定密码
            .execute(pool)
            .await?;

        // 检查插入是否成功（影响的行数大于0）
        if insert_result.rows_affected() > 0 {
            Ok(true) // 插入成功，返回 true
        } else {
            Ok(false) // 插入失败，返回 false
        }
    }
}
