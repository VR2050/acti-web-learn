use comrak::{ComrakOptions, markdown_to_html};
use std::{borrow::Cow, fmt::Write, path::Path};
use tokio::fs;

// 常量配置
const ASSETS_DIR: &str = "./src/assets";
const MD_DIR: &str = "md";
const CSS_PATH: &str = "./src/assets/github-markdown.css";
const CDN_CSS_URL: &str =
    "https://cdn.jsdelivr.net/npm/github-markdown-css@5.1.0/github-markdown.min.css";

pub async fn read_file_to_string(path: &str) -> Result<String, String> {
    fs::read_to_string(path)
        .await
        .map_err(|e| format!("Failed to read file {}: {}", path, e))
}
pub async fn load_css() -> Result<Cow<'static, str>, String> {
    // 优先尝试本地 CSS 文件
    if Path::new(CSS_PATH).exists() {
        return Ok(Cow::Owned(read_file_to_string(CSS_PATH).await?));
    } else {
        Err("loading css error".to_string())
    }
}

pub async fn show_md(md_name: &str) -> Result<String, String> {
    let md_path = format!("{}/{}/{}", ASSETS_DIR, MD_DIR, md_name);

    let markdown_content = read_file_to_string(&md_path)
        .await
        .map_err(|e| format!("无法读取文章内容：{}，路径：{}", e, md_path))?;

    let mut options = ComrakOptions::default();
    options.extension.autolink = true;
    options.extension.table = true;
    options.extension.tasklist = true;
    options.extension.strikethrough = true;
    options.render.unsafe_ = true;

    let html_content = markdown_to_html(&markdown_content, &options);

    let css_content = load_css().await?;

    let mut full_html = String::with_capacity(4096);

    write!(
        full_html,
        r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <link rel="stylesheet" href="{}">
    <style>
        {}
    </style>
</head>
<body>
    <div class="markdown-body">
        {}
    </div>
</body>
</html>"#,
        md_name, CDN_CSS_URL, css_content, html_content
    )
    .map_err(|e| format!("构建HTML时发生错误：{}", e))?;

    Ok(full_html)
}
