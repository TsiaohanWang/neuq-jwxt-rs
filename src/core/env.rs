//! 对环境变量的获取操作

use super::log::prelude::*;
use anyhow::anyhow;
use dotenv::{dotenv, var};

/// 登录所用学号的环境变量名
const USERNAME: &str = "NEUQ_USERNAME";
/// 登录所用密码的环境变量名
const PASSWORD: &str = "NEUQ_PASSWORD";

/// 本地环境变量的获取
pub mod local {
    use super::*;

    /// 从本地 `.env` 文件获取 `key` 对应值
    pub fn fetch(key: &str) -> anyhow::Result<String> {
        dotenv().ok();

        Ok(var(key)?)
    }

    /// 从本地 `.env` 文件获取登录时的 `USERNAME` 和 `PASSWORD`
    /// 
    /// - 若获取失败并不会返回错误。
    pub fn fetch_login_var() -> (Option<String>, Option<String>) {
        return if let Ok(username) = fetch(USERNAME) {
            if let Ok(password) = fetch(PASSWORD) {
                info!("Environment variables fetched.");
                (Some(username), Some(password))
            } else {
                warn!("Failed to fetch environment variable {}", PASSWORD);
                (Some(username), None)
            }
        } else {
            if let Ok(password) = fetch(PASSWORD) {
                warn!("Failed to fetch environment variable {}", USERNAME);
                (None, Some(password))
            } else {
                warn!(
                    "Failed to fetch environment variables {} / {}",
                    USERNAME, PASSWORD
                );
                (None, None)
            }
        };
    }

    /// 从本地 `.env` 文件获取登录时的 `USERNAME` 和 `PASSWORD`
    /// 
    /// - 若获取失败会返回错误。
    pub fn login_var() -> anyhow::Result<(String, String)> {
        let res = fetch_login_var();

        return match res {
            (Some(username), Some(password)) => Ok((username, password)),
            (Some(_username), None) => {
                Err(anyhow!("Environment variable {:?} is not set.", PASSWORD))
            }
            (None, Some(_password)) => {
                Err(anyhow!("Environment variable {:?} is not set.", USERNAME))
            }
            (None, None) => Err(anyhow!(
                "Environment variable {} / {} is not set.",
                USERNAME,
                PASSWORD
            )),
        };
    }
}

/// 获取环境变量的常用导入
pub mod prelude {
    pub use super::local::login_var;
}
