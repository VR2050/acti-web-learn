use crate::models::posts::Post;
use crate::models::posts::TagQuery;
use crate::models::user::*;
use crate::services::arch_service::arch;
use crate::services::auth::user_auth2;
use crate::services::register;
use crate::services::search_service;
use crate::services::search_service::SearchQuery;
use crate::services::show_posts;
use crate::services::tag_service::tag;
use crate::services::tag_service::tag_all;
use crate::services::update_posts::md_add;
use crate::services::update_posts::pic_update;
use actix_multipart::Multipart;
use actix_web::cookie::Cookie;
use actix_web::http::header::SET_COOKIE;
use actix_web::{HttpResponse, web};
use futures::StreamExt;
use serde_json::json;
use sqlx::MySqlPool;
use std::fs;

//用户登录操作
pub async fn login(form: web::Json<User>, pool: web::Data<MySqlPool>) -> HttpResponse {
    let user = form.into_inner();

    match user_auth2(&pool, user).await {
        Ok(true) => {
            let cookie = Cookie::build("VRTZVZ", "VR2050")
                .path("/dashboard")
                .http_only(true)
                .secure(true)
                .finish();
            HttpResponse::Ok()
                .insert_header((SET_COOKIE, cookie.to_string()))
                .body("Logged in successfully!")
        }
        _ => HttpResponse::Ok().json(json!({"status":"the uname or passwd error"})),
    }
}

//用户注册操作
pub async fn register(form: web::Form<Register_User>, pool: web::Data<MySqlPool>) -> HttpResponse {
    let user = form.into_inner();

    let result = register::register(&pool, user).await.unwrap();
    let register_status = RegisterStatus::new(result);
    HttpResponse::Ok().json(register_status)
}
//主站页面
pub async fn index_page() -> HttpResponse {
    let index = fs::read_to_string("./static/index.html").expect("no file exist");

    HttpResponse::Ok().body(index)
}

//文章展示
pub async fn show_blog(post: web::Query<Post>) -> HttpResponse {
    let post = post.into_inner();
    // println!("{:?}",post);
    let blog = show_posts::show_md(&post.get_md_name()).await.unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(blog)
}

//博客文章搜索
pub async fn search(keywords: web::Query<SearchQuery>, pool: web::Data<MySqlPool>) -> HttpResponse {
    let query = keywords.into_inner();
    if let Ok(posts) = search_service::search_posts(&pool, query).await {
        HttpResponse::Ok().json(json!({"search result":posts}))
    } else {
        HttpResponse::Ok().json(json!({"status":"posts not found"}))
    }
}

//博客文章归档
// pub async fn arch_posts();
pub async fn arch_posts(pool: web::Data<MySqlPool>) -> HttpResponse {
    if let Ok(archpost) = arch(&pool).await {
        HttpResponse::Ok().json(json!(archpost))
    } else {
        HttpResponse::Ok().json(json!({"status":"arch error"}))
    }
}
//博客文章标签分类
pub async fn tag_posts(pool: web::Data<MySqlPool>, tagword: web::Query<TagQuery>) -> HttpResponse {
    let tagword = tagword.into_inner();
    if let Ok(posts) = tag(&pool, &tagword.label).await {
        HttpResponse::Ok().json(json!({tagword.label:posts}))
    } else {
        HttpResponse::Ok().json(json!({"status":"tag error"}))
    }
}

//归档全部文章功能实现,返回json
pub async fn tag_all_posts(pool: web::Data<MySqlPool>) -> HttpResponse {
    if let Ok(posts) = tag_all(&pool).await {
        HttpResponse::Ok().json(json!(posts))
    } else {
        HttpResponse::Ok().json(json!({"status":"tags error"}))
    }
}

//管理员界面

pub async fn manage() -> HttpResponse {
    HttpResponse::Ok().body("dashboard")
}

//图片上传

pub async fn upload_pic(mut payload: Multipart) -> HttpResponse {
    if let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        let content_disposition = field.content_disposition().unwrap();
        let picture_name = content_disposition.get_filename().unwrap().to_string();
        // 安全措施之后再写
        match pic_update(&picture_name, &mut field).await {
            Ok(true) => HttpResponse::Ok().json(json!({"status":"picture图片上传成功"})),
            _ => HttpResponse::BadRequest().json(json!({"status":"sth error"})),
        }
    } else {
        HttpResponse::Ok().json(json!({"status":"sth error"}))
    }
}

//博客文件上传,前端便捷文档后端返回状态json
pub async fn upload_md(
    post: web::Query<Post>,
    mut payload: Multipart,
    pool: web::Data<&MySqlPool>,
) -> HttpResponse {
    if let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        let post = post.into_inner();
        match md_add(post, &pool, &mut field).await {
            Ok(true) => HttpResponse::Ok().json(json!({"status":"post upload success"})),
            _ => HttpResponse::BadRequest().json(json!({"status":"sth error"})),
        }
    } else {
        HttpResponse::BadRequest().json(json!({"status":"sth error"}))
    }
}
