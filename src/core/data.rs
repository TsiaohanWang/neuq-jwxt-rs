//! 核心数据结构的实现

use super::filter::{ClassroomFilter, Filter};
use super::log::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// 空闲教室查询得到的教室“教学楼”字段
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Building {
    /// 工学馆
    #[serde(rename = "工学馆")]
    GongXueGuan,
    /// 基础楼
    #[serde(rename = "基础楼")]
    JiChuLou,
    /// 综合实验楼
    #[serde(rename = "综合实验楼")]
    ZongHeShiYanLou,
    /// 地质楼
    #[serde(rename = "地质楼")]
    DiZhiLou,
    /// 管理楼
    #[serde(rename = "管理楼")]
    GuanLiLou,
    /// 大学会馆
    #[serde(rename = "大学会馆")]
    DaXueHuiGuan,
    /// 旧实验楼
    #[serde(rename = "旧实验楼")]
    JiuShiYanLou,
    /// 人文楼
    #[serde(rename = "人文楼")]
    RenWenLou,
    /// 科技楼
    #[serde(rename = "科技楼")]
    KeJiLou,

    #[serde(untagged)]
    Undefined(String),
}

impl Building {
    pub fn from_str(s: String) -> Building {
        match s.as_str() {
            "工学馆" => Building::GongXueGuan,
            "基础楼" => Building::JiChuLou,
            "综合实验楼" => Building::ZongHeShiYanLou,
            "地质楼" => Building::DiZhiLou,
            "管理楼" => Building::GuanLiLou,
            "大学会馆" => Building::DaXueHuiGuan,
            "旧实验楼" => Building::JiuShiYanLou,
            "人文楼" => Building::RenWenLou,
            "科技楼" => Building::KeJiLou,

            _ => {
                warn!("Undefined building: {:?}.", s);
                Building::Undefined(s.to_string())
            }
        }
    }
}

/// 空闲教室查询得到的教室“校区”字段
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Campus {
    /// 学校本部
    #[serde(rename = "学校本部")]
    XueXiaoBenBu,
    /// 北戴河校区
    #[serde(rename = "北戴河校区")]
    BeiDaiHeXiaoQu,
    /// 新校区
    #[serde(rename = "新校区")]
    XinXiaoQu,

    #[serde(untagged)]
    Undefined(String),
}

impl Campus {
    pub fn from_str(s: String) -> Campus {
        match s.as_str() {
            "学校本部" => Campus::XueXiaoBenBu,
            "北戴河校区" => Campus::BeiDaiHeXiaoQu,
            "新校区" => Campus::XinXiaoQu,

            _ => {
                warn!("Undefined campus: {:?}.", s);
                Campus::Undefined(s.to_string())
            }
        }
    }
}

/// 空闲教室查询得到的教室“教室类型（教室设备配置）”字段
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeName {
    /// 普通教室
    #[serde(rename = "普通教室")]
    PuTongJiaoShi,
    /// 多媒体大教室
    #[serde(rename = "多媒体大教室")]
    DuoMeiTiDaJiaoShi,
    /// 多媒体小教室
    #[serde(rename = "多媒体小教室")]
    DuoMeiTiXiaoJiaoShi,
    /// 语音室
    #[serde(rename = "语音室")]
    YuYinShi,
    /// 不排课教室
    #[serde(rename = "不排课教室")]
    BuPaiKeJiaoShi,
    /// 录播教室
    #[serde(rename = "录播教室")]
    LuBoJiaoShi,
    /// 机房
    #[serde(rename = "机房")]
    JiFang,
    /// 活动教室
    #[serde(rename = "活动教室")]
    HuoDongJiaoShi,
    /// 体育教学场地
    #[serde(rename = "体育教学场地")]
    TiYuJiaoXueChangDi,
    /// 智慧教室
    #[serde(rename = "智慧教室")]
    ZhiHuiJiaoShi,
    /// 实验室
    #[serde(rename = "实验室")]
    ShiYanShi,
    /// 研讨室
    #[serde(rename = "研讨室")]
    YanTaoShi,
    /// 多功能
    #[serde(rename = "多功能")]
    DuoGongNeng,

    #[serde(untagged)]
    Undefined(String),
}

impl TypeName {
    pub fn from_str(s: String) -> TypeName {
        match s.as_str() {
            "普通教室" => TypeName::PuTongJiaoShi,
            "多媒体大教室" => TypeName::DuoMeiTiDaJiaoShi,
            "多媒体小教室" => TypeName::DuoMeiTiXiaoJiaoShi,
            "语音室" => TypeName::YuYinShi,
            "不排课教室" => TypeName::BuPaiKeJiaoShi,
            "录播教室" => TypeName::LuBoJiaoShi,
            "机房" => TypeName::JiFang,
            "活动教室" => TypeName::HuoDongJiaoShi,
            "体育教学场地" => TypeName::TiYuJiaoXueChangDi,
            "智慧教室" => TypeName::ZhiHuiJiaoShi,
            "实验室" => TypeName::ShiYanShi,
            "研讨室" => TypeName::YanTaoShi,
            "多功能" => TypeName::DuoGongNeng,

            _ => {
                warn!("Undefined type name: {:?}.", s);
                TypeName::Undefined(s.to_string())
            }
        }
    }
}

/// 将空闲教室各字段的 String 转换为其专用数据结构的中间层
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Data(String);

impl Data {
    pub fn to_name(&self) -> Option<String> {
        match self.0.is_empty() {
            true => None,
            false => Some(self.0.clone()),
        }
    }

    pub fn to_building(&self) -> Option<Building> {
        match self.0.is_empty() {
            true => None,
            false => Some(Building::from_str(self.0.to_string())),
        }
    }

    pub fn to_campus(&self) -> Option<Campus> {
        match self.0.is_empty() {
            true => None,
            false => Some(Campus::from_str(self.0.to_string())),
        }
    }

    pub fn to_type_name(&self) -> Option<TypeName> {
        match self.0.is_empty() {
            true => None,
            false => Some(TypeName::from_str(self.0.to_string())),
        }
    }

    pub fn to_capacity(&self) -> Option<u16> {
        match self.0.is_empty() {
            true => None,
            false => match self.0.parse::<u16>() {
                Ok(c) => Some(c),
                Err(_e) => {
                    warn!(
                        "Failed to parse u16 from String {:?} in Classroom::capacity.",
                        self.0
                    );
                    None
                }
            },
        }
    }
}

impl From<String> for Data {
    fn from(s: String) -> Self {
        Data(s)
    }
}

/// 用于表示空闲教室查询得到的教室的数据结构
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Classroom {
    /// 名称
    pub name: Option<String>,
    /// 教学楼
    pub building: Option<Building>,
    /// 校区
    pub campus: Option<Campus>,
    /// 教室类型（教室设备配置）
    pub type_name: Option<TypeName>,
    /// 容量(≥)
    pub capacity: Option<u16>,
}

impl Classroom {
    /// 新建一个无任何有效字段的教室
    /// 
    /// - 此时该实例会判定为无效。
    pub fn new() -> Self {
        Self {
            name: None,
            building: None,
            campus: None,
            type_name: None,
            capacity: None,
        }
    }

    /// 使用给定的 String 初始化一个教室
    /// 
    /// String 会通过中间层 `Data(String)` 安全地转换为教室的对应字段。
    pub fn from(
        name: String,
        building: String,
        campus: String,
        type_name: String,
        capacity: String,
    ) -> Self {
        Self {
            name: Data::from(name).to_name(),
            building: Data::from(building).to_building(),
            campus: Data::from(campus).to_campus(),
            type_name: Data::from(type_name).to_type_name(),
            capacity: Data::from(capacity).to_capacity(),
        }
    }

    /// 设置当前教室的 `name` 字段
    /// 
    /// - 若传入空字符串会自动转为 `None`。
    pub fn set_name(&mut self, name: Option<String>) {
        match name.as_ref() {
            Some(n) => {
                if n.is_empty() {
                    self.name = None;
                    return;
                }
            }
            None => (),
        }
        self.name = name;
    }

    /// 设置当前教室的 `building` 字段 
    pub fn set_building(&mut self, building: Option<Building>) {
        self.building = building;
    }

    /// 设置当前教室的 `campus` 字段 
    pub fn set_campus(&mut self, campus: Option<Campus>) {
        self.campus = campus;
    }

    /// 设置当前教室的 `type_name` 字段 
    pub fn set_type_name(&mut self, type_name: Option<TypeName>) {
        self.type_name = type_name;
    }

    /// 设置当前教室的 `capacity` 字段 
    pub fn set_capacity(&mut self, capacity: Option<u16>) {
        self.capacity = capacity;
    }

    /// 判断当前 `Classroom` 实例是否为无效数据
    ///
    /// - 若 `name` 字段为 `None`，则实例无效；
    /// - 若除 `name` 以外的所有字段均为 `None`，则实例无效。
    pub fn is_invalid(&self) -> bool {
        match self.name.is_none() {
            true => return true,
            false => (),
        };

        match self.building.is_none()
            && self.campus.is_none()
            && self.type_name.is_none()
            && self.capacity.is_none()
        {
            true => return true,
            false => (),
        };

        false
    }
}

/// 空闲教室的集合类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassroomList(Vec<Classroom>);

impl ClassroomList {
    /// 创建一个空闲教室的空集合
    pub fn new() -> Self {
        Self(vec![])
    }

    /// 使用给定的 `Classroom` 动态数组初始化集合
    pub fn new_with(vec: Vec<Classroom>) -> Self {
        Self(vec)
    }

    /// 获取集合中空闲教室数
    /// 
    /// - 此方法不会保证集合已去重，同时无效教室也会计算在内。
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// 判断空闲教室集合是否为空
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// 向集合中新增空闲教室
    /// 
    /// - 此方法不会对新增教室进行重复元素的检验。
    pub fn push(&mut self, classroom: Classroom) {
        self.0.push(classroom);
    }

    /// 移除索引处的教室
    pub fn remove(&mut self, idx: usize) {
        self.0.remove(idx);
    }

    /// 设定指定索引处教室的 `name` 字段
    pub fn set_name(&mut self, idx: usize, name: Option<String>) {
        self.0[idx].set_name(name);
    }

    /// 设定指定索引处教室的 `building` 字段
    pub fn set_building(&mut self, idx: usize, building: Option<Building>) {
        self.0[idx].set_building(building);
    }

    /// 设定指定索引处教室的 `campus` 字段
    pub fn set_campus(&mut self, idx: usize, campus: Option<Campus>) {
        self.0[idx].set_campus(campus);
    }

    /// 设定指定索引处教室的 `type_name` 字段
    pub fn set_type_name(&mut self, idx: usize, type_name: Option<TypeName>) {
        self.0[idx].set_type_name(type_name);
    }

    /// 设定指定索引处教室的 `capacity` 字段
    pub fn set_capacity(&mut self, idx: usize, capacity: Option<u16>) {
        self.0[idx].set_capacity(capacity);
    }

    /// 消耗当前实例，对其每个元素检查是否符合过滤条件，符合条件的执行过滤器操作，最终得到一个新实例。
    pub fn apply_filter<F: FnMut(&mut Classroom)>(self, mut filter: ClassroomFilter<F>) -> Self {
        if let Some(mut exec) = filter.exec.take() {
            let mut list = Self::new();
            for mut classroom in self {
                let res = (&filter).matches(&classroom);
                if res.contains(&true) {
                    exec(&mut classroom);
                }
                list.push(classroom);
            }
            list
        } else {
            warn!("Failed to apply filter because the filter is invalid.");
            self
        }
    }

    /// 消耗当前实例，清除其中所有无效的 `Classroom` 实例并得到一个新实例。
    pub fn flush_invalid(self) -> Self {
        let mut list = ClassroomList::new();
        for classroom in self {
            if !classroom.is_invalid() {
                list.push(classroom);
            }
        }

        list
    }

    /// 消耗当前实例，清除其中重复的 `Classroom` 实例并得到一个新实例。
    pub fn sort(self) -> Self {
        let sorted = self
            .into_iter()
            .collect::<HashSet<Classroom>>()
            .into_iter()
            .collect::<Vec<Classroom>>();

        Self(sorted)
    }
}

/// 空闲教室集合类型的迭代器实现
impl Iterator for ClassroomList {
    type Item = Classroom;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

/// 核心数据结构的常用导入
pub mod prelude {
    pub use super::{Building, Campus, Classroom, ClassroomList, TypeName};
}
