use sqlx::{MySqlPool,self};
use crate::models::posts::{ArchPost, Post};
use chrono::NaiveDate;
use std::collections::HashMap;
pub async fn arch(pool: &MySqlPool) -> Result<Vec<ArchPost>, sqlx::Error> {
    // 使用静态字符串作为查询语句，避免动态拼接导致的SQL注入风险
    let query = r#"
        SELECT 
            YEAR(p.date) AS year,
            p.date as "date: NaiveDate",
            p.title,
            p.tags
        FROM posts p
        ORDER BY year DESC, p.date DESC
    "#;

    // 执行查询并将结果映射到临时结构体
    let rows = sqlx::query_as::<_, (i32, NaiveDate, String, String)>(query)
        .fetch_all(pool)
        .await?;

    // 按年份对帖子进行分组
    let mut grouped_by_year: HashMap<String, Vec<Post>> = HashMap::new();
    for row in rows {
        let (year, date, title, tags) = row;
        let post = Post {
            date,
            title,
            tags,
        };
        grouped_by_year.entry(year.to_string()).or_insert_with(Vec::new).push(post);
    }

    // 将 HashMap 转换为 ArchPost 结构体
    let archive_years: Vec<ArchPost> = grouped_by_year.into_iter()
        .map(|(year, posts)| ArchPost {
            year,
            posts,
        })
        .collect();

    Ok(archive_years)
}
