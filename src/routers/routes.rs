// use crate::services;

use super::handlers::*;
use actix_web::web::{self, ServiceConfig};

pub fn login_routes(cfg: &mut ServiceConfig) {
    cfg.route("/login", web::post().to(login));
}

pub fn regiser_routes(cfg: &mut ServiceConfig) {
    cfg.route("/regiter", web::post().to(register));
}

pub fn index_routes(cfg: &mut ServiceConfig) {
    cfg.route("/", web::get().to(index_page));
}

//展示博客文章
pub fn post(cfg: &mut ServiceConfig) {
    cfg.route("/posts", web::get().to(show_blog));
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
