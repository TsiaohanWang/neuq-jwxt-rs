//! 与教务系统进行交互的 HTTP 客户端

use super::log::prelude::*;
use super::query::{ClassroomQuery, Query};
use regex::Regex;
use reqwest::{Client, Response, header};
use sha1::{Digest, Sha1};

/// HTTP 客户端的身份标识字符串
const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/108.0.0.0 Safari/537.36";
/// HTTP 客户端的全局请求超时
const TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);
/// HTTP 客户端的操作间隔
const OPERATION_INTERVAL: std::time::Duration = std::time::Duration::from_secs(2);

/// 发送请求的教务系统 URL
const BASE_URL: &str = "https://jwxt.neuq.edu.cn/eams";
/// 发送登录请求的教务系统 URL 后缀
const LOGIN_URL_SUFFIX: &str = "/loginExt.action";
/// 提取登录 salt 的正则表达式
const LOGIN_SALT_REGEX: &str = r"CryptoJS\.SHA1\('([^']+)-' \+ form\['password'\]\.value\)";
/// 检验登录是否成功的重定向 URL 后缀
const LOGIN_VERIFIED_SUFFIX: &str = "/homeExt.action";
/// 发送空闲教室查询请求的教务系统 URL 后缀
const CLASSROOM_URL_SUFFIX: &str = "/classroom/apply/free!search.action";

// const COURSE_TABLE_URL_SUFFIX: &str = "/courseTableForStd!courseTable.action";

/// 与教务系统进行交互的核心 HTTP 客户端
#[derive(Debug)]
pub struct NeuqClient {
    client: Client,
    base_url: String,
}

impl NeuqClient {
    /// 构建一个 HTTP 客户端
    /// 
    /// - 已定义的常量会参与客户端构建。
    pub fn new() -> anyhow::Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "X-Requested-With",
            header::HeaderValue::from_static("XMLHttpRequest"),
        );
        headers.insert(
            "Accept",
            header::HeaderValue::from_static(
                "application/json, text/javascript, text/html, */*; q=0.01",
            ),
        );

        let client = Client::builder()
            .cookie_store(true)
            .user_agent(USER_AGENT)
            .default_headers(headers)
            .timeout(TIMEOUT)
            .build()?;

        info!("NeuqClient initialized.");

        Ok(Self {
            client,
            base_url: BASE_URL.to_owned(),
        })
    }

    /// 获取教务系统登录页面的 salt
    /// 
    /// - 已定义的常量会参与 salt 的获取。
    pub async fn get_salt(&self) -> anyhow::Result<String> {
        let login_url = format!("{}{}", self.base_url, LOGIN_URL_SUFFIX);
        let login_html = self.client.get(&login_url).send().await?.text().await?;

        info!("Login html received.");

        let re = Regex::new(LOGIN_SALT_REGEX)?;

        return if let Some(caps) = re.captures(&login_html) {
            info!("Login salt captured.");
            Ok(caps[1].to_string())
        } else {
            Err(anyhow::anyhow!("Failed to retrieve salt."))
        };
    }

    /// 使用传入的学号 `username` 和密码 `password` 进行登录，
    /// 返回是否成功登录的 `bool` 值
    /// 
    /// - 已定义的常量会定义操作间隔，并参与登录成功的判定。
    pub async fn login(&self, username: &str, password: &str) -> anyhow::Result<bool> {
        let salt = self.get_salt().await?;

        tokio::time::sleep(OPERATION_INTERVAL).await;

        let mut hasher = Sha1::new();
        Digest::update(&mut hasher, format!("{}-{}", salt, password).as_bytes());
        let hashed_password = hex::encode(hasher.finalize());

        let params = [
            ("username", username),
            ("password", &hashed_password),
            ("session_locale", "zh_CN"),
        ];

        let login_url = format!("{}{}", self.base_url, LOGIN_URL_SUFFIX);
        let res = self.client.post(&login_url).form(&params).send().await?;

        info!("Login response received.");
        debug!("Login response: {:?}", &res);

        Ok(res.url().path().contains(LOGIN_VERIFIED_SUFFIX))
    }

    /// 向服务端指定 URL 后缀 `url_suffix` 发送 POST 请求，
    /// 其中请求体实现了 `Query` 特征
    pub async fn post<Q: Query>(
        &self,
        url_suffix: &str,
        query_params: Q,
    ) -> anyhow::Result<Response> {
        let url = format!("{}{}", self.base_url, url_suffix);
        let params: Vec<(String, String)> = query_params.into();

        let res = self.client.post(&url).form(&params).send().await?;

        Ok(res)
    }

    /// 向服务端发送空闲教室查询的请求，
    /// 并返回接收到的 HTML 文档
    /// 
    /// - 其中请求体参数由 `ClassroomQuery` 定义。
    pub async fn get_classroom_html(&self, query_params: ClassroomQuery) -> anyhow::Result<String> {
        let res = self.post(CLASSROOM_URL_SUFFIX, query_params).await?;

        let html = res.text().await?;

        Ok(html)
    }

    // pub async fn get_course_table_html(&self, query_params: Query) -> anyhow::Result<String> {
    //     // let params = vec![
    //     //     ("ignoreHead", "1"),
    //     //     ("showPrintAndExport", "1"),
    //     //     ("setting.kind", "std"),
    //     //     ("ids", "44827"),
    //     //     ("semester.id", "87"),
    //     //     ("startWeek", "1"),
    //     // ];
    
    //     let res = self.client.post(&url).form(&params).send().await?;
    
    //     let html = res.text().await?;
    
    //     Ok(html)
    // }
}

/// 与教务系统进行交互的 HTTP 客户端的常用导入
pub mod prelude {
    pub use super::NeuqClient;
}
