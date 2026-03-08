//! 本例展示了构建空闲教室查询的 POST 请求体以及
//! 对返回的 HTML 文档的解析操作。

use neuq_jwxt_rs::core::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    subscriber_init();
    
    // 登录成功后...
    
    // 构建一个查询当日第 1 节至第 12 节的空闲教室的请求体
    let query = ClassroomQuery::with_local_date(1, 12)?;

    // 使用请求体发送 POST 请求并获取返回的 HTML
    let html = client.get_classroom_html(query).await?;
    info!("Fetched free classroom HTML successfully.");

    // 解析 HTML 从而得到 `ClassroomList` 实例
    let data = parse_classroom_html(html)?;
    info!("Parsed data from classroom HTML successfully.");

    Ok(())
}