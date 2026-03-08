//! 本例展示了从本地 `.env` 文件获取学号密码并
//! 登录教务系统的操作。

use neuq_jwxt_rs::core::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    subscriber_init();

    // 从本地 `.env` 文件获取环境变量
    let (usn, pwd) = login_var()?;

    // 创建客户端并使用环境变量登录
    let client = NeuqClient::new()?;
    let res = client.login(usn.as_str(), pwd.as_str()).await?;

    match res {
        true => info!("Login successful."),
        false => {
            error!("Login failed.");
            return Err(anyhow::anyhow!("Login failed."));
        }
    }

    Ok(())
}