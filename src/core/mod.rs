//! 库的核心内容，包含与[教务系统](https://jwxt.neuq.edu.cn/eams)交互的 HTTP 客户端和相关数据操作。

pub mod client;
pub mod data;
pub mod env;
pub mod file;
pub mod filter;
pub mod log;
pub mod parser;
pub mod query;

/// 核心模块的常用导入
pub mod prelude {
    pub use super::client::prelude::*;
    pub use super::data::prelude::*;
    pub use super::env::prelude::*;
    pub use super::file::prelude::*;
    pub use super::filter::prelude::*;
    pub use super::log::prelude::*;
    pub use super::parser::prelude::*;
    pub use super::query::prelude::*;
}
