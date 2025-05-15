use comrak::{markdown_to_html, ComrakOptions};
use std::{fmt::Write, path::Path};
use tokio::fs;

// 常量配置
const ASSETS_DIR: &str = "./src/assets";
const MD_DIR: &str = "md";
const CSS_PATH: &str = "./src/assets/github-markdown.css";
const CDN_CSS_URL: &str = "https://cdn.jsdelivr.net/npm/github-markdown-css@5.1.0/github-markdown.min.css";

pub async fn read_file_to_string(path: &str) -> Result<String, String> {
    fs::read_to_string(path)
        .await
        .map_err(|e| format!("Failed to read file {}: {}", path, e))
}

pub async fn load_css() -> Result<String, String> {
    // 优先尝试本地 CSS 文件
    if Path::new(CSS_PATH).exists() {
        return read_file_to_string(CSS_PATH).await;
    }

    // 精简版CSS，去除多余边距
    Ok(String::from(
        r#"
        .markdown-body {
            box-sizing: border-box;
            min-width: 200px;
            max-width: 980px;
            margin: 0;
            padding: 0;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
            font-size: 16px;
            line-height: 1.5;
        }
        body {
            margin: 0;
            padding: 0;
            background-color: #fff;
        }
        "#,
    ))
}

pub async fn show_md(md_name: &str) -> Result<String, String> {
    // 构建Markdown文件路径
    let md_path = format!("{}/{}/{}", ASSETS_DIR, MD_DIR, md_name);

    // 读取Markdown内容
    let markdown_content = match read_file_to_string(&md_path).await {
        Ok(content) => content,
        Err(e) => return Err(format!("无法读取文章内容：{}", e)),
    };

    // 设置Comrak选项
    let mut options = ComrakOptions::default();
    options.extension.autolink = true;
    options.extension.table = true;
    options.extension.tasklist = true;
    options.extension.strikethrough = true;
    options.render.unsafe_ = true;

    // Markdown 转 HTML
    let html_content = markdown_to_html(&markdown_content, &options);

    // 加载CSS样式
    let css_content = load_css().await?;

    // 构建完整的HTML页面
    let mut full_html = String::new();

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
        .content-wrapper {{
            padding: 10px;
            margin: 0 auto;
            max-width: 980px;
        }}
    </style>
</head>
<body>
    <div class="content-wrapper markdown-body">
        {}
    </div>
</body>
</html>"#,
        md_name,
        CDN_CSS_URL,
        css_content,
        html_content
    ).map_err(|e| format!("构建HTML时发生错误：{}", e))?;

    Ok(full_html)
}