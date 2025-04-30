// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 导入 calamine 相关类型
use calamine::{open_workbook, DataType, Reader, Xlsx}; // 如果需要xls, 也 use calamine::Xls;
use calamine::{open_workbook, DataType, Reader, Xls}; // 如果需要xls, 也 use calamine::Xls;
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


// 定义 Tauri 命令
#[tauri::command]
fn import_excel_data(file_path: String) -> Result<Vec<TableRowData>, String> {
    println!("Rust: Received file path: {}", file_path);

    // 根据文件扩展名选择打开方式 (xlsx 或 xls)
    // 这里以 xlsx 为例，你需要根据 calamine 的文档和 feature 处理 .xls
    let mut workbook: Xlsx<_> = open_workbook(file_path.clone()) // 使用 clone 避免所有权问题
        .map_err(|e| format!("无法打开工作簿 '{}': {}", file_path, e))?;

    // 获取第一个工作表的名称列表（如果需要按名称查找）
    // let sheet_names = workbook.sheet_names().to_owned();
    // if sheet_names.is_empty() {
    //     return Err("工作簿中没有找到工作表".to_string());
    // }
    // let first_sheet_name = &sheet_names[0];

    // 或者直接按索引获取第一个工作表 (更常用)
    if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
        println!("Rust: Reading sheet with dimensions: {:?}", range.get_size());

        let mut rows_iter = range.rows();
        let mut parsed_data: Vec<TableRowData> = Vec::new();

        // 1. 读取表头行 (假设第一行是表头)
        let headers: Vec<String> = if let Some(header_row) = rows_iter.next() {
             header_row.iter()
                .map(|cell| cell.get_string().unwrap_or_default().to_string())
                .collect()
        } else {
            return Err("无法读取表头行".to_string());
        };

        // 2. 遍历数据行
        for row in rows_iter {
            let mut row_data_map = HashMap::new();
            for (i, cell) in row.iter().enumerate() {
                if let Some(header_name) = headers.get(i) {
                     // 将所有单元格类型转换为字符串 (简单处理方式)
                    let cell_value_str = match cell {
                        DataType::Empty => "".to_string(),
                        DataType::String(s) => s.to_string(),
                        DataType::Float(f) => f.to_string(),
                        DataType::Int(i) => i.to_string(),
                        DataType::Bool(b) => b.to_string(),
                        DataType::DateTime(dt) => dt.to_string(), // 可能需要格式化
                        DataType::DateTimeIso(dtiso) => dtiso.to_string(),
                        DataType::DurationIso(duriso) => duriso.to_string(),
                        DataType::Error(e) => format!("Error: {:?}", e),
                        // _ => "Unsupported Cell Type".to_string(), // 处理其他类型
                    };
                    row_data_map.insert(header_name.clone(), cell_value_str);
                }
            }
             parsed_data.push(TableRowData { data: row_data_map });
        }

        println!("Rust: Parsed {} data rows.", parsed_data.len());
        Ok(parsed_data)

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
         // 注册你的 Rust 命令
        .invoke_handler(tauri::generate_handler![
            import_excel_data,
            execute_query // 也注册 execute_query
            // ... 其他命令
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
