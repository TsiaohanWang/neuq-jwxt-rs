//! 核心数据结构的文件 I/O 操作

use super::data::ClassroomList;
use super::log::prelude::*;
use serde::Serialize;
use std::fs::File;
use std::io::{BufReader, BufWriter};

/// 针对 JSON 文件的 I/O
pub mod json {
    use super::*;

    /// 将 `ClassroomList` 保存至 JSON
    pub fn save_classroom_list_json<T: Serialize>(content: &T, path: &str) -> anyhow::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, content)?;

        info!("Saved ClassroomList to {:?}.", path);
        Ok(())
    }

    /// 读取 JSON 并反序列化为 `ClassroomList`
    pub fn load_classroom_list_json(path: &str) -> anyhow::Result<ClassroomList> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let list: ClassroomList = serde_json::from_reader(&mut reader)?;

        info!("Loaded ClassroomList from {:?}.", path);
        Ok(list)
    }
}

/// 针对 YAML 文件的 I/O
pub mod yaml {
    use super::*;

    /// 将 `ClassroomList` 保存至 YAML
    pub fn save_classroom_list_yaml<T: Serialize>(content: &T, path: &str) -> anyhow::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        serde_saphyr::to_io_writer(&mut writer, content)?;

        info!("Saved ClassroomList to {:?}.", path);
        Ok(())
    }

    /// 读取 YAML 并反序列化为 `ClassroomList`
    pub fn load_classroom_list_yaml(path: &str) -> anyhow::Result<ClassroomList> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let list: ClassroomList = serde_saphyr::from_reader(&mut reader)?;

        info!("Loaded ClassroomList from {:?}.", path);
        Ok(list)
    }
}

/// 文件 I/O 的常用导入
pub mod prelude {
    pub use super::json::{load_classroom_list_json, save_classroom_list_json};
    pub use super::yaml::{load_classroom_list_yaml, save_classroom_list_yaml};
}
