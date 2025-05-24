use futures::StreamExt;
use actix_web::HttpResponse;
use crate::models::posts::Post;
use actix_multipart::Multipart;
use sqlx::MySqlPool;
use tokio::{fs::File, io::AsyncWriteExt};

//md文档上传功能
pub async fn md_upload(
    md: Post,
    pool: &MySqlPool,
    mut payload: Multipart,
) -> Result<bool, sqlx::Error> {
    // 返回的文件名
    let post_name = md.get_md_name();
    if let Some(item)=payload.next().await{
        let mut field=item?;
        
    }
    //先把文件保存到本地

    //进行数据库插入操作

    let insert = sqlx::query!(
        r#"INSERT INTO `posts`(`title`,`tags`,`date`) VALUES(?,?,?)"#,
        md.title,
        md.tags,
        md.date
    )
    .execute(pool)
    .await?;

    //文件保存到本地
    Ok(true)
}

//图片创建功能

//md文件本地保存
pub async fn save_md(file_name: &str, content: &str) -> tokio::io::Result<bool> {
    let mut md_file = File::create(format!("./src/assets/md/{}", file_name)).await?;

    md_file.write_all(content.as_bytes()).await?;

    md_file.flush().await?;
    Ok(true)
}
//图片本地保存
pub async fn save_pic(pic_name: &str, content: &[u8]) -> tokio::io::Result<bool> {
    let pic = format!("./src/assets/md_pictures/{}", pic_name);
    let mut pic = File::create(pic).await?;
    pic.write_all(content).await?;
    Ok(true)
}
//文件保存

// pub async fn save(filename:)


// pub async fn save_pic_stream(pic_name: &str, mut content_stream: impl futures::Stream<Item = Vec<u8>> + Unpin) -> tokio::io::Result<bool> {
//     let pic_path = format!("./src/{}", pic_name);
//     let mut file = File::create(&pic_path).await?;

//     while let Some(chunk) = content_stream.next().await {
//         file.write_all(&chunk).await?;
//     }
//     file.flush().await?; // 确保所有数据都被写入磁盘
//     Ok(true)
// }

//上传请求解析


