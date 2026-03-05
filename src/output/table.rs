use tabled::{settings::Style, Table};

pub fn render(data: &[serde_json::Value]) -> String {
    if data.is_empty() {
        return "No data found.".to_string();
    }

    // Extract column headers from first row
    let headers: Vec<String> = if let Some(serde_json::Value::Object(map)) = data.first() {
        map.keys().cloned().collect()
    } else {
        return "No data found.".to_string();
    };

    // Build rows as Vec<Vec<String>>
    let rows: Vec<Vec<String>> = data
        .iter()
        .filter_map(|row| {
            row.as_object()
                .map(|map| headers.iter().map(|h| format_cell(map.get(h))).collect())
        })
        .collect();

    let mut all_rows: Vec<Vec<String>> = Vec::with_capacity(rows.len() + 1);
    all_rows.push(headers);
    all_rows.extend(rows);

    let mut table = Table::from_iter(all_rows);
    table.with(Style::rounded());
    table.to_string()
}

fn format_cell(value: Option<&serde_json::Value>) -> String {
    match value {
        None | Some(serde_json::Value::Null) => "—".to_string(),
        Some(serde_json::Value::String(s)) => s.clone(),
        Some(serde_json::Value::Number(n)) => {
            if let Some(i) = n.as_i64() {
                format_number(i)
            } else if let Some(f) = n.as_f64() {
                format!("{:.2}", f)
            } else {
                n.to_string()
            }
        }
        Some(serde_json::Value::Bool(b)) => b.to_string(),
        Some(serde_json::Value::Array(arr)) => {
            let items: Vec<String> = arr.iter().map(|v| format_cell(Some(v))).collect();
            items.join(", ")
        }
        Some(v) => v.to_string(),
    }
}

fn format_number(n: i64) -> String {
    let s = n.abs().to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    if n < 0 {
        result.push('-');
    }
    result.chars().rev().collect()
}
