# 东秦教务系统 Rust 操作库

> [!WARNING]
> 本项目仍在开发中，可能会出现多次破坏性更新！请谨慎留意你所使用的版本！

`neuq-jwxt-rs` 是一个提供能与[东北大学秦皇岛分校教务系统](https://jwxt.neuq.edu.cn/eams)交互的 HTTP 客户端和数据处理的库。

## 功能

目前提供：

- 本地 `.env` 文件的读取；
- 教务系统的个人登录；
- 对教务系统“空闲教室查询”功能的：
  - 数据解析与过滤，以及
  - 支持文件读写的数据结构。

> [!NOTE]
> 未来将会实现：
> 
> - [ ] 对 Cloudflare Workers 运行环境的支持；
> - [ ] 对教务系统“我的课表”功能的：
>   - 数据解析与过滤，以及
>   - 支持文件读写的数据结构。

## 快速开始

当你在本地运行时，请创建 `.env` 文件并执行：

```bash
echo NEUQ_USERNAME=[你的教务系统学号] >> .env
echo NEUQ_PASSWORD=[你的教务系统密码] >> .env
```

然后在异步 `main` 函数中使用 `env_var` 即可进行读取。

接着使用 `NeuqClient` 提供的 `new` 和 `login` 方法即可完成登录。

### 示例

以下示例演示了如何读取本地 `.env` 文件中的环境变量，并使用新建 HTTP 客户端登录教务系统。

```rust
let (usn, pwd) = login_var()?;

let client = NeuqClient::new()?;
let res = client.login(usn.as_str(), pwd.as_str()).await?;

match res {
    true => info!("Login successful."),
    false => {
        error!("Login failed.");
        return Err(anyhow::anyhow!("Login failed."));
    }
}
```

可在 [`./src/examples`](./src/examples) 中找到更多应用示例。

## 版本支持

本项目最低支持的 Rust 版本（MSRV）为 `1.93.1`。