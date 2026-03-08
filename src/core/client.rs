use super::log::prelude::*;
use super::query::{ClassroomQuery, Query};
use regex::Regex;
use reqwest::{Client, Response, header};
use sha1::{Digest, Sha1};

const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/108.0.0.0 Safari/537.36";

const TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);
const OPERATION_INTERVAL: std::time::Duration = std::time::Duration::from_secs(2);

const BASE_URL: &str = "https://jwxt.neuq.edu.cn/eams";

const LOGIN_URL_SUFFIX: &str = "/loginExt.action";
const LOGIN_SALT_REGEX: &str = r"CryptoJS\.SHA1\('([^']+)-' \+ form\['password'\]\.value\)";

const LOGIN_VERIFIED_SUFFIX: &str = "/homeExt.action";

const CLASSROOM_URL_SUFFIX: &str = "/classroom/apply/free!search.action";

// const COURSE_TABLE_URL_SUFFIX: &str = "/courseTableForStd!courseTable.action";

#[derive(Debug)]
pub struct NeuqClient {
    client: Client,
    base_url: String,
}

impl NeuqClient {
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

pub mod prelude {
    pub use super::NeuqClient;
}
