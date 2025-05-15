use crate::models::posts::Post;
use sqlx::{self, MySqlPool};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct SearchQuery {
    pub keywords: String,
}

pub async fn search_posts(
    pool: &MySqlPool,
    keywords: SearchQuery,
) -> Result<Vec<Post>, sqlx::Error> {
    let query_str = format!("%{}%", keywords.keywords);

    // 注意这里的选择列和表名是否正确对应
    let posts = sqlx::query_as!(
        Post,
        r#"SELECT date as "date: chrono::NaiveDate", title, tags FROM posts WHERE title LIKE ?"#,
        query_str
    )
    .fetch_all(pool)
    .await?;

    Ok(posts)
}
