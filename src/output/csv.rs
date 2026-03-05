pub fn render(data: &[serde_json::Value]) -> String {
    if data.is_empty() {
        return String::new();
    }

    let headers: Vec<String> = if let Some(serde_json::Value::Object(map)) = data.first() {
        map.keys().cloned().collect()
    } else {
        return String::new();
    };

    let mut lines = Vec::with_capacity(data.len() + 1);
    lines.push(headers.join(","));

    for row in data {
        if let Some(map) = row.as_object() {
            let vals: Vec<String> = headers
                .iter()
                .map(|h| match map.get(h) {
                    None | Some(serde_json::Value::Null) => String::new(),
                    Some(serde_json::Value::String(s)) => {
                        if s.contains(',') || s.contains('"') {
                            format!("\"{}\"", s.replace('"', "\"\""))
                        } else {
                            s.clone()
                        }
                    }
                    Some(v) => v.to_string(),
                })
                .collect();
            lines.push(vals.join(","));
        }
    }

    lines.join("\n")
}
