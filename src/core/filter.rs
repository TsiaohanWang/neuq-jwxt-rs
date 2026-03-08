//! 对核心数据结构各字段的过滤器

use super::data::{Building, Campus, Classroom, TypeName};
use anyhow::Ok;
use regex::Regex;
use std::collections::HashSet;
use std::hash::Hash;

/// 数据过滤器所实现的特征
pub trait Filter {
    /// 将要筛选的数据类型
    type Item;

    /// 检验数据实例是否符合过滤条件
    ///
    /// 由于实例通常由多个数据项构成，因此返回 `Vec<bool>` 来保留每一项的匹配情况
    fn matches(&self, item: &Self::Item) -> Vec<bool>;
}

/// 用于 `ClassroomFilter` 过滤 `building`、`campus`、`type_name` 字段的匹配值集合类型。
pub type FilterPattern<T> = HashSet<Option<T>>;

/// 使用动态数组 `Vec<T>` 快速创建一个 `FilterPattern<T>`。
pub fn new_pattern<T: Eq + Hash>(pattern_vec: Vec<T>) -> FilterPattern<T> {
    let mut set = HashSet::new();
    for p in pattern_vec {
        set.insert(Some(p));
    }

    set
}

/// 向 `FilterPattern<T>` 实例中添加一个 `None`。
pub fn add_none_to_pattern<T: Eq + Hash>(mut filter_pattern: FilterPattern<T>) -> FilterPattern<T> {
    filter_pattern.insert(None);

    filter_pattern
}

#[derive(Debug)]
pub struct ClassroomFilter<F: FnMut(&mut Classroom)> {
    /// 对 `Classroom` 的 `name` 字段进行正则匹配的正则表达式
    name_pattern: Option<Regex>,

    /// 对 `Classroom` 的 `building` 字段进行匹配的模式
    ///
    /// 若 `HashSet<Option<Building>>` (`FilterPattern<Building>`) 包含被筛选 `Classroom` 的 `building` 字段，则匹配成功
    building_pattern: Option<FilterPattern<Building>>,

    /// 对 `Classroom` 的 `campus` 字段进行匹配的模式
    ///
    /// 若 `HashSet<Option<Campus>>` (`FilterPattern<Campus>`) 包含被筛选 `Classroom` 的 `campus` 字段，则匹配成功
    campus_pattern: Option<FilterPattern<Campus>>,

    /// 对 `Classroom` 的 `type_name` 字段进行匹配的模式
    ///
    /// 若 `HashSet<Option<TypeName>>` (`FilterPattern<TypeName>`) 包含被筛选 `Classroom` 的 `type_name` 字段，则匹配成功
    type_name_pattern: Option<FilterPattern<TypeName>>,

    /// 对 `Classroom` 的 `capacity` 字段进行匹配的模式
    ///
    /// `HashSet<[u16; 2]>` 的每个元素代表一个值区间（左闭右开），若 `capacity` 字段值符合任一区间则匹配成功
    capacity_pattern: Option<HashSet<[u16; 2]>>,

    /// 对符合过滤条件的 `Classroom` 所执行的操作
    pub exec: Option<F>,
}

impl<F: FnMut(&mut Classroom)> ClassroomFilter<F> {
    /// 新建一个空过滤器
    pub fn new() -> Self {
        Self {
            name_pattern: None,
            building_pattern: None,
            campus_pattern: None,
            type_name_pattern: None,
            capacity_pattern: None,
            exec: None,
        }
    }

    /// 设定过滤器的对 `Classroom` 的 `name` 字段进行正则匹配的正则表达式
    pub fn set_name(&mut self, regex: &str) -> anyhow::Result<&mut Self> {
        if !regex.is_empty() {
            self.name_pattern = Some(Regex::new(regex)?);
        };

        Ok(self)
    }

    /// 设定过滤器的对 `Classroom` 的 `building` 字段进行匹配的模式
    pub fn set_building(&mut self, set: HashSet<Option<Building>>) -> &mut Self {
        if !set.is_empty() {
            self.building_pattern = Some(set);
        };

        self
    }

    /// 设定过滤器的对 `Classroom` 的 `campus` 字段进行匹配的模式
    pub fn set_campus(&mut self, set: HashSet<Option<Campus>>) -> &mut Self {
        if !set.is_empty() {
            self.campus_pattern = Some(set);
        };

        self
    }

    /// 设定过滤器的对 `Classroom` 的 `type_name` 字段进行匹配的模式
    pub fn set_type_name(&mut self, set: HashSet<Option<TypeName>>) -> &mut Self {
        if !set.is_empty() {
            self.type_name_pattern = Some(set);
        }

        self
    }

    /// 设定过滤器的对 `Classroom` 的 `capacity` 字段进行匹配的模式
    pub fn set_capacity(&mut self, set: HashSet<[u16; 2]>) -> &mut Self {
        if !set.is_empty() {
            self.capacity_pattern = Some(set);
        }

        self
    }

    /// 设定过滤器的对符合过滤条件的 `Classroom` 所进行的操作
    pub fn set_exec(&mut self, exec: Option<F>) -> &mut Self {
        self.exec = exec;
        self
    }
}

impl<F: FnMut(&mut Classroom)> Filter for ClassroomFilter<F> {
    type Item = Classroom;
    fn matches(&self, record: &Self::Item) -> Vec<bool> {
        let mut res = vec![false; 5];

        if self.name_pattern.is_some() && record.name.is_some() {
            let re = self.name_pattern.as_ref().unwrap();

            match re.is_match(record.name.as_ref().unwrap().as_str()) {
                true => res[0] = true,
                false => (),
            }
        };

        if self.building_pattern.is_some() {
            let set = self.building_pattern.clone().unwrap();
            match set.contains(&record.building) {
                true => res[1] = true,
                false => (),
            }
        };

        if self.campus_pattern.is_some() {
            let set = self.campus_pattern.clone().unwrap();
            match set.contains(&record.campus) {
                true => res[2] = true,
                false => (),
            }
        };

        if self.type_name_pattern.is_some() {
            let set = self.type_name_pattern.clone().unwrap();
            match set.contains(&record.type_name) {
                true => res[3] = true,
                false => (),
            }
        };

        if self.capacity_pattern.is_some() {
            if record.capacity.is_none() {
                return res;
            }
            let c = record.capacity.unwrap();
            let set = self.capacity_pattern.clone().unwrap();

            for [lower, upper] in set {
                if lower <= c && c < upper {
                    res[4] = true;
                }
            }
        };

        res
    }
}

/// 过滤器的常用导入
pub mod prelude {
    pub use super::{ClassroomFilter, add_none_to_pattern, new_pattern};
}
