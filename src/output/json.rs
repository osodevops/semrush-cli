pub fn render(data: &[serde_json::Value], meta: &serde_json::Value) -> String {
    let output = serde_json::json!({
        "_meta": meta,
        "data": data,
    });
    serde_json::to_string_pretty(&output).unwrap_or_else(|_| "{}".to_string())
}
