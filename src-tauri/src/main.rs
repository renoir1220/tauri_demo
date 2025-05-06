// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 导入 calamine 相关类型
use calamine::{open_workbook, Data, DataType, Reader, Xlsx}; // 如果需要xls, 也 use calamine::Xls;

use std::collections::HashMap; // 用于构建行数据

// 定义返回给前端的行数据结构 (最佳实践)
// 这个结构应该与前端的 TableRowData 类型匹配
#[derive(serde::Serialize, Debug)] // 添加 Serialize 和 Debug
struct TableRowData {
    // 定义你的列名和类型，例如：
    // id: Option<String>,
    // diagnosis: Option<String>,
    // ihc: Option<String>,
    // ... 其他列
    // 或者使用 HashMap<String, String> / HashMap<String, serde_json::Value> 获得更大灵活性
    // 这里使用 HashMap<String, String> 作为示例
    // 注意：实际应用中所有值都转为 String 可能丢失类型信息，需要权衡
    data: HashMap<String, String>,
}


// 定义 Tauri 命令，修改返回类型签名
#[tauri::command]
fn import_excel_data(file_path: String) -> Result<Vec<HashMap<String, String>>, String> { // <--- 修改返回类型
    println!("Rust: Received file path: {}", file_path);

    let mut workbook: Xlsx<_> = open_workbook(file_path.clone())
        .map_err(|e| format!("无法打开工作簿 '{}': {}", file_path, e))?;

    // ... (获取 sheet name 的部分可以保留或移除，如果只用索引) ...

    if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
        println!("Rust: Reading sheet with dimensions: {:?}", range.get_size());

        let mut rows_iter = range.rows();
        // ---> 修改 parsed_data 的类型 <---
        let mut parsed_data: Vec<HashMap<String, String>> = Vec::new();

        let headers: Vec<String> = if let Some(header_row) = rows_iter.next() {
            header_row.iter()
                .map(|cell| cell.get_string().unwrap_or_default().to_string())
                .collect()
        } else {
            return Err("无法读取表头行".to_string());
        };

        // 遍历数据行
        for row in rows_iter {
            let mut row_data_map = HashMap::new(); // <-- 这是我们要直接返回的类型
            for (i, cell) in row.iter().enumerate() {
                if let Some(header_name) = headers.get(i) {
                    let cell_value_str = match cell { // 使用正确的 Data 枚举
                        Data::Empty => "".to_string(),
                        Data::String(s) => s.to_string(),
                        Data::Float(f) => f.to_string(),
                        Data::Int(i) => i.to_string(),
                        Data::Bool(b) => b.to_string(),
                        Data::DateTime(dt) => dt.to_string(),
                        Data::DateTimeIso(dt_iso) => dt_iso.to_string(),
                        Data::DurationIso(dur_iso) => dur_iso.to_string(),
                        Data::Error(e) => format!("Error: {:?}", e),
                    };
                    row_data_map.insert(header_name.clone(), cell_value_str);
                }
            }
            // ---> 直接 push HashMap <---
            parsed_data.push(row_data_map);
        }

        println!("Rust: Parsed {} data rows.", parsed_data.len());
        Ok(parsed_data) // <-- 返回 Vec<HashMap<String, String>>

    } else {
        Err(format!("无法读取 '{}' 的第一个工作表", file_path))
    }
}

#[tauri::command]
async fn execute_query(query: String) -> Result<Vec<TableRowData>, String> {
     // TODO: 实现数据库查询逻辑
     println!("Rust: Executing query: {}", query);
     // 模拟返回数据
     let mut row1 = HashMap::new();
     row1.insert("id".to_string(), "DB001".to_string());
     row1.insert("diagnosis".to_string(), "数据库查询结果1".to_string());
     let mut row2 = HashMap::new();
     row2.insert("id".to_string(), "DB002".to_string());
     row2.insert("diagnosis".to_string(), "数据库查询结果2".to_string());
     Ok(vec![TableRowData { data: row1 }, TableRowData { data: row2 }])
}


fn main() {
    tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
         // 注册你的 Rust 命令
        .invoke_handler(tauri::generate_handler![
            import_excel_data,
            execute_query // 也注册 execute_query
            // ... 其他命令
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
