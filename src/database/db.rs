use dotenv::dotenv;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use std::env;

pub struct Database {
    db_uname: String,
    db_passwd: String,
    db_name: String,
    db_host: String,
    db_port: String,
}

impl Database {
    /// 创建一个新的 Database 实例
    pub fn new(
        db_uname: String,
        db_name: String,
        db_passwd: String,
        db_host: String,
        db_port: String,
    ) -> Self {
        Self {
            db_uname,
            db_name,
            db_passwd,
            db_host,
            db_port,
        }
    }

    /// 初始化数据库连接池
    pub async fn db_init() -> Result<MySqlPool, sqlx::Error> {
        // 加载 .env 文件中的环境变量
        dotenv().ok();

        // 从环境变量中读取数据库配置
        let db = Self::from_env().unwrap();

        // 构建数据库连接 URL
        let db_url = format!(
            "mysql://{}:{}@{}:{}/{}",
            db.db_uname, db.db_passwd, db.db_host, db.db_port, db.db_name
        );

        // 创建数据库连接池
        let pool = MySqlPoolOptions::new()
            .max_connections(5) // 设置最大连接数
            .connect(&db_url)
            .await?;

        // 测试连接是否成功
        sqlx::query("SELECT 1").execute(&pool).await?;

        println!("Database connection pool initialized successfully!");

        Ok(pool)
    }

    /// 从环境变量加载数据库配置
    fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            db_uname: env::var("DB_UNAME")?,
            db_passwd: env::var("DB_PASSWD")?,
            db_name: env::var("DB_NAME")?,
            db_host: env::var("DB_HOST")?,
            db_port: env::var("DB_PORT")?,
        })
    }
}
