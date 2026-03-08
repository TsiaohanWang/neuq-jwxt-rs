//! 本例展示了将空闲教室数据进行过滤以及
//! 将之保存为文件的操作。

use neuq_jwxt_rs::core::prelude::*;
use std::collections::HashSet;

#[tokio::main]
async fn main() {
    subscriber_init();

    // 登录成功后...
    
    let query = ClassroomQuery::with_local_date(1, 12)?;

    let html = client.get_classroom_html(query).await?;

    let data = parse_classroom_html(html)?;

    // 将原 `ClassroomList` 保存到 `list.yaml`
    save_classroom_list_yaml(&data, "list.yaml")?;

    // 创建过滤器
    let mut filter = ClassroomFilter::new();

    // 创建过滤器操作
    // 
    // 这里将 `Classroom` 置为 `None`，使得该实例被判为无效
    let execution = |c: &mut Classroom| {
        c.set_name(None)
    };
    
    // 配置过滤器
    // 
    // 对 `Classroom` 的过滤条件：
    // - `campus` 字段为 `Campus::XueXiaoBenBu`或 `Campus::BeiDaiHeXiaoQu`，且
    // - `capacity` 字段值落在 $[0, 1)$ 或 $[1, 10)$，且
    // - `type_name` 字段值为 `None`。
    // 
    // 若满足过滤条件，该 `Classroom` 实例将被执行 `execution` 操作。
    filter
        .set_campus(new_pattern(vec![Campus::XueXiaoBenBu, Campus::BeiDaiHeXiaoQu]))
        .set_capacity(HashSet::from([[0u16, 1u16], [1u16, 10u16]]))
        .set_type_name(add_none_to_pattern(new_pattern(vec![])))
        .set_exec(Some(execution));

    // 应用过滤器，并清除无效 `Classroom`。
    // 
    // 此时所有原先符合过滤条件的实例均已不存在。
    let new_data = data.apply_filter(filter).flush_invalid();

    // 将过滤后的 `ClassroomList` 保存到 `filtered_list.yaml`
    save_classroom_list_yaml(&new_data, "filtered_list.yaml")?;

    Ok(())
}