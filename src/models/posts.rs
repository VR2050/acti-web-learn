use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct Post {
    pub date: NaiveDate,
    pub title: String,
    pub tags: String,
}

impl Post {
    pub fn new(date: NaiveDate, title: String, tags: String) -> Self {
        Self {
            date: date,
            title: title,
            tags: tags,
        }
    }
    //获取md文章文件名
    pub fn get_md_name(&self) -> String {
        let name = format!("{}_{}_{}.md", self.date.to_string(), self.title, self.tags);
        name
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ArchPost {
    pub year:String,
    pub posts:Vec<Post>
}

#[derive(Serialize,Deserialize,Clone)]
pub struct TagQuery{
    pub label:String,
}

