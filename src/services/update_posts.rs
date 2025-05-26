use bytes::Bytes;

use crate::models::posts::Post;
use actix_multipart::MultipartError;
use futures::{Stream, StreamExt};
use sqlx::MySqlPool;
use tokio::{fs::File, io::AsyncWriteExt};

//markdown文章保存
pub async fn md_add(
    post: Post,
    pool: &MySqlPool,
    mut contene_stream: impl Stream<Item = Result<Bytes, MultipartError>> + Unpin,
) -> tokio::io::Result<bool> {
    let post_name = post.get_md_name();
    let _update_post = sqlx::query!(
        "INSERT INTO `posts`(`title`,`tags`,`date`) VALUES(?,?,?)",
        post.title,
        post.tags,
        post.date
    )
    .execute(pool)
    .await;

    let mut md = File::create(format!("./src/assets/md{}", post_name)).await?;
    while let Some(chunk) = contene_stream.next().await {
        md.write_all(&chunk.unwrap()).await?
    }
    md.flush().await?;
    Ok(true)
}
//图片保存,安全措施之后再做

pub async fn pic_update(
    pic_name: &str,
    mut content_stream: impl Stream<Item = Result<Bytes, MultipartError>> + Unpin,
) -> tokio::io::Result<bool> {
    let pic_path = format!("./assets/md_pictures{}", pic_name);
    let mut file = File::create(&pic_path).await?;

    while let Some(chunk) = content_stream.next().await {
        let data = chunk.unwrap();
        file.write_all(&data).await?;
    }
    file.flush().await?; // 确保所有数据都被写入磁盘
    Ok(true)
}

//上传请求解析
