// use crate::services;

use super::handlers::*;
use actix_web::web::{self, ServiceConfig};

pub fn login_routes(cfg: &mut ServiceConfig) {
    cfg.route("/login", web::post().to(login));
}

pub fn regiser_routes(cfg: &mut ServiceConfig) {
    cfg.route("/register", web::post().to(register));
}

pub fn index_routes(cfg: &mut ServiceConfig) {
    cfg.route("/", web::get().to(index_page));
}

//展示博客文章
pub fn post(cfg: &mut ServiceConfig) {
    cfg.route("/post", web::get().to(show_blog));
}

//博客文章搜索
pub fn blog_search(cfg: &mut ServiceConfig) {
    cfg.route("/search", web::get().to(search));
}

//博客文章归档
pub fn blog_arch(cfg: &mut ServiceConfig) {
    cfg.route("/arch", web::get().to(arch_posts));
}

//博客文章标签分类
pub fn blog_tag(cfg: &mut ServiceConfig) {
    cfg.route("/tag", web::get().to(tag_posts));
}

pub fn blog_tag_all(cfg: &mut ServiceConfig) {
    cfg.route("/tags", web::get().to(tag_all_posts));
}

//管理员控制面板
pub fn dashboard(cfg: &mut ServiceConfig) {
    cfg.route("/dashboard", web::get().to(manage));
}

//md文档上传

pub fn upload_post(cfg: &mut ServiceConfig) {
    cfg.route("/upload", web::post().to(upload_md));
}

//图片上传

pub fn upload_picture(cfg: &mut ServiceConfig) {
    cfg.route("/upload", web::post().to(upload_pic));
}
