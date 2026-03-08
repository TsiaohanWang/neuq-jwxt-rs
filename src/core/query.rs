//! 包含了客户端发送 POST 请求时所需请求体的 trait，以及几个常用的实现该 trait 的类。

use serde::Serialize;

/// 一个用于客户端 POST 请求的请求体 trait。
///
/// - 仅是一个标记 trait，用于标记该类型是适于客户端发送 POST 请求的；
/// - 继承自 `Serialize` trait，此为 `reqwest::RequestBuilder::form` 的 trait 限制；
/// - 继承自 `Into<Vec<(String, String)>>` trait，此 trait 用于在 POST 请求前进行序列化。
pub trait Query: Serialize + Into<Vec<(String, String)>> {}

/// 用于空闲教室查询页的 POST 请求体。
///
/// - 其中公共字段为空闲教室查询页 POST 请求常用的参数。
#[derive(Debug, Serialize)]
pub struct ClassroomQuery {
    #[serde(rename = "cycleTime.dateBegin")]
    pub cycle_time_date_begin: String,
    #[serde(rename = "cycleTime.dateEnd")]
    pub cycle_time_date_end: String,
    #[serde(rename = "timeBegin")]
    pub time_begin: String,
    #[serde(rename = "timeEnd")]
    pub time_end: String,

    #[serde(rename = "classroom.building.id")]
    classroom_building_id: String,
    #[serde(rename = "pageSize")]
    page_size: String,
    #[serde(rename = "classroom.type.id")]
    classroom_type_id: String,
    #[serde(rename = "classroom.campus.id")]
    classroom_campus_id: String,
    #[serde(rename = "seats")]
    seats: String,
    #[serde(rename = "classroom.name")]
    classroom_name: String,
    #[serde(rename = "cycleTime.cycleCount")]
    cycle_time_cycle_count: String,
    #[serde(rename = "cycleTime.cycleType")]
    cycle_time_cycle_type: String,
    #[serde(rename = "roomApplyTimeType")]
    room_apply_time_type: String,
}

impl ClassroomQuery {
    /// 使用字符串与整数构建 Query 实例
    ///
    /// - `date_begin` 表示查询教室使用起始日期，格式为 `YYYY-MM-DD`；
    /// - `date_end` 表示查询教室使用结束日期，格式为 `YYYY-MM-DD`；
    /// - `time_begin` 表示查询教室使用起始小节，可选值为 `1` 至 `12`；
    /// - `time_end` 表示查询教室使用结束小节，可选值为 `1` 至 `12`。
    pub fn new(date_begin: &str, date_end: &str, time_begin: u8, time_end: u8) -> Self {
        assert!(time_begin <= time_end);

        Self {
            cycle_time_date_begin: date_begin.to_string(),
            cycle_time_date_end: date_end.to_string(),
            time_begin: time_begin.to_string(),
            time_end: time_end.to_string(),
            classroom_building_id: "".to_string(),
            page_size: "1000".to_string(),
            classroom_type_id: "".to_string(),
            classroom_campus_id: "".to_string(),
            seats: "".to_string(),
            classroom_name: "".to_string(),
            cycle_time_cycle_count: "1".to_string(),
            cycle_time_cycle_type: "1".to_string(),
            room_apply_time_type: "0".to_string(),
        }
    }

    /// 使用当前本地日期构建 Query 实例
    ///
    /// 其中 `date_begin` 与 `date_end` 均为当前本地日期。
    ///
    /// - `time_begin` 表示查询教室使用起始小节，可选值为 `1` 至 `12`；
    /// - `time_end` 表示查询教室使用结束小节，可选值为 `1` 至 `12`。
    pub fn with_local_date(time_begin: u8, time_end: u8) -> anyhow::Result<Self> {
        let local_date = time::OffsetDateTime::now_local()?.date().to_string();
        let date_begin = local_date.as_str();
        let date_end = local_date.as_str();

        Ok(Self::new(date_begin, date_end, time_begin, time_end))
    }
}

impl Into<Vec<(String, String)>> for ClassroomQuery {
    fn into(self) -> Vec<(String, String)> {
        vec![
            (
                "classroom.building.id".to_string(),
                self.classroom_building_id,
            ),
            (
                "cycleTime.dateBegin".to_string(),
                self.cycle_time_date_begin,
            ),
            ("cycleTime.dateEnd".to_string(), self.cycle_time_date_end),
            ("timeBegin".to_string(), self.time_begin),
            ("timeEnd".to_string(), self.time_end),
            ("pageSize".to_string(), self.page_size),
            ("classroom.type.id".to_string(), self.classroom_type_id),
            ("classroom.campus.id".to_string(), self.classroom_campus_id),
            ("seats".to_string(), self.seats),
            ("classroom.name".to_string(), self.classroom_name),
            (
                "cycleTime.cycleCount".to_string(),
                self.cycle_time_cycle_count,
            ),
            (
                "cycleTime.cycleType".to_string(),
                self.cycle_time_cycle_type,
            ),
            ("roomApplyTimeType".to_string(), self.room_apply_time_type),
        ]
    }
}

impl Query for ClassroomQuery {}

pub mod prelude {
    pub use super::ClassroomQuery;
}
