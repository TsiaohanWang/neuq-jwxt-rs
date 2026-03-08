use super::data::{Classroom, ClassroomList};
use super::log::prelude::*;
use scraper::{Html, Selector};

/// 解析每一个 HTML `tr` 元素的 `td` 时，依次传入 `Classroom` 的映射顺序。
///
/// 第 `TABLE_MAPPING[i]` 个 `td` 元素内容将作为 `Classroom::from` 的第 `i` 个参数传入。
const TABLE_MAPPING: [usize; 5] = [1, 2, 3, 4, 5];

pub fn parse_classroom_html(html_str: String) -> anyhow::Result<ClassroomList> {
    let document = Html::parse_document(html_str.as_str());
    let mut results = Vec::new();

    let table_selector = Selector::parse("table.gridtable").unwrap();
    let thead_th_selector = Selector::parse("thead th").unwrap();
    let tbody_tr_selector = Selector::parse("tbody tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    let table = match document.select(&table_selector).next() {
        Some(t) => t,
        None => {
            error!("Failed to parse table element from html.");
            return Err(anyhow::anyhow!("No table selected in html document."));
        }
    };

    let mut headers = Vec::new();
    for th in table.select(&thead_th_selector) {
        let text = th.text().collect::<String>().trim().to_string();
        headers.push(text);
    }

    for tr in table.select(&tbody_tr_selector) {
        let mut row_data = Vec::new();

        for td in tr.select(&td_selector) {
            let cell_text = td.text().collect::<String>().trim().to_string();
            row_data.push(cell_text);
        }

        let classroom = Classroom::from(
            row_data[TABLE_MAPPING[0]].to_string(),
            row_data[TABLE_MAPPING[1]].to_string(),
            row_data[TABLE_MAPPING[2]].to_string(),
            row_data[TABLE_MAPPING[3]].to_string(),
            row_data[TABLE_MAPPING[4]].to_string(),
        );

        if !row_data.is_empty() {
            results.push(classroom);
        }
    }

    Ok(ClassroomList::new_with(results))
}

pub mod prelude {
    pub use super::parse_classroom_html;
}
