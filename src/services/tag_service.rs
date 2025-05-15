use sqlx::{self, MySqlPool};

use crate::models::posts::Post;

pub async fn tag(pool: &MySqlPool, tag: &str) -> Result<Vec<Post>, sqlx::Error> {
    let query_str = format!("{}", tag);

    // 注意这里的选择列和表名是否正确对应

    let query_result = sqlx::query_as!(
        Post,
        r#"SELECT date as "date: chrono::NaiveDate", title, tags FROM posts WHERE tags = ?"#,
        query_str
    )
    .fetch_all(pool)
    .await?;

    Ok(query_result)
}



pub async fn tag_all(pool: &MySqlPool) -> Result<Vec<(String, Vec<Post>)>, sqlx::Error> {
    let posts = sqlx::query_as!(
        Post,
        r#"SELECT  title, date, tags FROM posts ORDER BY tags, id"#
    )
    .fetch_all(pool)
    .await?;

    let mut grouped = std::collections::HashMap::new();
    for post in posts {
        grouped
            .entry(post.tags.clone())
            .or_insert_with(Vec::new)
            .push(post);
    }

    let mut result: Vec<_> = grouped.into_iter().collect();
    result.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(result)
}
